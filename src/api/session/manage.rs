use std::{ops::Deref, sync::Arc};

use crate::api_impl::BotApi;
use crate::client::Context;
use crate::error::{BotError, Result};
use crate::manage::{C2CManageEvent, GroupManageEvent};
use crate::models::{
    api::{BotInfo, MessageResponse},
    message::{C2CMessageParams, GroupMessageParams},
};

use super::advance_msg_seq;

/// Stateful API surface for handling one group management event.
pub struct GroupManageSession {
    ctx: Context,
    event: GroupManageEvent,
    group_openid: Option<String>,
    next_msg_seq: u32,
}

impl GroupManageSession {
    pub(crate) fn new(ctx: Context, event: GroupManageEvent) -> Self {
        let group_openid = event.group_openid.clone();
        Self {
            ctx,
            event,
            group_openid,
            next_msg_seq: 1,
        }
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

    /// Returns the group management event this session is bound to.
    pub fn event(&self) -> &GroupManageEvent {
        &self.event
    }

    /// Returns the target group openid.
    pub fn group_openid(&self) -> Option<&str> {
        self.group_openid.as_deref()
    }

    /// Sends a group message, filling event_id and msg_seq when omitted.
    pub async fn send_message(
        &mut self,
        mut params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        self.prepare_message(&mut params);
        let group_openid = self.group_openid.as_deref().ok_or_else(|| {
            BotError::InvalidData("Missing group_openid for group manage session".to_string())
        })?;
        self.api().send_group_message(group_openid, params).await
    }

    fn prepare_message(&mut self, params: &mut GroupMessageParams) {
        if params.event_id.is_none() {
            params.event_id = self.event.event_id.clone();
        }
        if params.msg_seq.is_none() {
            params.msg_seq = Some(self.next_msg_seq);
        }
        advance_msg_seq(&mut self.next_msg_seq, params.msg_seq);
    }
}

impl Deref for GroupManageSession {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}

/// Stateful API surface for handling one C2C management event.
pub struct C2CManageSession {
    ctx: Context,
    event: C2CManageEvent,
    openid: Option<String>,
    next_msg_seq: u32,
}

impl C2CManageSession {
    pub(crate) fn new(ctx: Context, event: C2CManageEvent) -> Self {
        let openid = event.openid.clone();
        Self {
            ctx,
            event,
            openid,
            next_msg_seq: 1,
        }
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

    /// Returns the C2C management event this session is bound to.
    pub fn event(&self) -> &C2CManageEvent {
        &self.event
    }

    /// Returns the target user openid.
    pub fn openid(&self) -> Option<&str> {
        self.openid.as_deref()
    }

    /// Sends a C2C message, filling event_id and msg_seq when omitted.
    pub async fn send_message(&mut self, mut params: C2CMessageParams) -> Result<MessageResponse> {
        self.prepare_message(&mut params);
        let openid = self.openid.as_deref().ok_or_else(|| {
            BotError::InvalidData("Missing openid for C2C manage session".to_string())
        })?;
        self.api().send_c2c_message(openid, params).await
    }

    fn prepare_message(&mut self, params: &mut C2CMessageParams) {
        if params.event_id.is_none() {
            params.event_id = self.event.event_id.clone();
        }
        if params.msg_seq.is_none() {
            params.msg_seq = Some(self.next_msg_seq);
        }
        advance_msg_seq(&mut self.next_msg_seq, params.msg_seq);
    }
}

impl Deref for C2CManageSession {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
