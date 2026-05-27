use crate::forum::{ForumAuditResult, Post, Reply, Thread};
use crate::interaction::Interaction;
use crate::manage::{C2CFriendData, EnterAioEvent, SubscribeMessageStatusData};
use crate::models::{
    api::AudioAction,
    channel::Channel,
    gateway::{WSPayload, WSReadyData},
    guild::{Guild, Member},
    message::{Message, MessageAudit, MessageDelete},
};
use crate::reaction::MessageReaction;
use std::sync::{LazyLock, RwLock};

pub type EventParseFn = fn(&mut WSPayload, &[u8]) -> crate::Result<()>;
pub type ReadyHandler = fn(&mut WSPayload, &mut WSReadyData);
pub type ErrorNotifyHandler = fn(crate::error::SdkError);
pub type PlainEventHandler = fn(&mut WSPayload, &[u8]) -> crate::Result<()>;

macro_rules! handler_type {
    ($name:ident, $data:ty) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub fn(&mut WSPayload, &mut $data) -> crate::Result<()>);
    };
}

handler_type!(GuildEventHandler, Guild);
handler_type!(GuildMemberEventHandler, Member);
handler_type!(ChannelEventHandler, Channel);
handler_type!(MessageEventHandler, Message);
handler_type!(MessageDeleteEventHandler, MessageDelete);
handler_type!(PublicMessageDeleteEventHandler, MessageDelete);
handler_type!(DirectMessageDeleteEventHandler, MessageDelete);
handler_type!(MessageReactionEventHandler, MessageReaction);
handler_type!(ATMessageEventHandler, Message);
handler_type!(DirectMessageEventHandler, Message);
handler_type!(AudioEventHandler, AudioAction);
handler_type!(MessageAuditEventHandler, MessageAudit);
handler_type!(ThreadEventHandler, Thread);
handler_type!(PostEventHandler, Post);
handler_type!(ReplyEventHandler, Reply);
handler_type!(ForumAuditEventHandler, ForumAuditResult);
handler_type!(InteractionEventHandler, Interaction);
handler_type!(GroupATMessageEventHandler, Message);
handler_type!(C2CMessageEventHandler, Message);
handler_type!(C2CFriendEventHandler, C2CFriendData);
handler_type!(SubscribeMsgStatusEventHandler, SubscribeMessageStatusData);
handler_type!(EnterAIOEventHandler, EnterAioEvent);

#[derive(Default, Clone)]
pub struct HandlerRegistry {
    pub ready: Option<ReadyHandler>,
    pub error_notify: Option<ErrorNotifyHandler>,
    pub plain: Option<PlainEventHandler>,
    pub guild: Option<GuildEventHandler>,
    pub guild_member: Option<GuildMemberEventHandler>,
    pub channel: Option<ChannelEventHandler>,
    pub message: Option<MessageEventHandler>,
    pub message_reaction: Option<MessageReactionEventHandler>,
    pub at_message: Option<ATMessageEventHandler>,
    pub direct_message: Option<DirectMessageEventHandler>,
    pub message_audit: Option<MessageAuditEventHandler>,
    pub message_delete: Option<MessageDeleteEventHandler>,
    pub public_message_delete: Option<PublicMessageDeleteEventHandler>,
    pub direct_message_delete: Option<DirectMessageDeleteEventHandler>,
    pub audio: Option<AudioEventHandler>,
    pub thread: Option<ThreadEventHandler>,
    pub post: Option<PostEventHandler>,
    pub reply: Option<ReplyEventHandler>,
    pub forum_audit: Option<ForumAuditEventHandler>,
    pub interaction: Option<InteractionEventHandler>,
    pub group_at_message: Option<GroupATMessageEventHandler>,
    pub c2c_message: Option<C2CMessageEventHandler>,
    pub subscribe_msg_status: Option<SubscribeMsgStatusEventHandler>,
    pub c2c_friend: Option<C2CFriendEventHandler>,
    pub enter_aio: Option<EnterAIOEventHandler>,
}

pub static DEFAULT_HANDLERS: LazyLock<RwLock<HandlerRegistry>> =
    LazyLock::new(|| RwLock::new(HandlerRegistry::default()));
