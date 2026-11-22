use std::{ops::Deref, sync::Arc};

use crate::api_impl::BotApi;
use crate::client::Context;
use crate::error::Result;
use crate::models::{
    api::{BotInfo, MessageResponse},
    message::{Ark, C2CMessage, C2CMessageParams, Embed, Keyboard, Media},
};

use super::{prepare_reply, validate_reply};

/// Stateful API surface for handling one incoming C2C message.
pub struct C2CReplySession {
    ctx: Context,
    message: C2CMessage,
    next_msg_seq: u32,
}

impl C2CReplySession {
    pub(crate) fn new(ctx: Context, message: C2CMessage) -> Result<Self> {
        let session = Self {
            ctx,
            message,
            next_msg_seq: 1,
        };
        validate_reply(&session.message.id, session.openid())?;
        Ok(session)
    }

    /// Returns the API client backing this session.
    pub fn api(&self) -> &BotApi {
        self.ctx.api()
    }

    /// Returns an owned handle to the shared API client.
    pub fn api_handle(&self) -> Arc<BotApi> {
        self.ctx.api_handle()
    }

    /// Returns bot information captured when the event was dispatched.
    pub fn bot_info(&self) -> Option<&BotInfo> {
        self.ctx.bot_info.as_deref()
    }

    /// Returns the incoming message this session is bound to.
    pub fn message(&self) -> &C2CMessage {
        &self.message
    }

    /// Returns the target user openid.
    pub fn openid(&self) -> &str {
        if self.message.author.user_openid.is_empty() {
            &self.message.author.id
        } else {
            &self.message.author.user_openid
        }
    }

    /// Sends a text reply in the current C2C reply session.
    pub async fn reply(&mut self, content: impl Into<String>) -> Result<MessageResponse> {
        self.send_text_message(content).await
    }

    /// Sends a text message in the current C2C reply session.
    pub async fn send_text_message(
        &mut self,
        content: impl Into<String>,
    ) -> Result<MessageResponse> {
        self.send_message(C2CMessageParams::new_text(content)).await
    }

    /// Sends a raw markdown message in the current C2C reply session.
    pub async fn send_markdown_message(
        &mut self,
        content: impl Into<String>,
    ) -> Result<MessageResponse> {
        self.send_message(C2CMessageParams::new_markdown(content))
            .await
    }

    /// Sends an Ark message in the current C2C reply session.
    pub async fn send_ark_message(&mut self, ark: Ark) -> Result<MessageResponse> {
        self.send_message(C2CMessageParams::new_ark(ark)).await
    }

    /// Sends an embed message in the current C2C reply session.
    pub async fn send_embed_message(&mut self, embed: Embed) -> Result<MessageResponse> {
        self.send_message(C2CMessageParams::new_embed(embed)).await
    }

    /// Sends an uploaded media message in the current C2C reply session.
    pub async fn send_media_message(&mut self, media: Media) -> Result<MessageResponse> {
        self.send_message(C2CMessageParams::new_media(media)).await
    }

    /// Sends a markdown message with a keyboard in the current C2C reply session.
    pub async fn send_keyboard_message(
        &mut self,
        content: impl Into<String>,
        keyboard: Keyboard,
    ) -> Result<MessageResponse> {
        self.send_message(C2CMessageParams::new_keyboard(content, keyboard))
            .await
    }

    /// Sends a C2C message, filling reply ids and msg_seq when omitted.
    pub async fn send_message(&mut self, mut params: C2CMessageParams) -> Result<MessageResponse> {
        self.prepare_message(&mut params);
        self.api().send_c2c_message(self.openid(), params).await
    }

    /// Uploads a C2C file for this session's user.
    pub async fn post_file(
        &self,
        file_type: u32,
        url: impl Into<String>,
        srv_send_msg: Option<bool>,
    ) -> Result<Media> {
        let url = url.into();
        self.api()
            .post_c2c_file(self.openid(), file_type, &url, srv_send_msg)
            .await
    }

    fn prepare_message(&mut self, params: &mut C2CMessageParams) {
        prepare_reply(
            &self.message.id,
            &mut params.msg_id,
            params.event_id.as_deref(),
            &mut self.next_msg_seq,
            &mut params.msg_seq,
        );
    }
}

impl Deref for C2CReplySession {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
