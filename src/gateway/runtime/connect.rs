use super::super::Gateway;
use crate::error::{BotError, Result};
use crate::models::gateway::GatewayEvent;
use std::sync::atomic::Ordering;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tracing::debug;
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
    pub(super) async fn try_connect(
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
}
