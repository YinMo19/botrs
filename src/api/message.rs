use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        ApiMessage, Ark, C2CMessageParams, Embed, GroupMessageParams, Keyboard, KeyboardPayload,
        MarkdownPayload, Media, Message, MessagePagerType, MessageParams, MessageToCreate,
        MessagesPager, Reference, RichMediaMessage, SendType, SettingGuide, SettingGuideToCreate,
    },
};
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use tracing::debug;

impl BotApi {
    pub(crate) fn parse_message_response(response: Value) -> Result<Message> {
        if response
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .is_empty()
            && let Some(message) = response.get("message")
        {
            return Self::decode_json(message.clone());
        }
        Self::decode_json(response)
    }

    pub(crate) fn mention_content(user_ids: &[String]) -> String {
        user_ids
            .iter()
            .map(|user_id| format!("<@{user_id}>"))
            .collect()
    }

    pub(crate) fn channel_setting_guide_body(user_ids: &[String]) -> SettingGuideToCreate {
        SettingGuideToCreate {
            content: Some(Self::mention_content(user_ids)),
            setting_guide: None,
        }
    }

    pub(crate) fn dm_setting_guide_body(jump_guild_id: &str) -> SettingGuideToCreate {
        SettingGuideToCreate {
            content: None,
            setting_guide: Some(SettingGuide {
                guild_id: jump_guild_id.to_string(),
            }),
        }
    }

    // Message APIs

    /// Gets a specific message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    ///
    /// # Returns
    ///
    /// The message.
    pub async fn get_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Message> {
        debug!("Getting message {} in channel {}", message_id, channel_id);
        let path = resource::channel_message(channel_id, message_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::parse_message_response(response)
    }

    /// Gets channel messages using paginated requests.
    pub async fn get_messages(
        &self,
        token: &Token,
        channel_id: &str,
        pager: &MessagesPager,
    ) -> Result<Vec<Message>> {
        debug!("Getting messages in channel {}", channel_id);
        let params = pager.query_params();
        let path = resource::channel_messages(channel_id);
        self.request_json(
            token,
            Method::GET,
            &path,
            if params.is_empty() {
                None
            } else {
                Some(&params)
            },
            None::<&()>,
        )
        .await
    }

    /// Gets channel messages using simple pagination parameters.
    pub async fn get_messages_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        pager_type: Option<MessagePagerType>,
        message_id: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Message>> {
        let pager = MessagesPager::new(pager_type, message_id, limit);
        self.get_messages(token, channel_id, &pager).await
    }

    /// Sends a channel message using the structured message create payload.
    pub async fn post_message_to_create(
        &self,
        token: &Token,
        channel_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        debug!("Sending message to channel {}", channel_id);
        let path = resource::channel_messages(channel_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(msg))
            .await
    }

    /// Pascal-case alias for sending a channel message.
    pub async fn post_message_api(
        &self,
        token: &Token,
        channel_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_message_to_create(token, channel_id, msg).await
    }

    /// Edits a channel message using the structured message create payload.
    pub async fn patch_message_to_create(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        debug!("Editing message {} in channel {}", message_id, channel_id);
        let path = resource::channel_message(channel_id, message_id);
        self.request_json(token, Method::PATCH, &path, None::<&()>, Some(msg))
            .await
    }

    /// Pascal-case alias for editing a channel message.
    pub async fn patch_message_api(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.patch_message_to_create(token, channel_id, message_id, msg)
            .await
    }

    /// Sends a message to a channel using MessageParams.
    ///
    /// This is the new, recommended way to send channel messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `params` - Message parameters (see [`MessageParams`])
    ///
    /// # Returns
    ///
    /// The sent message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::MessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// // Simple text message
    /// let params = MessageParams::new_text("Hello world!");
    /// api.post_message_with_params(token, "channel_id", params).await?;
    ///
    /// // Message with reply
    /// let params = MessageParams::new_text("Reply!").with_reply("message_id");
    /// api.post_message_with_params(token, "channel_id", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_message_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending message to channel {}", channel_id);

        // Handle file_image encoding if raw bytes were provided separately
        let body = serde_json::to_value(MessageToCreate::from(params))?;

        let path = resource::channel_messages(channel_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Edits a channel message using MessageParams.
    pub async fn patch_message_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        debug!("Editing message {} in channel {}", message_id, channel_id);
        let body = serde_json::to_value(MessageToCreate::from(params))?;
        let path = resource::channel_message(channel_id, message_id);
        self.request_json(token, Method::PATCH, &path, None::<&()>, Some(&body))
            .await
    }

    /// Alias for editing a channel message.
    pub async fn patch_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        self.patch_message_with_params(token, channel_id, message_id, params)
            .await
    }

    /// Sends a message to a channel (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `content` - Message content
    /// * `embed` - Optional embed
    /// * `ark` - Optional ark template
    /// * `message_reference` - Optional message reference
    /// * `image` - Optional image URL
    /// * `file_image` - Optional file image data
    /// * `msg_id` - Optional message ID to reply to
    /// * `event_id` - Optional event ID
    /// * `markdown` - Optional markdown
    /// * `keyboard` - Optional keyboard
    ///
    /// # Returns
    ///
    /// The sent message response.
    #[deprecated(since = "0.1.0", note = "Use post_message_with_params instead")]
    pub async fn post_message(
        &self,
        token: &Token,
        channel_id: &str,
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
        let params = channel_like_message_params!(
            MessageParams,
            content,
            embed,
            ark,
            message_reference,
            image,
            file_image,
            msg_id,
            event_id,
            markdown,
            keyboard
        );

        self.post_message_with_params(token, channel_id, params)
            .await
    }

    /// Sends a group message using GroupMessageParams.
    ///
    /// This is the new, recommended way to send group messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `group_openid` - The group OpenID
    /// * `params` - Group message parameters (see [`GroupMessageParams`])
    ///
    /// # Returns
    ///
    /// The sent group message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::GroupMessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// let params = GroupMessageParams::new_text("Hello group!");
    /// api.post_group_message_with_params(token, "group_openid", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_group_message_with_params(
        &self,
        token: &Token,
        group_openid: &str,
        params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending group message to {}", group_openid);

        let body = serde_json::to_value(MessageToCreate::from(params))?;

        let path = resource::group_messages(group_openid);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Sends a group message using the structured API message envelope.
    pub async fn post_group_api_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &ApiMessage,
    ) -> Result<Message> {
        debug!("Sending group message to {}", group_openid);
        self.post_group_api_payload(token, group_openid, msg.send_type(), msg)
            .await
    }

    /// Sends a group message create payload and returns the full message.
    pub async fn post_group_message_to_create(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_group_api_payload(token, group_openid, msg.send_type(), msg)
            .await
    }

    /// Uploads or directly sends group rich media.
    pub async fn post_group_rich_media_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &RichMediaMessage,
    ) -> Result<Message> {
        self.post_group_api_payload(token, group_openid, msg.send_type(), msg)
            .await
    }

    async fn post_group_api_payload<T: Serialize + ?Sized>(
        &self,
        token: &Token,
        group_openid: &str,
        send_type: SendType,
        msg: &T,
    ) -> Result<Message> {
        self.request_json(
            token,
            Method::POST,
            &resource::group_send(group_openid, send_type),
            None::<&()>,
            Some(msg),
        )
        .await
    }

    /// Sends a group message (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `group_openid` - The group OpenID
    /// * `msg_type` - Message type (0=text, 1=rich text, 2=markdown, 3=ark, 4=embed, 7=media)
    /// * `content` - Message content
    /// * `embed` - Optional embed
    /// * `ark` - Optional ark template
    /// * `message_reference` - Optional message reference
    /// * `media` - Optional media
    /// * `msg_id` - Optional message ID to reply to
    /// * `msg_seq` - Optional message sequence number
    /// * `event_id` - Optional event ID
    /// * `markdown` - Optional markdown
    /// * `keyboard` - Optional keyboard
    ///
    /// # Returns
    ///
    /// The sent group message response.
    #[deprecated(since = "0.1.0", note = "Use post_group_message_with_params instead")]
    pub async fn post_group_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg_type: Option<u32>,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        media: Option<&Media>,
        msg_id: Option<&str>,
        msg_seq: Option<u32>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        let params = open_message_params!(
            GroupMessageParams,
            msg_type,
            content,
            embed,
            ark,
            message_reference,
            media,
            msg_id,
            msg_seq,
            event_id,
            markdown,
            keyboard
        );

        self.post_group_message_with_params(token, group_openid, params)
            .await
    }

    /// Sends a C2C (client-to-client) message using C2CMessageParams.
    ///
    /// This is the new, recommended way to send C2C messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `openid` - The user's OpenID
    /// * `params` - C2C message parameters (see [`C2CMessageParams`])
    ///
    /// # Returns
    ///
    /// The sent C2C message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::C2CMessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// let params = C2CMessageParams::new_text("Hello user!");
    /// api.post_c2c_message_with_params(token, "user_openid", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_c2c_message_with_params(
        &self,
        token: &Token,
        openid: &str,
        params: C2CMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending C2C message to {}", openid);

        let body = serde_json::to_value(MessageToCreate::from(params))?;

        let path = resource::c2c_messages(openid);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Sends a C2C message using the structured API message envelope.
    pub async fn post_c2c_api_message(
        &self,
        token: &Token,
        openid: &str,
        msg: &ApiMessage,
    ) -> Result<Message> {
        debug!("Sending C2C message to {}", openid);
        self.post_c2c_api_payload(token, openid, msg.send_type(), msg)
            .await
    }

    /// Sends a C2C message create payload and returns the full message.
    pub async fn post_c2c_message_to_create(
        &self,
        token: &Token,
        openid: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_c2c_api_payload(token, openid, msg.send_type(), msg)
            .await
    }

    /// Uploads or directly sends C2C rich media.
    pub async fn post_c2c_rich_media_message(
        &self,
        token: &Token,
        openid: &str,
        msg: &RichMediaMessage,
    ) -> Result<Message> {
        self.post_c2c_api_payload(token, openid, msg.send_type(), msg)
            .await
    }

    async fn post_c2c_api_payload<T: Serialize + ?Sized>(
        &self,
        token: &Token,
        openid: &str,
        send_type: SendType,
        msg: &T,
    ) -> Result<Message> {
        self.request_json(
            token,
            Method::POST,
            &resource::c2c_send(openid, send_type),
            None::<&()>,
            Some(msg),
        )
        .await
    }

    /// Sends a C2C (client-to-client) message (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `openid` - The user's OpenID
    /// * `msg_type` - Message type (0=text, 1=rich text, 2=markdown, 3=ark, 4=embed, 7=media)
    /// * `content` - Message content
    /// * `embed` - Optional embed
    /// * `ark` - Optional ark template
    /// * `message_reference` - Optional message reference
    /// * `media` - Optional media
    /// * `msg_id` - Optional message ID to reply to
    /// * `msg_seq` - Optional message sequence number
    /// * `event_id` - Optional event ID
    /// * `markdown` - Optional markdown
    /// * `keyboard` - Optional keyboard
    ///
    /// # Returns
    ///
    /// The sent C2C message response.
    #[deprecated(since = "0.1.0", note = "Use post_c2c_message_with_params instead")]
    pub async fn post_c2c_message(
        &self,
        token: &Token,
        openid: &str,
        msg_type: Option<u32>,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        media: Option<&Media>,
        msg_id: Option<&str>,
        msg_seq: Option<u32>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        let params = open_message_params!(
            C2CMessageParams,
            msg_type,
            content,
            embed,
            ark,
            message_reference,
            media,
            msg_id,
            msg_seq,
            event_id,
            markdown,
            keyboard
        );

        self.post_c2c_message_with_params(token, openid, params)
            .await
    }

    /// Posts a channel setting guide message.
    pub async fn post_setting_guide(
        &self,
        token: &Token,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<MessageResponse> {
        let body = Self::channel_setting_guide_body(&at_user_ids);
        let path = resource::channel_setting_guide(channel_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Posts a channel setting guide message and returns the full message.
    pub async fn post_setting_guide_message(
        &self,
        token: &Token,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<Message> {
        let body = Self::channel_setting_guide_body(&at_user_ids);
        let path = resource::channel_setting_guide(channel_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Recalls (deletes) a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `hidetip` - Whether to hide the recall tip
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn recall_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Recalling message {} in channel {}", message_id, channel_id);

        let params = Self::recall_hide_tip_query(hidetip);
        let path = resource::channel_message(channel_id, message_id);
        self.http.delete(token, &path, Some(&params)).await?;
        Ok(())
    }

    /// Recalls a C2C message.
    pub async fn retract_c2c_message(
        &self,
        token: &Token,
        openid: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Retracting C2C message {} for {}", message_id, openid);
        let params = Self::hide_tip_query(hidetip.unwrap_or(false));
        let path = resource::c2c_message(openid, message_id);
        self.http.delete(token, &path, params.as_ref()).await?;
        Ok(())
    }

    /// Recalls a group message.
    pub async fn retract_group_message(
        &self,
        token: &Token,
        group_openid: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!(
            "Retracting group message {} for {}",
            message_id, group_openid
        );
        let params = Self::hide_tip_query(hidetip.unwrap_or(false));
        let path = resource::group_message(group_openid, message_id);
        self.http.delete(token, &path, params.as_ref()).await?;
        Ok(())
    }
}
