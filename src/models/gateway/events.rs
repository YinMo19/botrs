/// Gateway opcode constants.
use super::OpCode;

pub mod opcodes {
    /// Dispatch event
    pub const DISPATCH: u8 = 0;
    /// Heartbeat
    pub const HEARTBEAT: u8 = 1;
    /// Identify
    pub const IDENTIFY: u8 = 2;
    /// Resume
    pub const RESUME: u8 = 6;
    /// Reconnect
    pub const RECONNECT: u8 = 7;
    /// Invalid session
    pub const INVALID_SESSION: u8 = 9;
    /// Hello
    pub const HELLO: u8 = 10;
    /// Heartbeat ACK
    pub const HEARTBEAT_ACK: u8 = 11;
}

/// Returns a human-readable name for a gateway opcode.
pub fn op_meaning(op: OpCode) -> &'static str {
    match op {
        opcodes::DISPATCH => "Event",
        opcodes::HEARTBEAT => "Heartbeat",
        opcodes::IDENTIFY => "Identify",
        opcodes::RESUME => "Resume",
        opcodes::RECONNECT => "Reconnect",
        opcodes::INVALID_SESSION => "InvalidSession",
        opcodes::HELLO => "Hello",
        opcodes::HEARTBEAT_ACK => "HeartbeatAck",
        _ => "unknown",
    }
}

pub fn event_to_intent(
    events: impl IntoIterator<Item = impl AsRef<str>>,
) -> crate::intents::Intent {
    events
        .into_iter()
        .fold(0, |intents, event| intents | event_intent(event.as_ref()))
}

fn event_intent(event: &str) -> crate::intents::Intent {
    match event {
        "GUILD_CREATE" | "GUILD_UPDATE" | "GUILD_DELETE" | "CHANNEL_CREATE" | "CHANNEL_UPDATE"
        | "CHANNEL_DELETE" => crate::intents::Intents::GUILDS,
        "GUILD_MEMBER_ADD" | "GUILD_MEMBER_UPDATE" | "GUILD_MEMBER_REMOVE" => {
            crate::intents::Intents::GUILD_MEMBERS
        }
        "MESSAGE_CREATE" | "MESSAGE_DELETE" => crate::intents::Intents::GUILD_MESSAGES,
        "GROUP_AT_MESSAGE_CREATE"
        | "C2C_MESSAGE_CREATE"
        | "SUBSCRIBE_MESSAGE_STATUS"
        | "FRIEND_ADD"
        | "FRIEND_DEL" => crate::intents::Intents::PUBLIC_MESSAGES,
        "MESSAGE_REACTION_ADD" | "MESSAGE_REACTION_REMOVE" => {
            crate::intents::Intents::GUILD_MESSAGE_REACTIONS
        }
        "AT_MESSAGE_CREATE" | "PUBLIC_MESSAGE_DELETE" => {
            crate::intents::Intents::PUBLIC_GUILD_MESSAGES
        }
        "DIRECT_MESSAGE_CREATE" | "DIRECT_MESSAGE_DELETE" => {
            crate::intents::Intents::DIRECT_MESSAGE
        }
        "AUDIO_START" | "AUDIO_FINISH" | "AUDIO_ON_MIC" | "AUDIO_OFF_MIC" => {
            crate::intents::Intents::AUDIO_ACTION
        }
        "MESSAGE_AUDIT_PASS" | "MESSAGE_AUDIT_REJECT" => crate::intents::Intents::MESSAGE_AUDIT,
        "FORUM_THREAD_CREATE"
        | "FORUM_THREAD_UPDATE"
        | "FORUM_THREAD_DELETE"
        | "FORUM_POST_CREATE"
        | "FORUM_POST_DELETE"
        | "FORUM_REPLY_CREATE"
        | "FORUM_REPLY_DELETE"
        | "FORUM_PUBLISH_AUDIT_RESULT" => crate::intents::Intents::FORUMS,
        "INTERACTION_CREATE" => crate::intents::Intents::INTERACTION,
        "ENTER_AIO" => crate::intents::Intents::ENTER_AIO,
        _ => 0,
    }
}

/// Gateway event payload aliases.
pub type WSGuildData = crate::models::guild::Guild;
pub type WSGuildMemberData = crate::models::guild::Member;
pub type WSChannelData = crate::models::channel::Channel;
pub type WSMessageData = crate::models::message::Message;
pub type WSATMessageData = crate::models::message::Message;
pub type WSDirectMessageData = crate::models::message::Message;
pub type WSMessageDeleteData = crate::models::message::MessageDelete;
pub type WSPublicMessageDeleteData = crate::models::message::MessageDelete;
pub type WSDirectMessageDeleteData = crate::models::message::MessageDelete;
pub type WSAudioData = crate::models::api::AudioAction;
pub type WSMessageReactionData = crate::reaction::MessageReaction;
pub type WSMessageAuditData = crate::models::message::MessageAudit;
pub type WSThreadData = crate::forum::Thread;
pub type WSPostData = crate::forum::Post;
pub type WSReplyData = crate::forum::Reply;
pub type WSForumAuditData = crate::forum::ForumAuditResult;
pub type WSInteractionData = crate::interaction::Interaction;
pub type WSGroupATMessageData = crate::models::message::Message;
pub type WSC2CMessageData = crate::models::message::Message;
pub type WSC2CFriendData = crate::manage::C2CFriendData;
pub type WSSubscribeMsgStatus = crate::manage::SubscribeMessageStatusData;
pub type WSEnterAIOData = crate::manage::EnterAioEvent;
