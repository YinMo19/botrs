use super::legacy::ChannelLikeMessageParts;
use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        Ark, Embed, Keyboard, MarkdownPayload, Message, MessagePagerType, MessageParams,
        MessageToCreate, MessagesPager, Reference,
    },
};
use crate::token::Token;
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Gets a specific message.
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
    pub async fn post_message_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending message to channel {}", channel_id);
        let body = MessageToCreate::from(params);
        let path = resource::channel_messages(channel_id);
        self.request_message_response_body(token, Method::POST, &path, &body)
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
        let body = MessageToCreate::from(params);
        let path = resource::channel_message(channel_id, message_id);
        self.request_message_response_body(token, Method::PATCH, &path, &body)
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
    #[deprecated(since = "0.1.0", note = "Use post_message_with_params instead")]
    #[allow(clippy::too_many_arguments)]
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
        let params: MessageParams = ChannelLikeMessageParts::new(
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

        self.post_message_with_params(token, channel_id, params)
            .await
    }
}
