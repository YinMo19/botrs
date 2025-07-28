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
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;

use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, info, warn};
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
    /// Atomic heartbeat interval for sharing between tasks
    heartbeat_interval_ms: Arc<AtomicU64>,
    /// Heartbeat task handle for cleanup
    heartbeat_handle: Option<tokio::task::JoinHandle<()>>,
    /// Connection alive status
    connection_alive: Arc<AtomicBool>,
    /// Connection start time for duration tracking
    connection_start_time: Option<Instant>,
    /// Total heartbeats sent counter
    heartbeat_count: Arc<AtomicU64>,
    /// Last heartbeat ACK time for monitoring
    last_heartbeat_ack: Arc<AtomicU64>,
    /// Heartbeat sent time for ACK tracking
    last_heartbeat_sent: Arc<AtomicU64>,
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
    /// Connects to the WebSocket gateway with auto-reconnect logic.
    pub async fn connect(
        &mut self,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        let mut connection_attempt = 0;
        loop {
            connection_attempt += 1;
            debug!("[botrs] 启动中... (第{}次连接尝试)", connection_attempt);
            debug!("[botrs] 连接到网关: {}", self.url);

            // Reset states before attempting connection (like Python's session reset)
            self.connection_alive.store(false, Ordering::Relaxed);
            self.is_ready.store(false, Ordering::Relaxed);
            self.heartbeat_count.store(0, Ordering::Relaxed);
            self.stop_heartbeat_task();

            let start_time = std::time::Instant::now();
            match self.try_connect(&event_sender).await {
                Ok(_) => {
                    let duration = start_time.elapsed();
                    debug!("[botrs] 连接正常结束，持续时间: {:?}", duration);
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    debug!("[botrs] 连接错误 (持续时间: {:?}): {}", duration, e);
                    // Reset connection state on error
                    self.connection_alive.store(false, Ordering::Relaxed);
                    self.is_ready.store(false, Ordering::Relaxed);
                }
            }

            // Check if we should reconnect
            if !self.can_reconnect.load(Ordering::Relaxed) {
                debug!("[botrs] 无法重连，停止连接尝试");
                break;
            }

            // Dynamic reconnect interval like Python: round(5 / max_concurrency)
            // For single connection, use 5 seconds, but add exponential backoff for frequent failures
            let base_interval = 5;
            let reconnect_interval = if connection_attempt <= 3 {
                base_interval
            } else {
                // Exponential backoff: 5, 10, 20, 40 seconds max
                std::cmp::min(base_interval * (1 << (connection_attempt - 3)), 40)
            };

            debug!(
                "[botrs] 等待{}秒后重连... (第{}次尝试)",
                reconnect_interval, connection_attempt
            );
            tokio::time::sleep(Duration::from_secs(reconnect_interval)).await;
        }

        Ok(())
    }

    /// Single connection attempt
    async fn try_connect(
        &mut self,
        event_sender: &mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        // Parse gateway URL
        let url = Url::parse(&self.url).map_err(BotError::Url)?;

        // Connect to WebSocket (using standard connection like Python's simple approach)
        let (ws_stream, _) = connect_async(&url).await.map_err(BotError::WebSocket)?;
        debug!("[botrs] WebSocket连接建立成功");

        // Mark connection as alive and record connection start time
        self.connection_alive.store(true, Ordering::Relaxed);
        self.connection_start_time = Some(Instant::now());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        debug!("[botrs] 连接状态已标记为活跃，开始时间: {}", timestamp);

        // Start the main event loop
        self.run_event_loop(ws_stream, event_sender.clone()).await
    }

    /// Runs the main WebSocket event loop.
    async fn run_event_loop(
        &mut self,
        ws_stream: WsStream,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        let (write_stream, mut read) = ws_stream.split();
        let write = Arc::new(Mutex::new(write_stream));

        // Main message handling loop
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    debug!("[botrs] 接收消息: {}", text);
                    if let Err(e) = self
                        .handle_message_content(&text, &event_sender, &write)
                        .await
                    {
                        debug!("Error handling message: {}", e);
                    }
                }
                Ok(Message::Binary(data)) => {
                    if let Ok(text) = String::from_utf8(data) {
                        debug!("[botrs] 接收消息: {}", text);
                        if let Err(e) = self
                            .handle_message_content(&text, &event_sender, &write)
                            .await
                        {
                            debug!("Error handling binary message: {}", e);
                        }
                    }
                }
                Ok(Message::Close(close_frame)) => {
                    debug!("[botrs] ws关闭, 停止接收消息!");
                    if let Some(frame) = close_frame {
                        info!(
                            "[botrs] 关闭, 返回码: {} , 返回信息: {}",
                            frame.code, frame.reason
                        );
                        self.handle_close_code(frame.code.into()).await;
                    }
                    // Mark connection as dead and stop heartbeat task
                    self.connection_alive.store(false, Ordering::Relaxed);
                    self.stop_heartbeat_task();
                    return Ok(()); // Return to trigger reconnection
                }
                Ok(Message::Ping(data)) => {
                    debug!("Received ping, sending pong");
                    let mut writer = write.lock().await;
                    if let Err(e) = writer.send(Message::Pong(data)).await {
                        debug!("Failed to send pong: {}", e);
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
                    let connection_duration = self
                        .connection_start_time
                        .map(|t| t.elapsed())
                        .unwrap_or(Duration::ZERO);
                    let total_heartbeats = self.heartbeat_count.load(Ordering::Relaxed);

                    info!(
                        "连接断开: {} (持续时间: {:?}, 心跳数: {})",
                        e, connection_duration, total_heartbeats
                    );
                    // Mark connection as dead and stop heartbeat task on error
                    self.connection_alive.store(false, Ordering::Relaxed);
                    self.is_ready.store(false, Ordering::Relaxed);
                    self.stop_heartbeat_task();
                    return Err(BotError::WebSocket(e));
                }
            }
        }

        // Connection ended, mark as dead and stop heartbeat task
        let connection_duration = self
            .connection_start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::ZERO);
        let total_heartbeats = self.heartbeat_count.load(Ordering::Relaxed);

        debug!(
            "[botrs] 连接正常结束 (持续时间: {:?}, 总心跳数: {})",
            connection_duration, total_heartbeats
        );

        self.connection_alive.store(false, Ordering::Relaxed);
        self.is_ready.store(false, Ordering::Relaxed);
        self.stop_heartbeat_task();
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
        // Parse the gateway event
        let event: GatewayEvent = serde_json::from_str(text).map_err(BotError::Json)?;

        // Check if this is a system event first (like Python's _is_system_event)
        if self.is_system_event(&event, write).await? {
            return Ok(());
        }

        // Update sequence number if present
        if let Some(seq) = event.sequence {
            if seq > 0 {
                self.last_seq.store(seq, Ordering::Relaxed);
            }
        }

        // Handle dispatch events
        if event.opcode == opcodes::DISPATCH {
            if let Some(event_type) = &event.event_type {
                match event_type.as_str() {
                    "READY" => {
                        match event
                            .data
                            .as_ref()
                            .and_then(|d| serde_json::from_value::<Ready>(d.clone()).ok())
                        {
                            Some(ready) => {
                                self.session_id = Some(ready.session_id.clone());
                                self.is_ready.store(true, Ordering::Relaxed);

                                let elapsed = self
                                    .connection_start_time
                                    .map(|t| t.elapsed())
                                    .unwrap_or(Duration::ZERO);
                                debug!(
                                    "[botrs] 收到 READY 事件，session_id: {}，连接耗时: {:?}",
                                    ready.session_id, elapsed
                                );
                                // Start heartbeat task with 30 second interval like Python
                                self.start_heartbeat_task(write.clone());
                                debug!("[botrs] 心跳任务已启动");

                                info!("[botrs] 机器人「{}」启动成功！", ready.user.username);
                            }
                            None => {
                                debug!("[botrs] READY 事件解析失败或无数据");
                            }
                        }
                    }
                    "RESUMED" => {
                        self.is_ready.store(true, Ordering::Relaxed);

                        debug!("[botrs] 收到 RESUMED 事件");
                        // Start heartbeat task after RESUMED as well
                        self.start_heartbeat_task(write.clone());
                        debug!("[botrs] 心跳任务已重新启动");

                        info!("[botrs] 机器人重连成功! ");
                    }
                    _ => {}
                }

                // Regular event dispatch
                if let Err(e) = event_sender.send(event) {
                    debug!("Failed to send event: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Handle system events like Python's _is_system_event
    async fn is_system_event(
        &mut self,
        event: &GatewayEvent,
        write: &Arc<Mutex<futures_util::stream::SplitSink<WsStream, Message>>>,
    ) -> Result<bool> {
        match event.opcode {
            opcodes::HELLO => {
                // Hello message with heartbeat interval
                if let Some(data) = &event.data {
                    if let Ok(hello) = serde_json::from_value::<Hello>(data.clone()) {
                        debug!(
                        "[botrs] 收到 HELLO 事件，服务器建议心跳间隔: {}ms (我们使用固定30000ms)",
                        hello.heartbeat_interval
                    );
                        self.heartbeat_interval = Some(hello.heartbeat_interval);
                        // Use 30000ms like Python
                        self.heartbeat_interval_ms.store(30000, Ordering::Relaxed);

                        // Send identify or resume like Python's on_connected
                        debug!("[botrs] 发送身份验证信息");
                        if let Err(e) = self.send_identify(write).await {
                            debug!("Failed to send identify: {}", e);
                        }
                    }
                }
                Ok(true)
            }
            opcodes::HEARTBEAT_ACK => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                self.last_heartbeat_ack.store(now, Ordering::Relaxed);

                let last_sent = self.last_heartbeat_sent.load(Ordering::Relaxed);
                let ack_latency = if last_sent > 0 {
                    now.saturating_sub(last_sent)
                } else {
                    0
                };

                debug!(
                    "[botrs] 收到心跳确认 (HEARTBEAT_ACK)，延迟: {}ms",
                    ack_latency
                );
                Ok(true)
            }
            opcodes::RECONNECT => {
                info!("[botrs] 服务器请求重连 (RECONNECT)");
                self.can_reconnect.store(true, Ordering::Relaxed);
                Ok(true)
            }
            opcodes::INVALID_SESSION => {
                info!("[botrs] 会话无效 (INVALID_SESSION)");
                self.can_reconnect.store(false, Ordering::Relaxed);
                Ok(true)
            }
            opcodes::HEARTBEAT => {
                // Server requesting heartbeat
                debug!("[botrs] 服务器请求立即心跳");
                let seq = self.last_seq.load(Ordering::Relaxed);

                let heartbeat_payload = serde_json::json!({
                    "op": opcodes::HEARTBEAT,
                    "d": seq
                });

                if let Ok(payload) = serde_json::to_string(&heartbeat_payload) {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                    self.last_heartbeat_sent.store(now, Ordering::Relaxed);

                    debug!("[botrs] 发送立即心跳: seq={}", seq);
                    debug!("[botrs] 发送消息: {}", payload);
                    let mut writer = write.lock().await;
                    if let Err(e) = writer.send(Message::Text(payload)).await {
                        debug!("Failed to send immediate heartbeat: {}", e);
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
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
                data: Some(serde_json::to_value(resume)?),
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
                data: Some(serde_json::to_value(identify)?),
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

    /// Handles close codes and determines reconnection behavior
    async fn handle_close_code(&mut self, close_code: u16) {
        let invalid_reconnect_codes = [9001, 9005];
        let auth_fail_codes = [4004];

        if auth_fail_codes.contains(&close_code) {
            info!("[botrs] 鉴权失败，重置token...");
            self.session_id = None;
            self.last_seq.store(0, Ordering::Relaxed);
            self.is_ready.store(false, Ordering::Relaxed);
        }

        if invalid_reconnect_codes.contains(&close_code)
            || !self.can_reconnect.load(Ordering::Relaxed)
        {
            debug!("[botrs] 无法重连，创建新连接!");
            self.session_id = None;
            self.last_seq.store(0, Ordering::Relaxed);
            self.is_ready.store(false, Ordering::Relaxed);
            self.can_reconnect.store(false, Ordering::Relaxed);
        } else {
            debug!("[botrs] 连接断开，准备重连...");
            self.is_ready.store(false, Ordering::Relaxed);
            self.can_reconnect.store(true, Ordering::Relaxed);
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

impl Gateway {
    /// Starts the heartbeat task with fixed 30-second interval (matching Python implementation).
    fn start_heartbeat_task(
        &mut self,
        write: Arc<Mutex<futures_util::stream::SplitSink<WsStream, Message>>>,
    ) {
        // Stop any existing heartbeat task
        self.stop_heartbeat_task();

        let last_seq = self.last_seq.clone();
        let connection_alive = self.connection_alive.clone();
        let heartbeat_counter = self.heartbeat_count.clone();
        let last_heartbeat_ack = self.last_heartbeat_ack.clone();
        let last_heartbeat_sent = self.last_heartbeat_sent.clone();

        debug!("[botrs] 心跳维持启动... (30秒间隔)");

        let handle = tokio::spawn(async move {
            // Use fixed 30-second interval like Python version
            let interval_seconds = 30;
            let heartbeat_start_time = Instant::now();

            loop {
                sleep(Duration::from_secs(interval_seconds)).await;

                let current_count = heartbeat_counter.fetch_add(1, Ordering::Relaxed) + 1;
                let total_elapsed = heartbeat_start_time.elapsed();

                // Check if connection is still alive (like Python's conn check)
                if !connection_alive.load(Ordering::Relaxed) {
                    debug!("[botrs] 心跳任务检测到连接已关闭，停止心跳");
                    return;
                }

                let seq = last_seq.load(Ordering::Relaxed);
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                debug!(
                    "[botrs] 准备发送第{}次心跳，seq={}，总运行时间: {:?}，时间戳: {}",
                    current_count, seq, total_elapsed, timestamp
                );

                // Check for missing heartbeat ACKs (if we've sent heartbeats before)
                let last_ack = last_heartbeat_ack.load(Ordering::Relaxed);
                let last_sent = last_heartbeat_sent.load(Ordering::Relaxed);

                if current_count > 1 && last_sent > 0 && last_ack < last_sent {
                    let time_since_last_ack = timestamp * 1000 - last_ack;
                    if time_since_last_ack > 60000 {
                        // 60 seconds without ACK
                        warn!(
                            "[botrs] 心跳确认超时 ({}ms 未收到ACK)，可能连接有问题",
                            time_since_last_ack
                        );
                    } else {
                        debug!("[botrs] 等待心跳确认中... ({}ms)", time_since_last_ack);
                    }
                }

                // Create heartbeat payload matching Python implementation
                let heartbeat_payload = serde_json::json!({
                    "op": opcodes::HEARTBEAT,
                    "d": seq
                });

                if let Ok(payload) = serde_json::to_string(&heartbeat_payload) {
                    // Check connection state before sending (like Python's send_msg)
                    if !connection_alive.load(Ordering::Relaxed) {
                        debug!("[botrs] 发送前检测到连接已关闭，停止心跳");
                        return;
                    }

                    match write.try_lock() {
                        Ok(mut writer) => {
                            let send_start = Instant::now();
                            let now_ms = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as u64;
                            last_heartbeat_sent.store(now_ms, Ordering::Relaxed);

                            debug!("[botrs] 发送心跳包 #{}", current_count);
                            debug!("[botrs] 发送消息: {}", payload);
                            if let Err(e) = writer.send(Message::Text(payload)).await {
                                let send_duration = send_start.elapsed();
                                debug!("[botrs] 心跳发送失败 (耗时: {:?}): {}", send_duration, e);
                                debug!("[botrs] ws连接已关闭, 心跳检测停止");
                                // Mark connection as dead when heartbeat fails
                                connection_alive.store(false, Ordering::Relaxed);
                                return;
                            } else {
                                let send_duration = send_start.elapsed();
                                debug!(
                                    "[botrs] 心跳 #{} 发送成功 (耗时: {:?})，等待确认...",
                                    current_count, send_duration
                                );
                            }
                        }
                        Err(_) => {
                            // Connection is being used, skip this heartbeat cycle but continue
                            debug!("[botrs] 连接正在被使用，跳过心跳 #{}", current_count);
                            continue;
                        }
                    }
                } else {
                    debug!("[botrs] 心跳序列化失败，连接可能已关闭");
                    return;
                }
            }
        });

        self.heartbeat_handle = Some(handle);
    }

    /// Stop the heartbeat task
    fn stop_heartbeat_task(&mut self) {
        if let Some(handle) = self.heartbeat_handle.take() {
            let total_heartbeats = self.heartbeat_count.load(Ordering::Relaxed);
            let connection_duration = self
                .connection_start_time
                .map(|t| t.elapsed())
                .unwrap_or(Duration::ZERO);

            handle.abort();
            debug!(
                "[botrs] 心跳任务已停止 (总心跳数: {}, 连接持续时间: {:?})",
                total_heartbeats, connection_duration
            );
        }
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
