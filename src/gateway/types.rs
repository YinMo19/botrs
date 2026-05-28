use crate::intents::Intents;
use crate::token_impl::Token;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub(super) type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;
pub(super) const SESSION_START_LIMIT_WINDOW_SECS: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GatewayAction {
    Continue,
    Reconnect,
}

/// WebSocket gateway client for the QQ Guild Bot API.
pub struct Gateway {
    /// Gateway URL
    pub(super) url: String,
    /// Bot token
    pub(super) token: Token,
    /// Intent flags
    pub(super) intents: Intents,
    /// Shard information [shard_id, shard_count]
    pub(super) shard: Option<[u32; 2]>,
    /// Session ID for resuming
    pub(super) session_id: Option<String>,
    /// Last sequence number received
    pub(super) last_seq: Arc<AtomicU64>,
    /// Heartbeat interval in milliseconds
    pub(super) heartbeat_interval: Option<u64>,
    /// Whether the connection is ready
    pub(super) is_ready: Arc<AtomicBool>,
    /// Whether we can reconnect
    pub(super) can_reconnect: Arc<AtomicBool>,
    /// Atomic heartbeat interval for sharing between tasks
    pub(super) heartbeat_interval_ms: Arc<AtomicU64>,
    /// Heartbeat task handle for cleanup
    pub(super) heartbeat_handle: Option<tokio::task::JoinHandle<()>>,
    /// Connection alive status
    pub(super) connection_alive: Arc<AtomicBool>,
    /// Connection start time for duration tracking
    pub(super) connection_start_time: Option<Instant>,
    /// Total heartbeats sent counter
    pub(super) heartbeat_count: Arc<AtomicU64>,
    /// Last heartbeat ACK time for monitoring
    pub(super) last_heartbeat_ack: Arc<AtomicU64>,
    /// Heartbeat sent time for ACK tracking
    pub(super) last_heartbeat_sent: Arc<AtomicU64>,
}

impl Gateway {
    /// Creates a gateway client for one websocket session or shard.
    pub fn new(
        url: impl Into<String>,
        token: Token,
        intents: Intents,
        shard: Option<[u32; 2]>,
    ) -> Self {
        let shard = shard.or(Some([0, 1]));
        Self {
            url: url.into(),
            token,
            intents,
            shard,
            session_id: None,
            heartbeat_interval: None,
            last_seq: Arc::new(AtomicU64::new(0)),
            is_ready: Arc::new(AtomicBool::new(false)),
            can_reconnect: Arc::new(AtomicBool::new(true)),
            heartbeat_interval_ms: Arc::new(AtomicU64::new(30000)),
            heartbeat_handle: None,
            connection_alive: Arc::new(AtomicBool::new(false)),
            connection_start_time: None,
            heartbeat_count: Arc::new(AtomicU64::new(0)),
            last_heartbeat_ack: Arc::new(AtomicU64::new(0)),
            last_heartbeat_sent: Arc::new(AtomicU64::new(0)),
        }
    }

    pub(crate) fn with_resume_state(
        mut self,
        session_id: impl Into<String>,
        last_seq: u64,
    ) -> Self {
        self.session_id = Some(session_id.into());
        self.last_seq.store(last_seq, Ordering::Relaxed);
        self
    }

    /// Calculates a session start interval from gateway limits.
    ///
    /// Uses `round(2 / max_concurrency)` and guards the interval to at
    /// least one second before starting the next websocket session.
    pub fn session_start_interval(max_concurrency: u32) -> Duration {
        let max_concurrency = u64::from(max_concurrency.max(1));
        let quotient = SESSION_START_LIMIT_WINDOW_SECS / max_concurrency;
        let remainder = SESSION_START_LIMIT_WINDOW_SECS % max_concurrency;

        let rounded = match remainder.saturating_mul(2).cmp(&max_concurrency) {
            std::cmp::Ordering::Less => quotient,
            std::cmp::Ordering::Greater => quotient + 1,
            std::cmp::Ordering::Equal if quotient.is_multiple_of(2) => quotient,
            std::cmp::Ordering::Equal => quotient + 1,
        };

        Duration::from_secs(rounded.max(1))
    }

    pub(super) const fn identify_intents(&self) -> u32 {
        if self.intents.bits() == 0 {
            Intents::GUILDS
        } else {
            self.intents.bits()
        }
    }
}
