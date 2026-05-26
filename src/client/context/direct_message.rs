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

    /// Sends a direct message using botpy's locals()-style request body.
    #[allow(clippy::too_many_arguments)]
    pub async fn post_dms_botpy(
        &self,
        guild_id: &str,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        image: Option<&str>,
        file_image: Option<&[u8]>,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&Keyboard>,
    ) -> Result<MessageResponse> {
        self.api
            .post_dms_botpy(
                &self.token,
                guild_id,
                content,
                embed,
                ark,
                message_reference,
                image,
                file_image,
                msg_id,
                event_id,
                markdown,
                keyboard,
            )
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
