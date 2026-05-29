use std::{ops::Deref, sync::Arc};

use crate::api_impl::BotApi;
use crate::client::Context;
use crate::error::Result;
use crate::models::{
    api::{BotInfo, MessageResponse},
    message::{GroupMessage, GroupMessageParams, Media},
};

use super::advance_msg_seq;

/// Stateful API surface for handling one incoming group message.
pub struct GroupReplySession {
    ctx: Context,
    message: GroupMessage,
    group_openid: String,
    next_msg_seq: u32,
}

impl GroupReplySession {
    pub(crate) fn new(ctx: Context, message: GroupMessage) -> Result<Self> {
        let group_openid = message.group_openid.clone();

        Ok(Self {
            ctx,
            message,
            group_openid,
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

    /// Returns the incoming message this session is bound to.
    pub fn message(&self) -> &GroupMessage {
        &self.message
    }

    /// Returns the target group openid.
    pub fn group_openid(&self) -> &str {
        &self.group_openid
    }

    /// Sends a text reply in the current group reply session.
    pub async fn reply(&mut self, content: impl Into<String>) -> Result<MessageResponse> {
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

    /// Sends a group message, filling reply ids and msg_seq when omitted.
    pub async fn send_message(
        &mut self,
        mut params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        self.prepare_message(&mut params);
        self.api()
            .send_group_message(&self.group_openid, params)
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
            .post_group_file(&self.group_openid, file_type, &url, srv_send_msg)
            .await
    }

    pub(crate) fn prepare_message(&mut self, params: &mut GroupMessageParams) {
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

impl Deref for GroupReplySession {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
