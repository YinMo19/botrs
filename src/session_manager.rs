//! Botgo-style websocket session management helpers.

#![allow(non_snake_case, non_upper_case_globals)]

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::{LazyLock, RwLock};
use std::time::Duration;

use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::error::{
    BotError, CodeConnCloseCantIdentify, CodeConnCloseCantResume, err_session_limit,
};
use crate::gateway::Gateway;
use crate::intents::Intents;
use crate::models::api::{GatewayResponse, WebsocketAP};
use crate::models::gateway::GatewayEvent;
use crate::token::Token;

pub static CanNotResumeErrSet: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CodeConnCloseCantResume]));

pub static CanNotIdentifyErrSet: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CodeConnCloseCantIdentify]));

pub type SessionFuture = Pin<Box<dyn Future<Output = (Session, crate::Result<()>)> + Send>>;
pub type SessionConnectFn =
    dyn Fn(Session, mpsc::UnboundedSender<GatewayEvent>) -> SessionFuture + Send + Sync;
pub type BoxedSessionManager = Box<dyn SessionManager>;
pub type SessionManagerFactory = dyn Fn() -> BoxedSessionManager + Send + Sync;

static DEFAULT_SESSION_MANAGER: LazyLock<RwLock<Box<SessionManagerFactory>>> =
    LazyLock::new(|| RwLock::new(Box::new(|| Box::new(ChanManager::new()))));

/// Botgo-compatible websocket session descriptor.
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub url: String,
    pub token: Token,
    pub intent: Intents,
    pub last_seq: u64,
    pub shards: crate::models::api::ShardConfig,
    pub app_id: Option<String>,
}

impl Session {
    pub fn new(
        url: impl Into<String>,
        token: Token,
        intent: Intents,
        shard_id: u32,
        shard_count: u32,
    ) -> Self {
        Self {
            id: String::new(),
            url: url.into(),
            token,
            intent,
            last_seq: 0,
            shards: crate::models::api::ShardConfig {
                shard_id,
                shard_count,
            },
            app_id: None,
        }
    }

    pub fn shard(&self) -> [u32; 2] {
        [self.shards.shard_id, self.shards.shard_count]
    }
}

/// Session manager interface aligned with botgo's SessionManager.
#[async_trait::async_trait]
pub trait SessionManager: Send + Sync {
    async fn start(
        &mut self,
        ap_info: &WebsocketAP,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()>;

    #[allow(non_snake_case)]
    async fn Start(
        &mut self,
        ap_info: &WebsocketAP,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()> {
        self.start(ap_info, token, intents, event_sender).await
    }
}

/// Local, channel-backed session manager matching botgo's default manager.
#[derive(Clone)]
pub struct ChanManager {
    session_sender: Option<mpsc::UnboundedSender<Session>>,
    connect_fn: std::sync::Arc<SessionConnectFn>,
}

impl Default for ChanManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChanManager {
    pub fn new() -> Self {
        Self::with_connect_fn(|session, event_sender| {
            Box::pin(async move {
                let mut next_session = session.clone();
                let shard = session.shard();
                let mut gateway =
                    Gateway::new(session.url, session.token, session.intent, Some(shard));
                if !session.id.is_empty() {
                    gateway = gateway.with_resume_state(session.id, session.last_seq);
                }
                let result = gateway.connect_once(event_sender).await;
                next_session.id = gateway.session_id().unwrap_or_default().to_string();
                next_session.last_seq = gateway.last_sequence();
                (next_session, result)
            })
        })
    }

    pub fn with_connect_fn<F>(connect_fn: F) -> Self
    where
        F: Fn(Session, mpsc::UnboundedSender<GatewayEvent>) -> SessionFuture
            + Send
            + Sync
            + 'static,
    {
        Self {
            session_sender: None,
            connect_fn: std::sync::Arc::new(connect_fn),
        }
    }

    pub fn sessions(ap_info: &WebsocketAP, token: Token, intents: Intents) -> Vec<Session> {
        (0..ap_info.shards)
            .map(|shard_id| {
                Session::new(
                    ap_info.url.clone(),
                    token.clone(),
                    intents,
                    shard_id,
                    ap_info.shards,
                )
            })
            .collect()
    }

    async fn new_connect(
        session_sender: mpsc::UnboundedSender<Session>,
        connect_fn: std::sync::Arc<SessionConnectFn>,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
        session: Session,
    ) {
        let (reconnect_session, result) = connect_fn(session, event_sender).await;
        if let Err(err) = result {
            error!("[ws/session/local] Listening err {}", err);
            if CanNotIdentify(&err) {
                error!("can not identify because server return {}", err);
                return;
            }
        }
        if let Err(err) = session_sender.send(reconnect_session) {
            debug!("[ws/session/local] session queue closed: {}", err);
        }
    }
}

#[async_trait::async_trait]
impl SessionManager for ChanManager {
    async fn start(
        &mut self,
        ap_info: &WebsocketAP,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()> {
        CheckSessionLimit(ap_info)?;
        let start_interval = CalcInterval(ap_info.session_start_limit.max_concurrency);
        info!(
            "[ws/session/local] will start {} sessions and per session start interval is {:?}",
            ap_info.shards, start_interval
        );

        let (tx, mut rx) = mpsc::unbounded_channel();
        self.session_sender = Some(tx.clone());

        for session in Self::sessions(ap_info, token, intents) {
            tx.send(session)
                .map_err(|err| BotError::session(format!("produce session failed: {err}")))?;
        }

        while let Some(session) = rx.recv().await {
            tokio::time::sleep(start_interval).await;
            tokio::spawn(Self::new_connect(
                tx.clone(),
                self.connect_fn.clone(),
                event_sender.clone(),
                session,
            ));
        }

        Ok(())
    }
}

pub fn new_session_manager() -> BoxedSessionManager {
    let factory = DEFAULT_SESSION_MANAGER
        .read()
        .expect("default session manager lock poisoned");
    factory()
}

#[allow(non_snake_case)]
pub fn NewSessionManager() -> BoxedSessionManager {
    new_session_manager()
}

pub fn set_session_manager_factory(
    factory: impl Fn() -> BoxedSessionManager + Send + Sync + 'static,
) {
    *DEFAULT_SESSION_MANAGER
        .write()
        .expect("default session manager lock poisoned") = Box::new(factory);
}

pub fn set_session_manager(manager: impl SessionManager + Clone + 'static) {
    set_session_manager_factory(move || Box::new(manager.clone()));
}

#[allow(non_snake_case)]
pub fn SetSessionManager(manager: impl SessionManager + Clone + 'static) {
    set_session_manager(manager);
}

pub fn calc_interval(max_concurrency: u32) -> Duration {
    Gateway::session_start_interval(max_concurrency)
}

#[allow(non_snake_case)]
pub fn CalcInterval(max_concurrency: u32) -> Duration {
    calc_interval(max_concurrency)
}

pub fn can_not_resume(err: &(dyn std::error::Error + 'static)) -> bool {
    CanNotResumeErrSet.contains(&crate::error::Error(err).Code())
}

#[allow(non_snake_case)]
pub fn CanNotResume(err: &(dyn std::error::Error + 'static)) -> bool {
    can_not_resume(err)
}

pub fn can_not_identify(err: &(dyn std::error::Error + 'static)) -> bool {
    CanNotIdentifyErrSet.contains(&crate::error::Error(err).Code())
}

#[allow(non_snake_case)]
pub fn CanNotIdentify(err: &(dyn std::error::Error + 'static)) -> bool {
    can_not_identify(err)
}

pub fn check_session_limit(ap_info: &GatewayResponse) -> crate::Result<()> {
    if ap_info.shards > ap_info.session_start_limit.remaining {
        Err(err_session_limit().into())
    } else {
        Ok(())
    }
}

#[allow(non_snake_case)]
pub fn CheckSessionLimit(ap_info: &GatewayResponse) -> crate::Result<()> {
    check_session_limit(ap_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{
        CodeConnCloseCantIdentify, CodeConnCloseCantResume, CodeNeedReConnect, New,
    };
    use crate::models::api::SessionStartLimit;

    fn ap_info(shards: u32, remaining: u32, max_concurrency: u32) -> GatewayResponse {
        GatewayResponse {
            url: "wss://example.com".to_string(),
            shards,
            session_start_limit: SessionStartLimit {
                total: 10,
                remaining,
                reset_after: 1000,
                max_concurrency,
            },
        }
    }

    #[test]
    fn calc_interval_matches_botgo() {
        assert_eq!(CalcInterval(0), Duration::from_secs(2));
        assert_eq!(CalcInterval(1), Duration::from_secs(2));
        assert_eq!(CalcInterval(2), Duration::from_secs(1));
        assert_eq!(CalcInterval(3), Duration::from_secs(1));
        assert_eq!(CalcInterval(100), Duration::from_secs(1));
    }

    #[test]
    fn check_session_limit_matches_botgo() {
        assert!(CheckSessionLimit(&ap_info(2, 2, 1)).is_ok());

        let err = CheckSessionLimit(&ap_info(3, 2, 1)).unwrap_err();
        assert!(CanNotIdentify(&err));
    }

    #[test]
    fn resume_and_identify_error_sets_match_botgo() {
        let resume = New(CodeConnCloseCantResume, "invalid session");
        let identify = New(CodeConnCloseCantIdentify, "bot banned");
        let reconnect = New(CodeNeedReConnect, "need reconnect");

        assert!(CanNotResume(&resume));
        assert!(!CanNotResume(&identify));
        assert!(CanNotIdentify(&identify));
        assert!(!CanNotIdentify(&resume));
        assert!(!CanNotIdentify(&reconnect));
    }

    #[test]
    fn sessions_are_generated_per_shard() {
        let token = Token::new("app_id", "secret");
        let sessions = ChanManager::sessions(&ap_info(3, 3, 1), token, Intents::default());

        let shards = sessions
            .into_iter()
            .map(|session| session.shard())
            .collect::<Vec<_>>();
        assert_eq!(shards, vec![[0, 3], [1, 3], [2, 3]]);
    }
}
