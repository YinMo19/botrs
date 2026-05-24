//! Gateway event models for the QQ Guild Bot API.

use crate::intents::Intents;
use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Botgo-compatible event type alias.
pub type EventType = String;
/// Botgo-compatible websocket opcode alias.
pub type OpCode = u8;
/// Botgo-compatible websocket opcode alias.
pub type OPCode = OpCode;

pub const WS_DISPATCH_EVENT: OpCode = opcodes::DISPATCH;
pub const WS_HEARTBEAT: OpCode = opcodes::HEARTBEAT;
pub const WS_IDENTITY: OpCode = opcodes::IDENTIFY;
pub const WS_RESUME: OpCode = opcodes::RESUME;
pub const WS_RECONNECT: OpCode = opcodes::RECONNECT;
pub const WS_INVALID_SESSION: OpCode = opcodes::INVALID_SESSION;
pub const WS_HELLO: OpCode = opcodes::HELLO;
pub const WS_HEARTBEAT_ACK: OpCode = opcodes::HEARTBEAT_ACK;
pub const HTTP_CALLBACK_ACK: OpCode = 12;
pub const HTTP_CALLBACK_VALIDATION: OpCode = 13;
#[allow(non_upper_case_globals)]
pub const WSDispatchEvent: OpCode = WS_DISPATCH_EVENT;
#[allow(non_upper_case_globals)]
pub const WSHeartbeat: OpCode = WS_HEARTBEAT;
#[allow(non_upper_case_globals)]
pub const WSIdentity: OpCode = WS_IDENTITY;
#[allow(non_upper_case_globals)]
pub const WSResume: OpCode = WS_RESUME;
#[allow(non_upper_case_globals)]
pub const WSReconnect: OpCode = WS_RECONNECT;
#[allow(non_upper_case_globals)]
pub const WSInvalidSession: OpCode = WS_INVALID_SESSION;
#[allow(non_upper_case_globals)]
pub const WSHello: OpCode = WS_HELLO;
#[allow(non_upper_case_globals)]
pub const WSHeartbeatAck: OpCode = WS_HEARTBEAT_ACK;
#[allow(non_upper_case_globals)]
pub const HTTPCallbackAck: OpCode = HTTP_CALLBACK_ACK;
#[allow(non_upper_case_globals)]
pub const HTTPCallbackValidation: OpCode = HTTP_CALLBACK_VALIDATION;

#[allow(non_upper_case_globals)]
pub const EventGuildCreate: &str = "GUILD_CREATE";
#[allow(non_upper_case_globals)]
pub const EventGuildUpdate: &str = "GUILD_UPDATE";
#[allow(non_upper_case_globals)]
pub const EventGuildDelete: &str = "GUILD_DELETE";
#[allow(non_upper_case_globals)]
pub const EventChannelCreate: &str = "CHANNEL_CREATE";
#[allow(non_upper_case_globals)]
pub const EventChannelUpdate: &str = "CHANNEL_UPDATE";
#[allow(non_upper_case_globals)]
pub const EventChannelDelete: &str = "CHANNEL_DELETE";
#[allow(non_upper_case_globals)]
pub const EventGuildMemberAdd: &str = "GUILD_MEMBER_ADD";
#[allow(non_upper_case_globals)]
pub const EventGuildMemberUpdate: &str = "GUILD_MEMBER_UPDATE";
#[allow(non_upper_case_globals)]
pub const EventGuildMemberRemove: &str = "GUILD_MEMBER_REMOVE";
#[allow(non_upper_case_globals)]
pub const EventMessageCreate: &str = "MESSAGE_CREATE";
#[allow(non_upper_case_globals)]
pub const EventMessageReactionAdd: &str = "MESSAGE_REACTION_ADD";
#[allow(non_upper_case_globals)]
pub const EventMessageReactionRemove: &str = "MESSAGE_REACTION_REMOVE";
#[allow(non_upper_case_globals)]
pub const EventAtMessageCreate: &str = "AT_MESSAGE_CREATE";
#[allow(non_upper_case_globals)]
pub const EventPublicMessageDelete: &str = "PUBLIC_MESSAGE_DELETE";
#[allow(non_upper_case_globals)]
pub const EventDirectMessageCreate: &str = "DIRECT_MESSAGE_CREATE";
#[allow(non_upper_case_globals)]
pub const EventDirectMessageDelete: &str = "DIRECT_MESSAGE_DELETE";
#[allow(non_upper_case_globals)]
pub const EventAudioStart: &str = "AUDIO_START";
#[allow(non_upper_case_globals)]
pub const EventAudioFinish: &str = "AUDIO_FINISH";
#[allow(non_upper_case_globals)]
pub const EventAudioOnMic: &str = "AUDIO_ON_MIC";
#[allow(non_upper_case_globals)]
pub const EventAudioOffMic: &str = "AUDIO_OFF_MIC";
#[allow(non_upper_case_globals)]
pub const EventMessageAuditPass: &str = "MESSAGE_AUDIT_PASS";
#[allow(non_upper_case_globals)]
pub const EventMessageAuditReject: &str = "MESSAGE_AUDIT_REJECT";
#[allow(non_upper_case_globals)]
pub const EventMessageDelete: &str = "MESSAGE_DELETE";
#[allow(non_upper_case_globals)]
pub const EventForumThreadCreate: &str = "FORUM_THREAD_CREATE";
#[allow(non_upper_case_globals)]
pub const EventForumThreadUpdate: &str = "FORUM_THREAD_UPDATE";
#[allow(non_upper_case_globals)]
pub const EventForumThreadDelete: &str = "FORUM_THREAD_DELETE";
#[allow(non_upper_case_globals)]
pub const EventForumPostCreate: &str = "FORUM_POST_CREATE";
#[allow(non_upper_case_globals)]
pub const EventForumPostDelete: &str = "FORUM_POST_DELETE";
#[allow(non_upper_case_globals)]
pub const EventForumReplyCreate: &str = "FORUM_REPLY_CREATE";
#[allow(non_upper_case_globals)]
pub const EventForumReplyDelete: &str = "FORUM_REPLY_DELETE";
#[allow(non_upper_case_globals)]
pub const EventForumAuditResult: &str = "FORUM_PUBLISH_AUDIT_RESULT";
#[allow(non_upper_case_globals)]
pub const EventInteractionCreate: &str = "INTERACTION_CREATE";
#[allow(non_upper_case_globals)]
pub const EventGroupAtMessageCreate: &str = "GROUP_AT_MESSAGE_CREATE";
#[allow(non_upper_case_globals)]
pub const EventC2CMessageCreate: &str = "C2C_MESSAGE_CREATE";
#[allow(non_upper_case_globals)]
pub const EventSubscribeMsgStatus: &str = "SUBSCRIBE_MESSAGE_STATUS";
#[allow(non_upper_case_globals)]
pub const EventC2CFriendAdd: &str = "FRIEND_ADD";
#[allow(non_upper_case_globals)]
pub const EventC2CFriendDel: &str = "FRIEND_DEL";
#[allow(non_upper_case_globals)]
pub const EventEnterAIO: &str = "ENTER_AIO";

/// Gateway event payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayEvent {
    /// The gateway event ID (used as passive event context ID)
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The event type
    #[serde(rename = "t")]
    pub event_type: Option<String>,
    /// The event data
    #[serde(rename = "d")]
    pub data: Option<serde_json::Value>,
    /// The sequence number
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
    /// The opcode
    #[serde(rename = "op")]
    pub opcode: u8,
}

/// Botgo-compatible websocket payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WSPayload {
    #[serde(flatten)]
    pub base: WSPayloadBase,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip)]
    pub raw_message: Option<Vec<u8>>,
}

/// Botgo-compatible websocket payload base.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSPayloadBase {
    #[serde(rename = "op")]
    pub op_code: OpCode,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u32>,
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

impl From<GatewayEvent> for WSPayload {
    fn from(event: GatewayEvent) -> Self {
        Self {
            base: WSPayloadBase {
                op_code: event.opcode,
                seq: event.sequence.map(|seq| seq as u32),
                event_type: event.event_type,
                event_id: event.id,
            },
            data: event.data,
            raw_message: None,
        }
    }
}

impl From<WSPayload> for GatewayEvent {
    fn from(payload: WSPayload) -> Self {
        Self {
            id: payload.base.event_id,
            event_type: payload.base.event_type,
            data: payload.data,
            sequence: payload.base.seq.map(u64::from),
            opcode: payload.base.op_code,
        }
    }
}

/// Gateway opcode constants.
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

pub fn op_means(op: OpCode) -> &'static str {
    match op {
        opcodes::DISPATCH => "Event",
        opcodes::HEARTBEAT => "Heartbeat",
        opcodes::IDENTIFY => "Identity",
        opcodes::RESUME => "Resume",
        opcodes::RECONNECT => "Reconnect",
        opcodes::INVALID_SESSION => "InvalidSession",
        opcodes::HELLO => "Hello",
        opcodes::HEARTBEAT_ACK => "HeartbeatAck",
        _ => "unknown",
    }
}

/// Botgo-compatible function name for opcode descriptions.
pub fn op_means_botgo(op: OpCode) -> &'static str {
    op_means(op)
}

pub use op_means_botgo as OPMeans;

/// Hello payload from the gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    /// Heartbeat interval in milliseconds
    pub heartbeat_interval: u64,
}

/// Identify payload for gateway authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identify {
    /// Bot token
    pub token: String,
    /// Intent flags
    pub intents: u32,
    /// Shard information
    pub shard: Option<[u32; 2]>,
    /// Properties
    pub properties: IdentifyProperties,
}

/// Properties for identify payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifyProperties {
    /// Operating system
    #[serde(rename = "$os")]
    pub os: String,
    /// Browser/library name
    #[serde(rename = "$browser")]
    pub browser: String,
    /// Device name
    #[serde(rename = "$device")]
    pub device: String,
}

impl Default for IdentifyProperties {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            browser: "botrs".to_string(),
            device: "botrs".to_string(),
        }
    }
}

/// Botgo-compatible identify payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSIdentityData {
    pub token: String,
    pub intents: u32,
    pub shard: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IdentifyProperties>,
}

impl From<Identify> for WSIdentityData {
    fn from(identify: Identify) -> Self {
        Self {
            token: identify.token,
            intents: identify.intents,
            shard: identify.shard.map(Vec::from).unwrap_or_default(),
            properties: Some(identify.properties),
        }
    }
}

impl From<WSIdentityData> for Identify {
    fn from(data: WSIdentityData) -> Self {
        Self {
            token: data.token,
            intents: data.intents,
            shard: (data.shard.len() == 2).then(|| [data.shard[0], data.shard[1]]),
            properties: data.properties.unwrap_or_default(),
        }
    }
}

impl WSIdentityData {
    pub fn new(token: impl Into<String>, intents: Intents, shard: Option<[u32; 2]>) -> Self {
        Self {
            token: token.into(),
            intents: intents.bits,
            shard: shard.map(Vec::from).unwrap_or_default(),
            properties: Some(IdentifyProperties::default()),
        }
    }
}

/// Resume payload for gateway reconnection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resume {
    /// Bot token
    pub token: String,
    /// Session ID
    pub session_id: String,
    /// Last sequence number
    pub seq: u64,
}

/// Botgo-compatible resume payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSResumeData {
    pub token: String,
    pub session_id: String,
    pub seq: u32,
}

impl From<Resume> for WSResumeData {
    fn from(resume: Resume) -> Self {
        Self {
            token: resume.token,
            session_id: resume.session_id,
            seq: resume.seq as u32,
        }
    }
}

impl From<WSResumeData> for Resume {
    fn from(data: WSResumeData) -> Self {
        Self {
            token: data.token,
            session_id: data.session_id,
            seq: u64::from(data.seq),
        }
    }
}

/// Botgo-compatible hello payload alias.
pub type WSHelloData = Hello;

/// Botgo-compatible ready user object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSUser {
    pub id: Snowflake,
    pub username: String,
    #[serde(default)]
    pub bot: bool,
}

/// Ready event data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ready {
    /// Gateway version
    pub version: u32,
    /// Session ID
    pub session_id: String,
    /// Bot information
    pub user: crate::models::robot::Robot,
    /// Shard information
    pub shard: Option<[u32; 2]>,
}

/// Botgo-compatible ready payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSReadyData {
    pub version: u32,
    pub session_id: String,
    pub user: WSUser,
    pub shard: Vec<u32>,
}

impl From<Ready> for WSReadyData {
    fn from(ready: Ready) -> Self {
        Self {
            version: ready.version,
            session_id: ready.session_id,
            user: WSUser {
                id: ready.user.id,
                username: ready.user.username,
                bot: ready.user.bot,
            },
            shard: ready.shard.map(Vec::from).unwrap_or_default(),
        }
    }
}

/// Botgo-compatible gateway event payload aliases.
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
pub type WSMessageReactionData = crate::reaction::Reaction;
pub type WSMessageAuditData = crate::models::message::MessageAudit;
pub type WSThreadData = crate::forum::Thread;
pub type WSPostData = crate::forum::Post;
pub type WSReplyData = crate::forum::Reply;
pub type WSForumAuditData = crate::forum::ForumAuditResult;
pub type WSInteractionData = crate::interaction::Interaction;
pub type WSGroupATMessageData = crate::models::message::Message;
pub type WSC2CMessageData = crate::models::message::Message;
pub type WSC2CFriendData = crate::manage::C2CManageEvent;
pub type WSSubscribeMsgStatus = crate::manage::SubscribeMessageStatusData;
pub type WSEnterAIOData = crate::manage::EnterAioEvent;
