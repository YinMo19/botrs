/// Event type alias.
pub type EventType = String;
/// Websocket opcode alias.
pub type OpCode = u8;
/// Websocket opcode alias.
pub type OPCode = OpCode;

use super::events::opcodes;

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
