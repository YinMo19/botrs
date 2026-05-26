use super::super::Gateway;
use super::SharedWriter;
use crate::error::Result;
use crate::models::gateway::{GatewayEvent, Identify, IdentifyProperties, Resume, opcodes};
use futures_util::SinkExt;
use std::sync::atomic::Ordering;
use tokio_tungstenite::tungstenite::Message;
use tracing::debug;

impl Gateway {
    /// Sends an identify payload to authenticate with the gateway.
    pub(super) async fn send_identify(&mut self, write: &SharedWriter) -> Result<()> {
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
            let intents = self.identify_intents();
            debug!(
                "Sending identify with intents={} shard={:?}",
                intents, self.shard
            );
            let identify = Identify {
                token: self.token.bot_token().await?,
                intents,
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
