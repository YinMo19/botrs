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

    /// Opens one WebSocket connection and runs its event loop.
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
