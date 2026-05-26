use super::Gateway;
use super::types::WsStream;
use crate::models::gateway::opcodes;
use futures_util::SinkExt;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, warn};

impl Gateway {
    /// Starts the heartbeat task using the interval provided by HELLO.
    pub(super) fn start_heartbeat_task(
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
        let heartbeat_interval_ms = self.heartbeat_interval_ms.clone();

        debug!(
            "[botrs] 心跳维持启动... ({}ms间隔)",
            self.normalized_heartbeat_interval_ms()
        );

        let handle = tokio::spawn(async move {
            let heartbeat_start_time = Instant::now();

            loop {
                let interval = Gateway::heartbeat_interval_duration(
                    heartbeat_interval_ms.load(Ordering::Relaxed),
                );
                sleep(interval).await;

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
    pub(super) fn stop_heartbeat_task(&mut self) {
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

    pub(super) fn normalized_heartbeat_interval_ms(&self) -> u64 {
        self.heartbeat_interval_ms.load(Ordering::Relaxed).max(1)
    }

    pub(super) fn heartbeat_interval_duration(interval_ms: u64) -> Duration {
        Duration::from_millis(interval_ms.max(1))
    }
}
