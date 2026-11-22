use std::{ops::Deref, sync::Arc};

use crate::api_impl::BotApi;
use crate::client::Context;
use crate::error::Result;
use crate::models::{
    api::{BotInfo, MessageResponse},
    group::GroupBotState,
    message::{Ark, Embed, GroupMessage, GroupMessageParams, Keyboard, Media},
};

use super::{prepare_reply, validate_reply};

/// Stateful API surface for handling one incoming group message.
pub struct GroupReplySession {
    ctx: Context,
    message: GroupMessage,
    next_msg_seq: u32,
}

impl GroupReplySession {
    pub(crate) fn new(ctx: Context, message: GroupMessage) -> Result<Self> {
        let session = Self {
            ctx,
            message,
            next_msg_seq: 1,
        };
        validate_reply(&session.message.id, session.group_openid())?;
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
    pub fn message(&self) -> &GroupMessage {
        &self.message
    }

    /// Returns the target group openid.
    pub fn group_openid(&self) -> &str {
        if self.message.group_openid.is_empty() {
            &self.message.group_id
        } else {
            &self.message.group_openid
        }
    }

    /// Reads this group's current bot settings on demand.
    pub async fn bot_state(&self) -> Result<GroupBotState> {
        self.api().get_group_bot_state(self.group_openid()).await
    }

    /// Sends a text reply in the current group reply session.
    pub async fn reply(&mut self, content: impl Into<String>) -> Result<MessageResponse> {
        self.send_text_message(content).await
    }

    /// Sends a text message in the current group reply session.
    pub async fn send_text_message(
        &mut self,
        content: impl Into<String>,
    ) -> Result<MessageResponse> {
        self.send_message(GroupMessageParams::new_text(content))
            .await
    }

    /// Sends a raw markdown message in the current group reply session.
    pub async fn send_markdown_message(
        &mut self,
        content: impl Into<String>,
    ) -> Result<MessageResponse> {
        self.send_message(GroupMessageParams::new_markdown(content))
            .await
    }

    /// Sends an Ark message in the current group reply session.
    pub async fn send_ark_message(&mut self, ark: Ark) -> Result<MessageResponse> {
        self.send_message(GroupMessageParams::new_ark(ark)).await
    }

    /// Sends an embed message in the current group reply session.
    pub async fn send_embed_message(&mut self, embed: Embed) -> Result<MessageResponse> {
        self.send_message(GroupMessageParams::new_embed(embed))
            .await
    }

    /// Sends an uploaded media message in the current group reply session.
    pub async fn send_media_message(&mut self, media: Media) -> Result<MessageResponse> {
        self.send_message(GroupMessageParams::new_media(media))
            .await
    }

    /// Sends a markdown message with a keyboard in the current group reply session.
    pub async fn send_keyboard_message(
        &mut self,
        content: impl Into<String>,
        keyboard: Keyboard,
    ) -> Result<MessageResponse> {
        self.send_message(GroupMessageParams::new_keyboard(content, keyboard))
            .await
    }

    /// Sends a group message, filling reply ids and msg_seq when omitted.
    pub async fn send_message(
        &mut self,
        mut params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        self.prepare_message(&mut params);
        self.api()
            .send_group_message(self.group_openid(), params)
            .await
    }

    /// Uploads a group file for this session's group.
    pub async fn post_file(
        &self,
        file_type: u32,
        url: impl Into<String>,
        srv_send_msg: Option<bool>,
    ) -> Result<Media> {
        let url = url.into();
        self.api()
            .post_group_file(self.group_openid(), file_type, &url, srv_send_msg)
            .await
    }

    pub(crate) fn prepare_message(&mut self, params: &mut GroupMessageParams) {
        prepare_reply(
            &self.message.id,
            &mut params.msg_id,
            params.event_id.as_deref(),
            &mut self.next_msg_seq,
            &mut params.msg_seq,
        );
    }
}

impl Deref for GroupReplySession {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
