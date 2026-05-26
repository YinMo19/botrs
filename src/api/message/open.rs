use super::legacy::OpenMessageParts;
use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        ApiMessage, Ark, C2CMessageParams, Embed, GroupMessageParams, KeyboardPayload,
        MarkdownPayload, Media, Message, MessageToCreate, Reference, RichMediaMessage, SendType,
    },
};
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use tracing::debug;

#[derive(Clone, Copy)]
enum OpenMessageTarget<'a> {
    Group(&'a str),
    C2c(&'a str),
}

impl<'a> OpenMessageTarget<'a> {
    const fn name(self) -> &'static str {
        match self {
            Self::Group(_) => "group",
            Self::C2c(_) => "C2C",
        }
    }

    const fn id(self) -> &'a str {
        match self {
            Self::Group(id) | Self::C2c(id) => id,
        }
    }

    fn send_path(self, send_type: SendType) -> String {
        match self {
            Self::Group(id) => resource::group_send(id, send_type),
            Self::C2c(id) => resource::c2c_send(id, send_type),
        }
    }
}

impl BotApi {
    /// Sends a group message using GroupMessageParams.
    pub async fn post_group_message_with_params(
        &self,
        token: &Token,
        group_openid: &str,
        params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending group message to {}", group_openid);
        let body = MessageToCreate::from(params);
        let path = resource::group_messages(group_openid);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Sends a group message using the structured API message envelope.
    pub async fn post_group_api_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &ApiMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(
            token,
            OpenMessageTarget::Group(group_openid),
            msg.send_type(),
            msg,
        )
        .await
    }

    /// Sends a group message create payload and returns the full message.
    pub async fn post_group_message_to_create(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_open_api_payload(
            token,
            OpenMessageTarget::Group(group_openid),
            msg.send_type(),
            msg,
        )
        .await
    }

    /// Uploads or directly sends group rich media.
    pub async fn post_group_rich_media_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &RichMediaMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(
            token,
            OpenMessageTarget::Group(group_openid),
            msg.send_type(),
            msg,
        )
        .await
    }

    /// Sends a group message (legacy API for backward compatibility).
    #[deprecated(since = "0.1.0", note = "Use post_group_message_with_params instead")]
    #[allow(clippy::too_many_arguments)]
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
        let params: GroupMessageParams = OpenMessageParts::new(
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
            keyboard,
        )
        .into();

        self.post_group_message_with_params(token, group_openid, params)
            .await
    }

    /// Sends a C2C (client-to-client) message using C2CMessageParams.
    pub async fn post_c2c_message_with_params(
        &self,
        token: &Token,
        openid: &str,
        params: C2CMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending C2C message to {}", openid);
        let body = MessageToCreate::from(params);
        let path = resource::c2c_messages(openid);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Sends a C2C message using the structured API message envelope.
    pub async fn post_c2c_api_message(
        &self,
        token: &Token,
        openid: &str,
        msg: &ApiMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(token, OpenMessageTarget::C2c(openid), msg.send_type(), msg)
            .await
    }

    /// Sends a C2C message create payload and returns the full message.
    pub async fn post_c2c_message_to_create(
        &self,
        token: &Token,
        openid: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_open_api_payload(token, OpenMessageTarget::C2c(openid), msg.send_type(), msg)
            .await
    }

    /// Uploads or directly sends C2C rich media.
    pub async fn post_c2c_rich_media_message(
        &self,
        token: &Token,
        openid: &str,
        msg: &RichMediaMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(token, OpenMessageTarget::C2c(openid), msg.send_type(), msg)
            .await
    }

    /// Sends a C2C (client-to-client) message (legacy API for backward compatibility).
    #[deprecated(since = "0.1.0", note = "Use post_c2c_message_with_params instead")]
    #[allow(clippy::too_many_arguments)]
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
        let params: C2CMessageParams = OpenMessageParts::new(
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
            keyboard,
        )
        .into();

        self.post_c2c_message_with_params(token, openid, params)
            .await
    }

    async fn post_open_api_payload<T>(
        &self,
        token: &Token,
        target: OpenMessageTarget<'_>,
        send_type: SendType,
        msg: &T,
    ) -> Result<Message>
    where
        T: Serialize + ?Sized,
    {
        debug!("Sending {} message to {}", target.name(), target.id());
        self.request_json(
            token,
            Method::POST,
            &target.send_path(send_type),
            None::<&()>,
            Some(msg),
        )
        .await
    }
}
