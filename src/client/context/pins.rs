use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn pin_message(&self, channel_id: &str, message_id: &str) -> Result<()> {
        let _ = self
            .api
            .put_pin(&self.token, channel_id, message_id)
            .await?;
        Ok(())
    }

    /// Unpins one message from a channel.
    pub async fn unpin_message(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.api
            .delete_pin(&self.token, channel_id, message_id)
            .await
    }

    /// Lists pinned messages in a channel.
    pub async fn get_pins(&self, channel_id: &str) -> Result<PinsMessage> {
        self.api.get_pins(&self.token, channel_id).await
    }

    /// Clears all pinned messages in a channel.
    pub async fn clean_pins(&self, channel_id: &str) -> Result<()> {
        self.api.clean_pins(&self.token, channel_id).await
    }
}
