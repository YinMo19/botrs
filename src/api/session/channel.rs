use std::{ops::Deref, sync::Arc};

use crate::api_impl::BotApi;
use crate::client::Context;
use crate::error::Result;
use crate::models::{
    api::{BotInfo, MessageResponse},
    message::{Ark, Embed, Keyboard, Message, MessageParams},
};

use super::{prepare_reply, validate_reply};

/// Stateful API surface for handling one incoming guild channel message.
pub struct ChannelReplySession {
    ctx: Context,
    message: Message,
    next_msg_seq: u32,
}

impl ChannelReplySession {
    pub(crate) fn new(ctx: Context, message: Message) -> Result<Self> {
        let session = Self {
            ctx,
            message,
            next_msg_seq: 1,
        };
        validate_reply(&session.message.id, session.channel_id())?;
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
    pub fn message(&self) -> &Message {
        &self.message
    }

    /// Returns the target channel id.
    pub fn channel_id(&self) -> &str {
        &self.message.channel_id
    }

    /// Sends a text reply in the current channel reply session.
    pub async fn reply(&mut self, content: impl Into<String>) -> Result<MessageResponse> {
        self.send_text_message(content).await
    }

    /// Sends a text message in the current channel reply session.
    pub async fn send_text_message(
        &mut self,
        content: impl Into<String>,
    ) -> Result<MessageResponse> {
        self.send_message(MessageParams::new_text(content)).await
    }

    /// Sends a raw markdown message in the current channel reply session.
    pub async fn send_markdown_message(
        &mut self,
        content: impl Into<String>,
    ) -> Result<MessageResponse> {
        self.send_message(MessageParams::new_markdown(content))
            .await
    }

    /// Sends an Ark message in the current channel reply session.
    pub async fn send_ark_message(&mut self, ark: Ark) -> Result<MessageResponse> {
        self.send_message(MessageParams::new_ark(ark)).await
    }

    /// Sends an embed message in the current channel reply session.
    pub async fn send_embed_message(&mut self, embed: Embed) -> Result<MessageResponse> {
        self.send_message(MessageParams::new_embed(embed)).await
    }

    /// Sends a markdown message with a keyboard in the current channel reply session.
    pub async fn send_keyboard_message(
        &mut self,
        content: impl Into<String>,
        keyboard: Keyboard,
    ) -> Result<MessageResponse> {
        self.send_message(MessageParams::new_keyboard(content, keyboard))
            .await
    }

    /// Sends a channel message, filling reply ids and msg_seq when omitted.
    pub async fn send_message(&mut self, mut params: MessageParams) -> Result<MessageResponse> {
        self.prepare_message(&mut params);
        self.api().send_message(self.channel_id(), params).await
    }

    fn prepare_message(&mut self, params: &mut MessageParams) {
        prepare_reply(
            &self.message.id,
            &mut params.msg_id,
            params.event_id.as_deref(),
            &mut self.next_msg_seq,
            &mut params.msg_seq,
        );
    }
}

impl Deref for ChannelReplySession {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
