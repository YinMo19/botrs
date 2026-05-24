//! Botgo-compatible remote session-manager facade.
//!
//! BotRS does not ship a Redis client in the core crate. The types in this
//! module keep botgo's public extension points available while delegating the
//! actual websocket scheduling to the local channel manager unless an external
//! integration builds on top of these hooks.

#![allow(non_snake_case, non_upper_case_globals)]

use std::sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use tokio::sync::mpsc;

use crate::error::Err;
use crate::intents::Intents;
use crate::models::api::WebsocketAP;
use crate::models::gateway::GatewayEvent;
use crate::session_manager::{ChanManager, SessionManager};
use crate::token::Token;

pub static ErrGotLockFailed: LazyLock<Err> = LazyLock::new(|| {
    Err::new(
        9999,
        "compete for init sessions failed, wait to consume session",
        None::<String>,
    )
});
pub static ErrSessionMarshalFailed: LazyLock<Err> =
    LazyLock::new(|| Err::new(9999, "session marshal failed", None::<String>));
pub static ErrProduceFailed: LazyLock<Err> =
    LazyLock::new(|| Err::new(9999, "produce session failed", None::<String>));
pub static ErrorNotOk: LazyLock<Err> =
    LazyLock::new(|| Err::new(9999, "redis write not ok", None::<String>));

pub type Option = Box<dyn Fn(&mut RedisManager) + Send + Sync>;

pub fn WithClusterKey(key: impl Into<String>) -> Option {
    let key = key.into();
    Box::new(move |manager| manager.cluster_key = key.clone())
}

#[derive(Clone)]
pub struct RedisManager {
    cluster_key: String,
    local: ChanManager,
}

impl Default for RedisManager {
    fn default() -> Self {
        Self::new([])
    }
}

impl RedisManager {
    pub fn new(opts: impl IntoIterator<Item = Option>) -> Self {
        let mut manager = Self {
            cluster_key: "defaultCluster".to_string(),
            local: ChanManager::new(),
        };
        for opt in opts {
            opt(&mut manager);
        }
        manager
    }

    #[allow(non_snake_case)]
    pub fn New(opts: impl IntoIterator<Item = Option>) -> Self {
        Self::new(opts)
    }

    pub fn cluster_key(&self) -> &str {
        &self.cluster_key
    }
}

#[async_trait::async_trait]
impl SessionManager for RedisManager {
    async fn start(
        &mut self,
        ap_info: &WebsocketAP,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()> {
        self.local
            .start(ap_info, token, intents, event_sender)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct Lock {
    key: String,
    value: String,
    locked: Arc<AtomicBool>,
}

impl Lock {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            locked: Arc::new(AtomicBool::new(false)),
        }
    }

    #[allow(non_snake_case)]
    pub fn New(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(key, value)
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    pub fn Lock(&self, expire: Duration) -> crate::Result<()> {
        if expire.is_zero() {
            return Err(ErrorNotOk.clone().into());
        }
        self.locked.store(true, Ordering::Relaxed);
        Ok(())
    }

    pub fn Renew(&self, expire: Duration) -> crate::Result<()> {
        if !self.is_locked() || expire.is_zero() {
            return Err(ErrorNotOk.clone().into());
        }
        Ok(())
    }

    pub fn Release(&self) -> crate::Result<()> {
        self.locked.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub fn StartRenew(&self, expire: Duration) -> crate::Result<()> {
        self.Renew(expire)
    }

    pub fn StopRenew(&self) {
        self.locked.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_options_set_cluster_key() {
        let manager = RedisManager::new([WithClusterKey("cluster-a")]);
        assert_eq!(manager.cluster_key(), "cluster-a");
    }

    #[test]
    fn lock_facade_tracks_state() {
        let lock = Lock::new("key", "value");
        assert!(!lock.is_locked());
        lock.Lock(Duration::from_secs(1)).unwrap();
        assert!(lock.is_locked());
        lock.Renew(Duration::from_secs(1)).unwrap();
        lock.Release().unwrap();
        assert!(!lock.is_locked());
    }
}
