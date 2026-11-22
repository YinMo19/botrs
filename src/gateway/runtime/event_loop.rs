use super::super::Gateway;
use super::super::types::{GatewayAction, WsStream};
use super::SharedWriter;
use crate::error::{BotError, Result};
use crate::models::gateway::GatewayEvent;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, info};

impl Gateway {
    /// Runs the main WebSocket event loop.
    pub(super) async fn run_event_loop(
        &mut self,
        ws_stream: WsStream,
        event_sender: mpsc::Sender<GatewayEvent>,
    ) -> Result<()> {
        let (write_stream, mut read) = ws_stream.split();
        let write: SharedWriter = Arc::new(Mutex::new(write_stream));

        // Main message handling loop
        while let Some(message) = read.next().await {
            let text = match message {
                Ok(Message::Text(text)) => text,
                Ok(Message::Binary(data)) => match String::from_utf8(data) {
                    Ok(text) => text,
                    Err(_) => continue,
                },
                Ok(Message::Close(close_frame)) => {
                    debug!("[botrs] ws关闭, 停止接收消息!");
                    if let Some(frame) = close_frame {
                        info!(
                            "[botrs] 关闭, 返回码: {} , 返回信息: {}",
                            frame.code, frame.reason
                        );
                        self.handle_close_code(frame.code.into())?;
                    }
                    break;
                }
                Ok(Message::Ping(data)) => {
                    debug!("Received ping, sending pong");
                    let mut writer = write.lock().await;
                    writer.send(Message::Pong(data)).await?;
                    continue;
                }
                Ok(Message::Pong(_) | Message::Frame(_)) => continue,
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
                    return Err(BotError::WebSocket(Box::new(e)));
                }
            };
            if let GatewayAction::Reconnect = self
                .handle_message_content(&text, &event_sender, &write)
                .await?
            {
                break;
            }
        }

        let connection_duration = self
            .connection_start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::ZERO);
        let total_heartbeats = self.heartbeat_count.load(Ordering::Relaxed);

        debug!(
            "[botrs] 连接正常结束 (持续时间: {:?}, 总心跳数: {})",
            connection_duration, total_heartbeats
        );

        Ok(())
    }
}
