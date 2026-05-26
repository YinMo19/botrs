use super::{BotApi, resource};
use crate::error::Result;
use crate::models::api::PinsMessage;
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Pins one message in a channel.
    pub async fn put_pin(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<PinsMessage> {
        debug!("Pinning message {} in channel {}", message_id, channel_id);
        let path = resource::channel_pin(channel_id, message_id);
        let response = self
            .http
            .put(token, &path, None::<&()>, None::<&()>)
            .await?;
        Self::decode_json(response)
    }

    /// Unpins one message from a channel.
    pub async fn delete_pin(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<()> {
        debug!("Unpinning message {} in channel {}", message_id, channel_id);
        let path = resource::channel_pin(channel_id, message_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Clears all pinned messages in a channel.
    pub async fn clean_pins(&self, token: &Token, channel_id: &str) -> Result<()> {
        debug!("Clearing pinned messages in channel {}", channel_id);
        let path = resource::channel_pins_all(channel_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Lists pinned messages in a channel.
    pub async fn get_pins(&self, token: &Token, channel_id: &str) -> Result<PinsMessage> {
        debug!("Getting pinned messages in channel {}", channel_id);
        let path = resource::channel_pins(channel_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }
}
