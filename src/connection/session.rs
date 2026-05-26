use super::ConnectionState;
use crate::api::BotApi;
use futures_util::stream::{SplitSink, SplitStream};
use serde_json::Value;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info};

// Type aliases to simplify complex types
type ConnectFn = Box<
    dyn Fn(
            Session,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(), crate::error::BotError>> + Send>,
        > + Send
        + Sync,
>;
type DispatchFn = Box<dyn Fn(&str, Value) + Send + Sync>;

/// Type alias for websocket sink
pub type WsSink =
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>;

/// Type alias for websocket stream
pub type WsStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// Session information for websocket connections
#[derive(Debug, Clone)]
pub struct Session {
    /// Session ID
    pub session_id: String,
    /// Shard information
    pub shard: (u32, u32),
    /// Gateway URL
    pub url: String,
    /// Whether this session needs reconnection
    pub needs_reconnect: bool,
}

impl Session {
    /// Create a new session
    pub fn new(session_id: String, shard: (u32, u32), url: String) -> Self {
        Self {
            session_id,
            shard,
            url,
            needs_reconnect: false,
        }
    }

    /// Mark this session as needing reconnection
    pub fn mark_for_reconnect(&mut self) {
        self.session_id = String::new();
        self.needs_reconnect = true;
    }
}

/// Connection session pool for managing multiple websocket sessions
#[allow(unused)]
pub struct ConnectionSession {
    /// Maximum concurrent connections
    max_async: usize,
    /// Connection function
    connect_fn: ConnectFn,
    /// Event dispatcher
    dispatch_fn: DispatchFn,
    /// Session list
    sessions: Vec<Session>,
    /// Connection state
    state: Arc<Mutex<ConnectionState>>,
}

impl ConnectionSession {
    /// Create a new connection session
    pub fn new<F, D>(max_async: usize, connect_fn: F, dispatch_fn: D, api: BotApi) -> Self
    where
        F: Fn(
                Session,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<(), crate::error::BotError>> + Send>,
            > + Send
            + Sync
            + 'static,
        D: Fn(&str, Value) + Send + Sync + 'static,
    {
        Self {
            max_async,
            connect_fn: Box::new(connect_fn),
            dispatch_fn: Box::new(dispatch_fn),
            sessions: Vec::new(),
            state: Arc::new(Mutex::new(ConnectionState::new(api))),
        }
    }

    /// Add a session to the connection pool
    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    /// Run multiple sessions with specified interval
    pub async fn multi_run(mut self, session_interval: u64) -> Result<(), crate::error::BotError> {
        if self.sessions.is_empty() {
            return Ok(());
        }

        let mut index = 0;
        let mut tasks = Vec::new();

        while !self.sessions.is_empty() {
            debug!("Session list loop running");
            let time_interval = session_interval * (index + 1);
            info!(
                "Max concurrent connections: {}, Starting sessions: {}",
                self.max_async,
                self.sessions.len()
            );

            for _ in 0..self.max_async {
                if self.sessions.is_empty() {
                    break;
                }

                let session = self.sessions.remove(0);

                tasks.push(tokio::spawn(async move {
                    // For now, we'll skip the actual connection logic
                    // TODO: Implement proper connection handling
                    debug!("Would connect session: {:?}", session);
                    sleep(Duration::from_secs(time_interval)).await;
                }));
            }

            index += self.max_async as u64;
        }

        // Wait for all tasks to complete
        for task in tasks {
            if let Err(e) = task.await {
                error!("Task execution failed: {:?}", e);
            }
        }

        Ok(())
    }

    /// Get the connection state
    pub fn state(&self) -> Arc<Mutex<ConnectionState>> {
        self.state.clone()
    }
}
