use super::super::Gateway;
use super::SharedWriter;
use crate::error::Result;
use crate::models::gateway::{GatewayEvent, Identify, IdentifyProperties, Resume, opcodes};
use futures_util::SinkExt;
use std::sync::atomic::Ordering;
use tokio_tungstenite::tungstenite::Message;
use tracing::debug;

impl Gateway {
    async fn authentication_event(&self) -> Result<GatewayEvent> {
        if let Some(session_id) = &self.session_id {
            // Resume existing session
            debug!("Resuming session: {}", session_id);
            let resume = Resume {
                token: self.token.raw_access_token().await?,
                session_id: session_id.clone(),
                seq: self.last_seq.load(Ordering::Relaxed),
            };

            Ok(GatewayEvent {
                id: None,
                event_type: None,
                data: Some(serde_json::to_value(resume)?),
                sequence: None,
                opcode: opcodes::RESUME,
            })
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

            Ok(GatewayEvent {
                id: None,
                event_type: None,
                data: Some(serde_json::to_value(identify)?),
                sequence: None,
                opcode: opcodes::IDENTIFY,
            })
        }
    }

    /// Sends an identify payload to authenticate with the gateway.
    pub(super) async fn send_identify(&mut self, write: &SharedWriter) -> Result<()> {
        let identify = self.authentication_event().await?;

        let payload = serde_json::to_string(&identify)?;
        debug!("Sending identify payload");

        // Send through WebSocket
        let mut writer = write.lock().await;
        writer.send(Message::Text(payload)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::gateway::Gateway;
    use crate::intents::Intents;
    use crate::models::gateway::opcodes;
    use crate::token::Token;

    #[tokio::test]
    async fn identify_uses_qqbot_authorization_token() {
        let token = Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let gateway = Gateway::new(
            "wss://example.com",
            token,
            Intents::new().with_public_messages(),
            Some([0, 1]),
        );

        let event = gateway.authentication_event().await.unwrap();
        let data = event.data.unwrap();

        assert_eq!(event.opcode, opcodes::IDENTIFY);
        assert_eq!(data["token"], "QQBot ACCESS_TOKEN_XXXXXX");
        assert_eq!(data["intents"], Intents::PUBLIC_MESSAGES);
        assert_eq!(data["shard"], serde_json::json!([0, 1]));
    }

    #[tokio::test]
    async fn resume_uses_raw_access_token() {
        let token = Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let gateway = Gateway::new(
            "wss://example.com",
            token,
            Intents::new().with_public_messages(),
            Some([0, 1]),
        )
        .with_resume_state("SESSION_XXXXXX", 42);

        let event = gateway.authentication_event().await.unwrap();
        let data = event.data.unwrap();

        assert_eq!(event.opcode, opcodes::RESUME);
        assert_eq!(data["token"], "ACCESS_TOKEN_XXXXXX");
        assert_eq!(data["session_id"], "SESSION_XXXXXX");
        assert_eq!(data["seq"], 42);
    }
}
