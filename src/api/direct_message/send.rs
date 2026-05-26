use crate::api::{BotApi, message::legacy::ChannelLikeMessageParts, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        Ark, DirectMessageParams, Embed, Keyboard, MarkdownPayload, Message, MessageToCreate,
        Reference,
    },
};
use crate::token::Token;
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Sends a direct message using DirectMessageParams.
    pub async fn post_dms_with_params(
        &self,
        token: &Token,
        guild_id: &str,
        params: DirectMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending direct message to guild session {}", guild_id);
        let body = MessageToCreate::from(params);
        let path = resource::dms_messages(guild_id);
        self.request_message_response_body(token, Method::POST, &path, &body)
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
    #[deprecated(since = "0.1.0", note = "Use post_dms_with_params instead")]
    #[allow(clippy::too_many_arguments)]
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
        let params: DirectMessageParams = ChannelLikeMessageParts::new(
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
        .into();

        self.post_dms_with_params(token, guild_id, params).await
    }
}
