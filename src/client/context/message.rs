use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn send_message(&self, channel_id: &str, content: &str) -> Result<MessageResponse> {
        let params = crate::models::message::MessageParams::new_text(content);
        self.api
            .post_message_with_params(&self.token, channel_id, params)
            .await
    }

    /// Sends a channel message containing an embed and optional text content.
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

    /// Sends a channel reply and ignores lookup failures for the referenced message.
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

    /// Sends a text message to an open-platform group conversation.
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

    /// Sends a text message to a C2C conversation.
    pub async fn send_c2c_message(&self, openid: &str, content: &str) -> Result<MessageResponse> {
        let params = crate::models::message::C2CMessageParams::new_text(content);
        self.api
            .post_c2c_message_with_params(&self.token, openid, params)
            .await
    }

    /// Fetches one channel message by ID.
    pub async fn get_message(&self, channel_id: &str, message_id: &str) -> Result<Message> {
        self.api
            .get_message(&self.token, channel_id, message_id)
            .await
    }

    /// Lists channel messages using a pre-built pager.
    pub async fn get_messages(
        &self,
        channel_id: &str,
        pager: &MessagesPager,
    ) -> Result<Vec<Message>> {
        self.api.get_messages(&self.token, channel_id, pager).await
    }

    /// Lists channel messages using inline pagination parameters.
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

    /// Edits a channel message with the provided message payload.
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

    /// Sends a botpy-style inline keyboard message.
    pub async fn post_keyboard_message(
        &self,
        channel_id: &str,
        keyboard: Option<&KeyboardPayload>,
        markdown: Option<&MarkdownPayload>,
    ) -> Result<MessageResponse> {
        self.api
            .post_keyboard_message(&self.token, channel_id, keyboard, markdown)
            .await
    }

    /// Edits a guild message using botpy's inline markdown/keyboard body shape.
    pub async fn patch_guild_message(
        &self,
        channel_id: &str,
        patch_msg_id: &str,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        self.api
            .patch_guild_message(
                &self.token,
                channel_id,
                patch_msg_id,
                msg_id,
                event_id,
                markdown,
                keyboard,
            )
            .await
    }

    /// Posts a channel setting-guide message for the specified users.
    pub async fn post_setting_guide(
        &self,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<MessageResponse> {
        self.api
            .post_setting_guide(&self.token, channel_id, at_user_ids)
            .await
    }

    /// Posts a channel setting-guide message and returns the full message model.
    pub async fn post_setting_guide_message(
        &self,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<Message> {
        self.api
            .post_setting_guide_message(&self.token, channel_id, at_user_ids)
            .await
    }

    /// Recalls a channel message, optionally hiding the recall notification.
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

    /// Recalls a C2C message, optionally hiding the recall notification.
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

    /// Recalls a group message, optionally hiding the recall notification.
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

    /// Recalls a direct message, optionally hiding the recall notification.
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
