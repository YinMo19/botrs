use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn create_dms(
        &self,
        recipient_id: &str,
        source_guild_id: &str,
    ) -> Result<DirectMessageSession> {
        self.api
            .create_dms(&self.token, source_guild_id, recipient_id)
            .await
    }

    /// Creates a direct message session.

    pub async fn create_direct_message(
        &self,
        dm: &DirectMessageToCreate,
    ) -> Result<DirectMessageSession> {
        self.api.create_direct_message(&self.token, dm).await
    }

    /// Sends a direct message and returns the full message.

    pub async fn post_direct_message(
        &self,
        guild_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.api
            .post_direct_message(&self.token, guild_id, msg)
            .await
    }

    /// Posts a DM setting guide message.

    pub async fn post_dm_setting_guide(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<MessageResponse> {
        self.api
            .post_dm_setting_guide(&self.token, guild_id, jump_guild_id)
            .await
    }

    /// Posts a DM setting guide message and returns the full message.

    pub async fn post_dm_setting_guide_message(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<Message> {
        self.api
            .post_dm_setting_guide_message(&self.token, guild_id, jump_guild_id)
            .await
    }
}
