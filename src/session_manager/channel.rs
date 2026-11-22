use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep_until};
use tracing::{error, info};

use super::{Session, SessionManager, can_not_identify, can_not_resume, check_session_limit};
use crate::error::BotError;
use crate::gateway::Gateway;
use crate::intents::Intents;
use crate::models::api::GatewayResponse;
use crate::models::gateway::GatewayEvent;
use crate::token_impl::Token;

pub(super) type SessionFuture = Pin<Box<dyn Future<Output = (Session, crate::Result<()>)> + Send>>;
pub(super) type SessionConnectFn =
    dyn Fn(Session, mpsc::UnboundedSender<GatewayEvent>) -> SessionFuture + Send + Sync;

/// Local, channel-backed default session manager.
#[derive(Clone)]
pub struct ChanManager {
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

    pub(super) fn with_connect_fn<F>(connect_fn: F) -> Self
    where
        F: Fn(Session, mpsc::UnboundedSender<GatewayEvent>) -> SessionFuture
            + Send
            + Sync
            + 'static,
    {
        Self {
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
        connect_fn: Arc<SessionConnectFn>,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
        session: Session,
    ) -> crate::Result<Session> {
        let (mut reconnect_session, result) = connect_fn(session, event_sender).await;
        if let Err(err) = result {
            error!("[ws/session/local] Listening err {}", err);
            if can_not_resume(&err) {
                reconnect_session.id.clear();
                reconnect_session.last_seq = 0;
            }
            if can_not_identify(&err) {
                return Err(err);
            }
        }
        Ok(reconnect_session)
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
        let start_interval =
            Gateway::session_start_interval(ap_info.session_start_limit.max_concurrency);
        info!(
            "[ws/session/local] will start {} sessions and per session start interval is {:?}",
            ap_info.shards, start_interval
        );

        let mut pending = VecDeque::from(Self::sessions(ap_info, token, intents));
        let mut workers = JoinSet::new();
        let mut next_start = Instant::now();
        while !pending.is_empty() || !workers.is_empty() {
            tokio::select! {
                Some(result) = workers.join_next(), if !workers.is_empty() => {
                    let session = result.map_err(|err| BotError::session(format!("shard worker stopped: {err}")))??;
                    pending.push_back(session);
                }
                _ = sleep_until(next_start), if !pending.is_empty() => {
                    let session = pending.pop_front().expect("pending shard");
                    workers.spawn(Self::new_connect(self.connect_fn.clone(), event_sender.clone(), session));
                    next_start = Instant::now() + start_interval;
                }
            }
        }
        Ok(())
    }
}
