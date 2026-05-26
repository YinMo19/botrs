use super::super::Gateway;
use super::super::types::GatewayAction;
use super::SharedWriter;
use crate::error::Result;
use crate::models::gateway::{GatewayEvent, Hello, opcodes};
use futures_util::SinkExt;
use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, info};

impl Gateway {
    /// Handle system events like Python's _is_system_event
    pub(super) async fn handle_system_event(
        &mut self,
        event: &GatewayEvent,
        write: &SharedWriter,
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
}
