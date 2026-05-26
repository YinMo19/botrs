use super::super::Gateway;
use super::super::types::GatewayAction;
use super::SharedWriter;
use crate::error::{BotError, Result};
use crate::models::gateway::{GatewayEvent, Ready, opcodes};
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, info};

impl Gateway {
    /// Handles one decoded WebSocket text frame.
    ///
    /// System opcodes may mutate gateway state or request a control action;
    /// dispatch events are forwarded to the client event loop.
    pub(super) async fn handle_message_content(
        &mut self,
        text: &str,
        event_sender: &mpsc::UnboundedSender<GatewayEvent>,
        write: &SharedWriter,
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
}
