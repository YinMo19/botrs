use std::{ops::Deref, sync::Arc};

use crate::api_impl::BotApi;
use crate::client::Context;
use crate::error::Result;
use crate::models::{
    api::{BotInfo, MessageResponse},
    message::{DirectMessageParams, Message},
};

use super::advance_msg_seq;

/// Stateful API surface for handling one incoming direct message.
pub struct DirectReplySession {
    ctx: Context,
    message: Message,
    guild_id: String,
    next_msg_seq: u32,
}

impl DirectReplySession {
    pub(crate) fn new(ctx: Context, message: Message) -> Result<Self> {
        let guild_id = message.guild_id.clone();

        Ok(Self {
            ctx,
            message,
            guild_id,
            next_msg_seq: 1,
        })
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
        self.ctx.bot_info.as_ref()
    }

    /// Returns the incoming direct message this session is bound to.
    pub fn message(&self) -> &Message {
        &self.message
    }

    /// Returns the direct-message guild session id.
    pub fn guild_id(&self) -> &str {
        &self.guild_id
    }

    /// Sends a text reply in the current direct-message reply session.
    pub async fn reply(&mut self, content: impl Into<String>) -> Result<MessageResponse> {
        self.send_message(DirectMessageParams::new_text(content))
            .await
    }

    /// Sends a raw markdown message in the current direct-message reply session.
    pub async fn send_markdown_message(
        &mut self,
        content: impl Into<String>,
    ) -> Result<MessageResponse> {
        self.send_message(DirectMessageParams::new_markdown(content))
            .await
    }

    /// Sends a direct message, filling reply ids and msg_seq when omitted.
    pub async fn send_message(
        &mut self,
        mut params: DirectMessageParams,
    ) -> Result<MessageResponse> {
        self.prepare_message(&mut params);
        self.api().send_direct_message(&self.guild_id, params).await
    }

    fn prepare_message(&mut self, params: &mut DirectMessageParams) {
        if params.msg_id.is_none() {
            params.msg_id = Some(self.message.id.clone());
        }
        if params.event_id.is_none() {
            params.event_id = self.message.event_id.clone();
        }
        if params.msg_seq.is_none() {
            params.msg_seq = Some(self.next_msg_seq);
        }
        advance_msg_seq(&mut self.next_msg_seq, params.msg_seq);
    }
}

impl Deref for DirectReplySession {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
