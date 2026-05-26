use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn send_message(&self, channel_id: &str, content: &str) -> Result<MessageResponse> {
        let params = crate::models::message::MessageParams::new_text(content);
        self.api
            .post_message_with_params(&self.token, channel_id, params)
            .await
    }

    /// Sends a message with embed to a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID to send the message to
    /// * `content` - Optional message content
    /// * `embed` - Embed to send
    ///
    /// # Returns
    ///
    /// The sent message response.

    pub async fn send_message_with_embed(
        &self,
        channel_id: &str,
        content: Option<&str>,
        embed: &Embed,
    ) -> Result<MessageResponse> {
        let params = crate::models::message::MessageParams {
            content: content.map(|s| s.to_string()),
            embed: Some(embed.clone()),
            ..Default::default()
        };
        self.api
            .post_message_with_params(&self.token, channel_id, params)
            .await
    }

    /// Sends a reply to a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID to send the reply to
    /// * `content` - Reply content
    /// * `message_id` - The message ID to reply to
    ///
    /// # Returns
    ///
    /// The sent message response.

    pub async fn reply_message(
        &self,
        channel_id: &str,
        content: &str,
        message_id: &str,
    ) -> Result<MessageResponse> {
        let reference = Reference {
            message_id: Some(message_id.to_string()),
            ignore_get_message_error: Some(true),
        };

        let params = crate::models::message::MessageParams {
            content: Some(content.to_string()),
            message_reference: Some(reference),
            ..Default::default()
        };
        self.api
            .post_message_with_params(&self.token, channel_id, params)
            .await
    }

    /// Sends a group message.
    ///
    /// # Arguments
    ///
    /// * `group_openid` - The group OpenID
    /// * `content` - Message content
    ///
    /// # Returns
    ///
    /// The sent group message response.

    pub async fn send_group_message(
        &self,
        group_openid: &str,
        content: &str,
    ) -> Result<MessageResponse> {
        let params = crate::models::message::GroupMessageParams::new_text(content);
        self.api
            .post_group_message_with_params(&self.token, group_openid, params)
            .await
    }

    /// Sends a C2C (client-to-client) message.
    ///
    /// # Arguments
    ///
    /// * `openid` - The user's OpenID
    /// * `content` - Message content
    ///
    /// # Returns
    ///
    /// The sent C2C message response.

    pub async fn send_c2c_message(&self, openid: &str, content: &str) -> Result<MessageResponse> {
        let params = crate::models::message::C2CMessageParams::new_text(content);
        self.api
            .post_c2c_message_with_params(&self.token, openid, params)
            .await
    }

    /// Gets guild information.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Guild information.

    pub async fn get_message(&self, channel_id: &str, message_id: &str) -> Result<Message> {
        self.api
            .get_message(&self.token, channel_id, message_id)
            .await
    }

    /// Gets channel messages using pagination.

    pub async fn get_messages(
        &self,
        channel_id: &str,
        pager: &MessagesPager,
    ) -> Result<Vec<Message>> {
        self.api.get_messages(&self.token, channel_id, pager).await
    }

    /// Gets channel messages using simple pagination parameters.

    pub async fn get_messages_with_params(
        &self,
        channel_id: &str,
        pager_type: Option<MessagePagerType>,
        message_id: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Message>> {
        self.api
            .get_messages_with_params(&self.token, channel_id, pager_type, message_id, limit)
            .await
    }

    /// Edits a channel message.

    pub async fn patch_message(
        &self,
        channel_id: &str,
        message_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        self.api
            .patch_message(&self.token, channel_id, message_id, params)
            .await
    }

    /// Posts a channel setting guide message.

    pub async fn post_setting_guide(
        &self,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<MessageResponse> {
        self.api
            .post_setting_guide(&self.token, channel_id, at_user_ids)
            .await
    }

    /// Posts a channel setting guide message and returns the full message.

    pub async fn post_setting_guide_message(
        &self,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<Message> {
        self.api
            .post_setting_guide_message(&self.token, channel_id, at_user_ids)
            .await
    }

    /// Recalls (deletes) a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID to recall
    /// * `hide_tip` - Whether to hide the recall tip
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

    pub async fn recall_message(
        &self,
        channel_id: &str,
        message_id: &str,
        hide_tip: bool,
    ) -> Result<()> {
        self.api
            .recall_message(&self.token, channel_id, message_id, Some(hide_tip))
            .await
    }

    /// Recalls a C2C message.

    pub async fn retract_c2c_message(
        &self,
        openid: &str,
        message_id: &str,
        hide_tip: bool,
    ) -> Result<()> {
        self.api
            .retract_c2c_message(&self.token, openid, message_id, Some(hide_tip))
            .await
    }

    /// Recalls a group message.

    pub async fn retract_group_message(
        &self,
        group_openid: &str,
        message_id: &str,
        hide_tip: bool,
    ) -> Result<()> {
        self.api
            .retract_group_message(&self.token, group_openid, message_id, Some(hide_tip))
            .await
    }

    /// Recalls a direct message.

    pub async fn retract_dm_message(
        &self,
        guild_id: &str,
        message_id: &str,
        hide_tip: bool,
    ) -> Result<()> {
        self.api
            .retract_dm_message(&self.token, guild_id, message_id, Some(hide_tip))
            .await
    }
}
