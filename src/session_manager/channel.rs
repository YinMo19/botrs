use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::mpsc;
use tracing::{debug, error, info};

use super::{
    Session, SessionManager, calc_interval, can_not_identify, can_not_resume, check_session_limit,
};
use crate::error::BotError;
use crate::gateway::Gateway;
use crate::intents::Intents;
use crate::models::api::GatewayResponse;
use crate::models::gateway::GatewayEvent;
use crate::token::Token;

pub(super) type SessionFuture = Pin<Box<dyn Future<Output = (Session, crate::Result<()>)> + Send>>;
pub(super) type SessionConnectFn =
    dyn Fn(Session, mpsc::UnboundedSender<GatewayEvent>) -> SessionFuture + Send + Sync;

/// Local, channel-backed default session manager.
#[derive(Clone)]
pub struct ChanManager {
    session_sender: Option<mpsc::UnboundedSender<Session>>,
    connect_fn: Arc<SessionConnectFn>,
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

    fn with_connect_fn<F>(connect_fn: F) -> Self
    where
        F: Fn(Session, mpsc::UnboundedSender<GatewayEvent>) -> SessionFuture
            + Send
            + Sync
            + 'static,
    {
        Self {
            session_sender: None,
            connect_fn: Arc::new(connect_fn),
        }
    }

    pub fn sessions(ap_info: &GatewayResponse, token: Token, intents: Intents) -> Vec<Session> {
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

    pub(super) async fn new_connect(
        session_sender: mpsc::UnboundedSender<Session>,
        connect_fn: Arc<SessionConnectFn>,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
        session: Session,
    ) {
        let (mut reconnect_session, result) = connect_fn(session, event_sender).await;
        if let Err(err) = result {
            error!("[ws/session/local] Listening err {}", err);
            if can_not_resume(&err) {
                reconnect_session.id.clear();
                reconnect_session.last_seq = 0;
            }
            if can_not_identify(&err) {
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
        ap_info: &GatewayResponse,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()> {
        check_session_limit(ap_info)?;
        let start_interval = calc_interval(ap_info.session_start_limit.max_concurrency);
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
