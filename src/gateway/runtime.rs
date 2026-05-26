use super::Gateway;
use super::types::{GatewayAction, WsStream};
use crate::error::{BotError, Result};
use crate::models::gateway::{
    GatewayEvent, Hello, Identify, IdentifyProperties, Ready, Resume, opcodes,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, info};
use url::Url;

impl Gateway {
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
        let mut connection_count: u64 = 0;
        loop {
            connection_count += 1;
            debug!("[botrs] 启动中... (第{}次连接)", connection_count);
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

            debug!(
                "[botrs] 等待{}秒后重连...",
                self.reconnect_interval.as_secs()
            );
            tokio::time::sleep(self.reconnect_interval).await;
        }

        Ok(())
    }

    /// Connects once and returns after the connection ends.
    ///
    /// This is the primitive used by session managers: reconnect
    /// throttling and requeueing are owned by the manager, not by recursive
    /// websocket connection loops.
    pub async fn connect_once(
        &mut self,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        self.connection_alive.store(false, Ordering::Relaxed);
        self.is_ready.store(false, Ordering::Relaxed);
        self.heartbeat_count.store(0, Ordering::Relaxed);
        self.stop_heartbeat_task();

        let result = self.try_connect(&event_sender).await;
        if result.is_err() {
            self.connection_alive.store(false, Ordering::Relaxed);
            self.is_ready.store(false, Ordering::Relaxed);
        }
        result
    }

    /// Single connection attempt
    async fn try_connect(
        &mut self,
        event_sender: &mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        // Parse gateway URL
        let url = Url::parse(&self.url).map_err(BotError::Url)?;

        // Connect to WebSocket (using standard connection like Python's simple approach)
        let (ws_stream, _) = connect_async(&url).await?;
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
                    match self
                        .handle_message_content(&text, &event_sender, &write)
                        .await
                    {
                        Ok(GatewayAction::Continue) => {}
                        Ok(GatewayAction::Reconnect) => {
                            debug!("[botrs] 系统事件要求重连，退出当前连接");
                            self.connection_alive.store(false, Ordering::Relaxed);
                            self.stop_heartbeat_task();
                            return Ok(());
                        }
                        Err(e) => {
                            debug!("Error handling message: {}", e);
                        }
                    }
                }
                Ok(Message::Binary(data)) => {
                    if let Ok(text) = String::from_utf8(data) {
                        debug!("[botrs] 接收消息: {}", text);
                        match self
                            .handle_message_content(&text, &event_sender, &write)
                            .await
                        {
                            Ok(GatewayAction::Continue) => {}
                            Ok(GatewayAction::Reconnect) => {
                                debug!("[botrs] 系统事件要求重连，退出当前连接");
                                self.connection_alive.store(false, Ordering::Relaxed);
                                self.stop_heartbeat_task();
                                return Ok(());
                            }
                            Err(e) => {
                                debug!("Error handling binary message: {}", e);
                            }
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
                        self.handle_close_code(frame.code.into())?;
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
                    return Err(BotError::WebSocket(Box::new(e)));
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
    ) -> Result<GatewayAction> {
        // Parse the gateway event
        let event: GatewayEvent = serde_json::from_str(text).map_err(BotError::Json)?;

        // Check if this is a system event first (like Python's _is_system_event)
        if let Some(action) = self.handle_system_event(&event, write).await? {
            return Ok(action);
        }

        // Update sequence number if present
        if let Some(seq) = event.sequence
            && seq > 0
        {
            self.last_seq.store(seq, Ordering::Relaxed);
        }

        // Handle dispatch events
        if event.opcode == opcodes::DISPATCH
            && let Some(event_type) = &event.event_type
        {
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

        Ok(GatewayAction::Continue)
    }

    /// Handle system events like Python's _is_system_event
    async fn handle_system_event(
        &mut self,
        event: &GatewayEvent,
        write: &Arc<Mutex<futures_util::stream::SplitSink<WsStream, Message>>>,
    ) -> Result<Option<GatewayAction>> {
        match event.opcode {
            opcodes::HELLO => {
                // Hello message with heartbeat interval
                if let Some(data) = &event.data
                    && let Ok(hello) = serde_json::from_value::<Hello>(data.clone())
                {
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
                Ok(Some(GatewayAction::Continue))
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
                Ok(Some(GatewayAction::Continue))
            }
            opcodes::RECONNECT => {
                info!("[botrs] 服务器请求重连 (RECONNECT)");
                self.can_reconnect.store(true, Ordering::Relaxed);
                self.connection_alive.store(false, Ordering::Relaxed);
                let mut writer = write.lock().await;
                if let Err(e) = writer.send(Message::Close(None)).await {
                    debug!("Failed to close websocket after RECONNECT: {}", e);
                }
                Ok(Some(GatewayAction::Reconnect))
            }
            opcodes::INVALID_SESSION => {
                info!("[botrs] 会话无效 (INVALID_SESSION)");
                self.session_id = None;
                self.last_seq.store(0, Ordering::Relaxed);
                self.is_ready.store(false, Ordering::Relaxed);
                self.can_reconnect.store(true, Ordering::Relaxed);
                self.connection_alive.store(false, Ordering::Relaxed);
                let mut writer = write.lock().await;
                if let Err(e) = writer.send(Message::Close(None)).await {
                    debug!("Failed to close websocket after INVALID_SESSION: {}", e);
                }
                Ok(Some(GatewayAction::Reconnect))
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
                Ok(Some(GatewayAction::Continue))
            }
            _ => Ok(None),
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
                id: None,
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
                id: None,
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
        writer.send(Message::Text(payload)).await?;

        Ok(())
    }
}
