use crate::models::gateway::*;
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

handler_type!(GuildEventHandler, WSGuildData);
handler_type!(GuildMemberEventHandler, WSGuildMemberData);
handler_type!(ChannelEventHandler, WSChannelData);
handler_type!(MessageEventHandler, WSMessageData);
handler_type!(MessageDeleteEventHandler, WSMessageDeleteData);
handler_type!(PublicMessageDeleteEventHandler, WSPublicMessageDeleteData);
handler_type!(DirectMessageDeleteEventHandler, WSDirectMessageDeleteData);
handler_type!(MessageReactionEventHandler, WSMessageReactionData);
handler_type!(ATMessageEventHandler, WSATMessageData);
handler_type!(DirectMessageEventHandler, WSDirectMessageData);
handler_type!(AudioEventHandler, WSAudioData);
handler_type!(MessageAuditEventHandler, WSMessageAuditData);
handler_type!(ThreadEventHandler, WSThreadData);
handler_type!(PostEventHandler, WSPostData);
handler_type!(ReplyEventHandler, WSReplyData);
handler_type!(ForumAuditEventHandler, WSForumAuditData);
handler_type!(InteractionEventHandler, WSInteractionData);
handler_type!(GroupATMessageEventHandler, WSGroupATMessageData);
handler_type!(C2CMessageEventHandler, WSC2CMessageData);
handler_type!(C2CFriendEventHandler, WSC2CFriendData);
handler_type!(SubscribeMsgStatusEventHandler, WSSubscribeMsgStatus);
handler_type!(EnterAIOEventHandler, WSEnterAIOData);

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
