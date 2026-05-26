use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        Ark, DirectMessageParams, DirectMessageSession, DirectMessageToCreate, Embed, Keyboard,
        MarkdownPayload, Message, MessageToCreate, Reference,
    },
};
use crate::token::Token;
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Sends a direct message using DirectMessageParams.
    ///
    /// This is the new, recommended way to send direct messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The DM session guild ID
    /// * `params` - Direct message parameters (see [`DirectMessageParams`])
    ///
    /// # Returns
    ///
    /// The sent direct message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::DirectMessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// let params = DirectMessageParams::new_text("Hello DM!");
    /// api.post_dms_with_params(token, "guild_id", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_dms_with_params(
        &self,
        token: &Token,
        guild_id: &str,
        params: DirectMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending direct message to guild session {}", guild_id);

        let body = serde_json::to_value(MessageToCreate::from(params))?;

        let path = resource::dms_messages(guild_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Sends a direct message using the structured message create payload.
    pub async fn post_direct_message(
        &self,
        token: &Token,
        guild_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        debug!("Sending direct message to guild {}", guild_id);
        let path = resource::dms_messages(guild_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(msg))
            .await
    }

    /// Sends a direct message (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The DM session guild ID
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
    /// The sent direct message response.
    #[deprecated(since = "0.1.0", note = "Use post_dms_with_params instead")]
    pub async fn post_dms(
        &self,
        token: &Token,
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
        let params = channel_like_message_params!(
            DirectMessageParams,
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

        self.post_dms_with_params(token, guild_id, params).await
    }

    /// Creates a direct message session using a structured payload.
    pub async fn create_direct_message(
        &self,
        token: &Token,
        dm: &DirectMessageToCreate,
    ) -> Result<DirectMessageSession> {
        debug!(
            "Creating DM session for user {} from guild {}",
            dm.recipient_id, dm.source_guild_id
        );
        let response = self
            .http
            .post(token, resource::USER_ME_DMS, None::<&()>, Some(dm))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a direct message session.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The source guild ID
    /// * `user_id` - The target user ID
    ///
    /// # Returns
    ///
    /// DM session information.
    pub async fn create_dms(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
    ) -> Result<DirectMessageSession> {
        let dm = DirectMessageToCreate::new(guild_id, user_id);
        self.create_direct_message(token, &dm).await
    }

    /// Posts a DM setting guide message.
    pub async fn post_dm_setting_guide(
        &self,
        token: &Token,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<MessageResponse> {
        let body = Self::dm_setting_guide_body(jump_guild_id);
        let path = resource::dms_setting_guide(guild_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Posts a DM setting guide message and returns the full message.
    pub async fn post_dm_setting_guide_message(
        &self,
        token: &Token,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<Message> {
        let body = Self::dm_setting_guide_body(jump_guild_id);
        let path = resource::dms_setting_guide(guild_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Recalls a direct message.
    pub async fn retract_dm_message(
        &self,
        token: &Token,
        guild_id: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Retracting DM message {} in {}", message_id, guild_id);
        let params = Self::hide_tip_query(hidetip.unwrap_or(false));
        let path = resource::dms_message(guild_id, message_id);
        self.http.delete(token, &path, params.as_ref()).await?;
        Ok(())
    }
}
