//! WebSocket gateway implementation for the QQ Guild Bot API.
//!
//! This module provides the WebSocket client for connecting to the QQ Guild Bot API gateway,
//! handling authentication, heartbeats, and event dispatching.

use crate::error::{BotError, Result};
use crate::intents::Intents;
use crate::models::gateway::*;
use crate::token::Token;
use futures_util::{SinkExt, StreamExt};

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, sleep};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, warn};
use url::Url;

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

/// WebSocket gateway client for the QQ Guild Bot API.
pub struct Gateway {
    /// Gateway URL
    url: String,
    /// Bot token
    token: Token,
    /// Intent flags
    intents: Intents,
    /// Shard information [shard_id, shard_count]
    shard: Option<[u32; 2]>,
    /// Session ID for resuming
    session_id: Option<String>,
    /// Last sequence number received
    last_seq: Arc<AtomicU64>,
    /// Heartbeat interval in milliseconds
    heartbeat_interval: Option<u64>,
    /// Whether the connection is ready
    is_ready: Arc<AtomicBool>,
    /// Whether we can reconnect
    can_reconnect: Arc<AtomicBool>,
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
            last_seq: Arc::new(AtomicU64::new(0)),
            heartbeat_interval: None,
            is_ready: Arc::new(AtomicBool::new(false)),
            can_reconnect: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Connects to the gateway and starts the event loop.
    ///
    /// # Arguments
    ///
    /// * `event_sender` - Channel to send events to
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn connect(
        &mut self,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        info!("Connecting to gateway: {}", self.url);

        // Parse and validate URL
        let url = Url::parse(&self.url).map_err(BotError::Url)?;

        // Connect to WebSocket
        let (ws_stream, _) = connect_async(&url).await.map_err(BotError::WebSocket)?;
        info!("Connected to gateway successfully");

        // Start the main event loop
        self.run_event_loop(ws_stream, event_sender).await
    }

    /// Runs the main WebSocket event loop.
    async fn run_event_loop(
        &mut self,
        ws_stream: WsStream,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        let (write_stream, mut read) = ws_stream.split();
        let write = Arc::new(Mutex::new(write_stream));

        // Set up heartbeat task
        let heartbeat_task = {
            let last_seq = self.last_seq.clone();
            let is_ready = self.is_ready.clone();
            let write_clone = write.clone();
            let heartbeat_interval = self.heartbeat_interval.unwrap_or(30000);

            async move {
                loop {
                    // Wait for heartbeat interval to be set
                    while !is_ready.load(Ordering::Relaxed) {
                        sleep(Duration::from_millis(100)).await;
                    }

                    let mut heartbeat_timer = interval(Duration::from_millis(heartbeat_interval));

                    loop {
                        heartbeat_timer.tick().await;

                        let seq = last_seq.load(Ordering::Relaxed);
                        let heartbeat = GatewayEvent {
                            event_type: None,
                            data: serde_json::json!(seq),
                            sequence: None,
                            opcode: opcodes::HEARTBEAT,
                        };

                        if let Ok(payload) = serde_json::to_string(&heartbeat) {
                            debug!("Sending heartbeat with seq: {}", seq);
                            let mut writer = write_clone.lock().await;
                            if let Err(e) = writer.send(Message::Text(payload)).await {
                                error!("Failed to send heartbeat: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
        };

        // Spawn heartbeat task
        tokio::spawn(heartbeat_task);

        // Main message handling loop
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Err(e) = self
                        .handle_message_content(&text, &event_sender, &write)
                        .await
                    {
                        error!("Error handling message: {}", e);
                    }
                }
                Ok(Message::Binary(data)) => {
                    if let Ok(text) = String::from_utf8(data) {
                        if let Err(e) = self
                            .handle_message_content(&text, &event_sender, &write)
                            .await
                        {
                            error!("Error handling binary message: {}", e);
                        }
                    }
                }
                Ok(Message::Close(close_frame)) => {
                    info!("WebSocket connection closed by server");
                    if let Some(frame) = close_frame {
                        warn!("Close code: {}, reason: {}", frame.code, frame.reason);
                        self.handle_close_code(frame.code.into()).await;
                    }
                    break;
                }
                Ok(Message::Ping(data)) => {
                    debug!("Received ping, sending pong");
                    let mut writer = write.lock().await;
                    if let Err(e) = writer.send(Message::Pong(data)).await {
                        error!("Failed to send pong: {}", e);
                    }
                }
                Ok(Message::Pong(_)) => {
                    debug!("Received pong");
                }
                Ok(Message::Frame(_)) => {
                    // Handle frame messages if needed
                    debug!("Received frame message");
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    return Err(BotError::WebSocket(e));
                }
            }
        }

        Ok(())
    }

    /// Handles an incoming WebSocket message content.
    ///
    /// # Arguments
    ///
    /// * `text` - The message text
    /// * `event_sender` - Channel to send events
    /// * `write` - WebSocket write stream
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    async fn handle_message_content(
        &mut self,
        text: &str,
        event_sender: &mpsc::UnboundedSender<GatewayEvent>,
        write: &Arc<Mutex<futures_util::stream::SplitSink<WsStream, Message>>>,
    ) -> Result<()> {
        debug!("Received message: {}", text);

        // Parse the gateway event
        let event: GatewayEvent = serde_json::from_str(text).map_err(BotError::Json)?;

        // Update sequence number if present
        if let Some(seq) = event.sequence {
            self.last_seq.store(seq, Ordering::Relaxed);
        }

        // Handle different opcodes
        match event.opcode {
            opcodes::DISPATCH => {
                // Handle special events
                if let Some(event_type) = &event.event_type {
                    match event_type.as_str() {
                        "READY" => {
                            if let Ok(ready) = serde_json::from_value::<Ready>(event.data.clone()) {
                                info!("Bot is ready! Session ID: {}", ready.session_id);
                                self.session_id = Some(ready.session_id.clone());
                                self.is_ready.store(true, Ordering::Relaxed);
                            }
                        }
                        "RESUMED" => {
                            info!("Session resumed successfully");
                            self.is_ready.store(true, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                }

                // Regular event dispatch
                if let Err(e) = event_sender.send(event) {
                    error!("Failed to send event: {}", e);
                }
            }
            opcodes::HEARTBEAT => {
                // Server requesting heartbeat
                debug!("Server requested heartbeat");
                let seq = self.last_seq.load(Ordering::Relaxed);
                let heartbeat = GatewayEvent {
                    event_type: None,
                    data: serde_json::json!(seq),
                    sequence: None,
                    opcode: opcodes::HEARTBEAT,
                };

                if let Ok(payload) = serde_json::to_string(&heartbeat) {
                    let mut writer = write.lock().await;
                    if let Err(e) = writer.send(Message::Text(payload)).await {
                        error!("Failed to send immediate heartbeat: {}", e);
                    }
                }
            }
            opcodes::RECONNECT => {
                // Server requesting reconnect
                warn!("Server requested reconnect");
                self.can_reconnect.store(true, Ordering::Relaxed);
            }
            opcodes::INVALID_SESSION => {
                // Session is invalid, need to re-identify
                warn!("Session is invalid, re-identifying");
                self.session_id = None;
                self.can_reconnect.store(false, Ordering::Relaxed);
                if let Err(e) = self.send_identify(write).await {
                    error!("Failed to re-identify: {}", e);
                }
            }
            opcodes::HELLO => {
                // Hello message with heartbeat interval
                if let Ok(hello) = serde_json::from_value::<Hello>(event.data) {
                    info!(
                        "Received Hello, heartbeat interval: {}ms",
                        hello.heartbeat_interval
                    );
                    self.heartbeat_interval = Some(hello.heartbeat_interval);

                    // Send identify or resume
                    if let Err(e) = self.send_identify(write).await {
                        error!("Failed to send identify: {}", e);
                    }
                }
            }
            opcodes::HEARTBEAT_ACK => {
                // Heartbeat acknowledgment
                debug!("Received heartbeat ACK");
            }
            _ => {
                warn!("Unknown opcode: {}", event.opcode);
            }
        }

        Ok(())
    }

    /// Sends an identify payload to authenticate with the gateway.
    async fn send_identify(
        &mut self,
        write: &Arc<Mutex<futures_util::stream::SplitSink<WsStream, Message>>>,
    ) -> Result<()> {
        let identify = if let Some(session_id) = &self.session_id {
            // Resume existing session
            debug!("Resuming session: {}", session_id);
            let resume = Resume {
                token: self.token.bot_token().await?,
                session_id: session_id.clone(),
                seq: self.last_seq.load(Ordering::Relaxed),
            };

            GatewayEvent {
                event_type: None,
                data: serde_json::to_value(resume)?,
                sequence: None,
                opcode: opcodes::RESUME,
            }
        } else {
            // New identification
            debug!("Sending identify");
            let identify = Identify {
                token: self.token.bot_token().await?,
                intents: self.intents.bits(),
                shard: self.shard,
                properties: IdentifyProperties::default(),
            };

            GatewayEvent {
                event_type: None,
                data: serde_json::to_value(identify)?,
                sequence: None,
                opcode: opcodes::IDENTIFY,
            }
        };

        let payload = serde_json::to_string(&identify)?;
        debug!("Sending identify payload");

        // Send through WebSocket
        let mut writer = write.lock().await;
        writer
            .send(Message::Text(payload))
            .await
            .map_err(BotError::WebSocket)?;

        Ok(())
    }

    /// Handles WebSocket close codes.
    async fn handle_close_code(&mut self, code: u16) {
        match code {
            4004 => {
                // Authentication failed
                warn!("Authentication failed, clearing token");
                // Token should be refreshed
            }
            9001 | 9005 => {
                // Invalid reconnect codes
                warn!("Invalid reconnect code: {}, creating new session", code);
                self.session_id = None;
                self.last_seq.store(0, Ordering::Relaxed);
                self.can_reconnect.store(false, Ordering::Relaxed);
            }
            _ => {
                debug!("WebSocket closed with code: {}", code);
            }
        }
    }

    /// Returns true if the gateway is connected and ready.
    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::Relaxed)
    }

    /// Returns true if the gateway can reconnect.
    pub fn can_reconnect(&self) -> bool {
        self.can_reconnect.load(Ordering::Relaxed)
    }

    /// Gets the current session ID.
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Gets the last sequence number.
    pub fn last_sequence(&self) -> u64 {
        self.last_seq.load(Ordering::Relaxed)
    }
}

impl std::fmt::Debug for Gateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gateway")
            .field("url", &self.url)
            .field("intents", &self.intents)
            .field("shard", &self.shard)
            .field("session_id", &self.session_id)
            .field("is_ready", &self.is_ready())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_creation() {
        let token = Token::new("test_app_id", "test_secret");
        let intents = Intents::default();
        let gateway = Gateway::new("wss://example.com", token, intents, None);

        assert!(!gateway.is_ready());
        assert!(gateway.session_id().is_none());
        assert_eq!(gateway.last_sequence(), 0);
    }

    #[test]
    fn test_gateway_with_shard() {
        let token = Token::new("test_app_id", "test_secret");
        let intents = Intents::default();
        let gateway = Gateway::new("wss://example.com", token, intents, Some([0, 1]));

        assert_eq!(gateway.shard, Some([0, 1]));
    }
}
