use crate::intents::Intents;
use crate::token::Token;
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
    /// Fixed interval between reconnect attempts, derived from gateway session limits
    pub(super) reconnect_interval: Duration,
}

impl Gateway {
    /// Creates a new gateway client.
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket gateway URL
    /// * `token` - Authentication token
    /// * `intents` - Intent flags for events to receive
    /// * `shard` - Optional shard information
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{Token, Intents};
    /// use botrs::gateway::Gateway;
    ///
    /// let token = Token::new("app_id", "secret");
    /// let intents = Intents::default();
    /// let gateway = Gateway::new("wss://api.sgroup.qq.com/websocket", token, intents, None);
    /// ```
    pub fn new(
        url: impl Into<String>,
        token: Token,
        intents: Intents,
        shard: Option<[u32; 2]>,
    ) -> Self {
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
            reconnect_interval: Duration::from_secs(SESSION_START_LIMIT_WINDOW_SECS),
        }
    }

    /// Configures the fixed reconnect interval used after a connection exits.
    ///
    /// Official SDKs throttle new websocket starts with a session-manager interval
    /// rather than a long-lived exponential backoff. A zero duration is normalized
    /// to one second to avoid tight reconnect loops.
    pub fn with_reconnect_interval(mut self, reconnect_interval: Duration) -> Self {
        self.reconnect_interval = if reconnect_interval.is_zero() {
            Duration::from_secs(1)
        } else {
            reconnect_interval
        };
        self
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
}
