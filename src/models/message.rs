//! Message-related data models for the QQ Guild Bot API.
//!
//! This module contains message types that correspond to the Python botpy implementation.

use crate::models::{HasId, Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// Represents a message in a guild channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// The message's unique ID
    pub id: Option<Snowflake>,
    /// The message content
    pub content: Option<String>,
    /// The ID of the channel this message was sent in
    pub channel_id: Option<Snowflake>,
    /// The ID of the guild this message was sent in
    pub guild_id: Option<Snowflake>,
    /// The author of this message
    pub author: Option<MessageUser>,
    /// The member information of the author
    pub member: Option<MessageMember>,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Users mentioned in this message
    pub mentions: Vec<MessageUser>,
    /// Attachments in this message
    pub attachments: Vec<MessageAttachment>,
    /// Global message sequence number
    pub seq: Option<u64>,
    /// Channel-specific message sequence number
    pub seq_in_channel: Option<String>,
    /// When this message was sent
    pub timestamp: Option<Timestamp>,
    /// Event ID from the gateway
    pub event_id: Option<String>,
}

impl Message {
    /// Creates a new message.
    pub fn new() -> Self {
        Self {
            id: None,
            content: None,
            channel_id: None,
            guild_id: None,
            author: None,
            member: None,
            message_reference: None,
            mentions: Vec::new(),
            attachments: Vec::new(),
            seq: None,
            seq_in_channel: None,
            timestamp: None,
            event_id: None,
        }
    }

    /// Creates a new message from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            author: data
                .get("author")
                .map(|v| MessageUser::from_data(v.clone())),
            member: data.get("member").map(|v| MessageMember {
                nick: v.get("nick").and_then(|n| n.as_str()).map(String::from),
                roles: v.get("roles").and_then(|r| r.as_array()).map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }),
                joined_at: v
                    .get("joined_at")
                    .and_then(|j| j.as_str())
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&chrono::Utc)),
            }),
            message_reference: data.get("message_reference").map(|v| MessageReference {
                message_id: v
                    .get("message_id")
                    .and_then(|id| id.as_str())
                    .map(String::from),
            }),
            mentions: data
                .get("mentions")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| MessageUser::from_data(v.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            attachments: data
                .get("attachments")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| MessageAttachment::from_data(v.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            seq: data.get("seq").and_then(|v| v.as_u64()),
            seq_in_channel: data
                .get("seq_in_channel")
                .and_then(|v| v.as_str())
                .map(String::from),
            timestamp: data
                .get("timestamp")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            event_id: Some(event_id),
        }
    }

    /// Reply to this message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        token: &crate::token::Token,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(channel_id), Some(msg_id)) = (&self.channel_id, &self.id) {
            api.post_message(
                token,
                channel_id,
                Some(content),
                None,                     // embed
                None,                     // ark
                None,                     // message_reference
                None,                     // image
                None,                     // file_image
                Some(msg_id),             // msg_id for reply
                self.event_id.as_deref(), // event_id
                None,                     // markdown
                None,                     // keyboard
            )
            .await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing channel_id or message_id for reply".to_string(),
            ))
        }
    }

    /// Returns true if this message has content.
    pub fn has_content(&self) -> bool {
        self.content.as_ref().map_or(false, |c| !c.is_empty())
    }

    /// Returns true if this message has attachments.
    pub fn has_attachments(&self) -> bool {
        !self.attachments.is_empty()
    }

    /// Returns true if this message mentions users.
    pub fn has_mentions(&self) -> bool {
        !self.mentions.is_empty()
    }

    /// Returns true if the author is a bot.
    pub fn is_from_bot(&self) -> bool {
        self.author
            .as_ref()
            .map_or(false, |a| a.bot.unwrap_or(false))
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Message {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

/// Represents a direct message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectMessage {
    /// The message's unique ID
    pub id: Option<Snowflake>,
    /// The message content
    pub content: Option<String>,
    /// The ID of the channel this message was sent in
    pub channel_id: Option<Snowflake>,
    /// The ID of the guild this message was sent in
    pub guild_id: Option<Snowflake>,
    /// Whether this is a direct message
    pub direct_message: Option<bool>,
    /// The author of this message
    pub author: Option<DirectMessageUser>,
    /// The member information of the author
    pub member: Option<DirectMessageMember>,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Attachments in this message
    pub attachments: Vec<MessageAttachment>,
    /// Global message sequence number
    pub seq: Option<u64>,
    /// Channel-specific message sequence number
    pub seq_in_channel: Option<String>,
    /// Source guild ID
    pub src_guild_id: Option<Snowflake>,
    /// When this message was sent
    pub timestamp: Option<Timestamp>,
    /// Event ID from the gateway
    pub event_id: Option<String>,
}

impl DirectMessage {
    /// Creates a new direct message.
    pub fn new() -> Self {
        Self {
            id: None,
            content: None,
            channel_id: None,
            guild_id: None,
            direct_message: None,
            author: None,
            member: None,
            message_reference: None,
            attachments: Vec::new(),
            seq: None,
            seq_in_channel: None,
            src_guild_id: None,
            timestamp: None,
            event_id: None,
        }
    }

    /// Creates a new direct message from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            direct_message: data.get("direct_message").and_then(|v| v.as_bool()),
            author: data
                .get("author")
                .map(|v| DirectMessageUser::from_data(v.clone())),
            member: data
                .get("member")
                .map(|v| DirectMessageMember::from_data(v.clone())),
            message_reference: data
                .get("message_reference")
                .map(|v| MessageReference::from_data(v.clone())),
            attachments: data
                .get("attachments")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| MessageAttachment::from_data(v.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            seq: data.get("seq").and_then(|v| v.as_u64()),
            seq_in_channel: data
                .get("seq_in_channel")
                .and_then(|v| v.as_str())
                .map(String::from),
            src_guild_id: data
                .get("src_guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            timestamp: data
                .get("timestamp")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            event_id: Some(event_id),
        }
    }

    /// Reply to this direct message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        token: &crate::token::Token,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(guild_id), Some(msg_id)) = (&self.guild_id, &self.id) {
            api.post_dms(
                token,
                guild_id,
                Some(content),
                None,                     // embed
                None,                     // ark
                None,                     // message_reference
                None,                     // image
                None,                     // file_image
                Some(msg_id),             // msg_id for reply
                self.event_id.as_deref(), // event_id
                None,                     // markdown
                None,                     // keyboard
            )
            .await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing guild_id or message_id for DM reply".to_string(),
            ))
        }
    }
}

impl Default for DirectMessage {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMessage {
    /// The message's unique ID
    pub id: Option<Snowflake>,
    /// The message content
    pub content: Option<String>,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Users mentioned in this message
    #[serde(default)]
    pub mentions: Vec<GroupMessageUser>,
    /// Attachments in this message
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    /// Global message sequence number
    pub msg_seq: Option<u64>,
    /// When this message was sent
    pub timestamp: Option<Timestamp>,
    /// The author of this message
    pub author: Option<GroupMessageUser>,
    /// Group OpenID
    pub group_openid: Option<String>,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl GroupMessage {
    /// Creates a new group message.
    pub fn new() -> Self {
        Self {
            id: None,
            content: None,
            message_reference: None,
            mentions: Vec::new(),
            attachments: Vec::new(),
            msg_seq: None,
            timestamp: None,
            author: None,
            group_openid: None,
            event_id: None,
        }
    }

    /// Creates a new group message from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .map(String::from),
            message_reference: data
                .get("message_reference")
                .map(|v| MessageReference::from_data(v.clone())),
            mentions: data
                .get("mentions")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| GroupMessageUser::from_data(v.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            attachments: data
                .get("attachments")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| MessageAttachment::from_data(v.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            msg_seq: data.get("msg_seq").and_then(|v| v.as_u64()),
            timestamp: data
                .get("timestamp")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            author: data
                .get("author")
                .map(|v| GroupMessageUser::from_data(v.clone())),
            group_openid: data
                .get("group_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            event_id: Some(event_id),
        }
    }

    /// Reply to this group message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        token: &crate::token::Token,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(group_openid), Some(msg_id)) = (&self.group_openid, &self.id) {
            api.post_group_message(
                token,
                group_openid,
                Some(0), // msg_type: text
                Some(content),
                None,                     // embed
                None,                     // ark
                None,                     // message_reference
                None,                     // media
                Some(msg_id), // msg_id for reply - this is the key for reply functionality
                None,         // msg_seq - let server handle this
                self.event_id.as_deref(), // event_id
                None,         // markdown
                None,         // keyboard
            )
            .await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing group_openid or message_id for reply".to_string(),
            ))
        }
    }
}

impl Default for GroupMessage {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a C2C (client-to-client) message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct C2CMessage {
    /// The message's unique ID
    pub id: Option<Snowflake>,
    /// The message content
    pub content: Option<String>,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Users mentioned in this message
    pub mentions: Vec<C2CMessageUser>,
    /// Attachments in this message
    pub attachments: Vec<MessageAttachment>,
    /// Global message sequence number
    pub msg_seq: Option<u64>,
    /// When this message was sent
    pub timestamp: Option<Timestamp>,
    /// The author of this message
    pub author: Option<C2CMessageUser>,
    /// Event ID from the gateway
    pub event_id: Option<String>,
}

impl C2CMessage {
    /// Creates a new C2C message.
    pub fn new() -> Self {
        Self {
            id: None,
            content: None,
            message_reference: None,
            mentions: Vec::new(),
            attachments: Vec::new(),
            msg_seq: None,
            timestamp: None,
            author: None,
            event_id: None,
        }
    }

    /// Creates a new C2C message from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .map(String::from),
            message_reference: data
                .get("message_reference")
                .map(|v| MessageReference::from_data(v.clone())),
            mentions: data
                .get("mentions")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| C2CMessageUser::from_data(v.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            attachments: data
                .get("attachments")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|v| MessageAttachment::from_data(v.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            msg_seq: data.get("msg_seq").and_then(|v| v.as_u64()),
            timestamp: data
                .get("timestamp")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            author: data
                .get("author")
                .map(|v| C2CMessageUser::from_data(v.clone())),
            event_id: Some(event_id),
        }
    }

    /// Reply to this C2C message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        token: &crate::token::Token,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(user_openid), Some(msg_id)) = (
            self.author.as_ref().and_then(|a| a.user_openid.as_ref()),
            &self.id,
        ) {
            api.post_c2c_message(
                token,
                user_openid,
                Some(0), // msg_type: text
                Some(content),
                None,                     // embed
                None,                     // ark
                None,                     // message_reference
                None,                     // media
                Some(msg_id),             // msg_id for reply
                Some(1),                  // msg_seq
                self.event_id.as_deref(), // event_id
                None,                     // markdown
                None,                     // keyboard
            )
            .await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing user_openid or message_id for C2C reply".to_string(),
            ))
        }
    }
}

impl Default for C2CMessage {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a message audit event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageAudit {
    /// The audit ID
    pub audit_id: Option<Snowflake>,
    /// The message ID that was audited
    pub message_id: Option<Snowflake>,
    /// The channel ID where the message was posted
    pub channel_id: Option<Snowflake>,
    /// The guild ID where the message was posted
    pub guild_id: Option<Snowflake>,
    /// The audit time
    pub audit_time: Option<Timestamp>,
    /// The create time
    pub create_time: Option<Timestamp>,
    /// Event ID from the gateway
    pub event_id: Option<String>,
}

impl MessageAudit {
    /// Creates a new message audit.
    pub fn new() -> Self {
        Self {
            audit_id: None,
            message_id: None,
            channel_id: None,
            guild_id: None,
            audit_time: None,
            create_time: None,
            event_id: None,
        }
    }

    /// Creates a new message audit from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        Self {
            audit_id: data
                .get("audit_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            message_id: data
                .get("message_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            audit_time: data
                .get("audit_time")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            create_time: data
                .get("create_time")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            event_id: Some(event_id),
        }
    }
}

impl Default for MessageAudit {
    fn default() -> Self {
        Self::new()
    }
}

/// User information in a regular message.
/// Represents a user mentioned in a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageUser {
    /// The user's ID
    pub id: Option<Snowflake>,
    /// The user's username
    pub username: Option<String>,
    /// Whether the user is a bot
    pub bot: Option<bool>,
    /// The user's avatar hash
    pub avatar: Option<String>,
}

impl MessageUser {
    /// Creates a new message user from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            username: data
                .get("username")
                .and_then(|v| v.as_str())
                .map(String::from),
            bot: data.get("bot").and_then(|v| v.as_bool()),
            avatar: data
                .get("avatar")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// User information in a direct message.
/// Represents a user in a direct message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectMessageUser {
    /// The user's ID
    pub id: Option<Snowflake>,
    /// The user's username
    pub username: Option<String>,
    /// The user's avatar hash
    pub avatar: Option<String>,
}

impl DirectMessageUser {
    /// Creates a new direct message user from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            username: data
                .get("username")
                .and_then(|v| v.as_str())
                .map(String::from),
            avatar: data
                .get("avatar")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// User information in a group message.
/// Represents a user in a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMessageUser {
    /// The user's ID
    pub id: Option<String>,
    /// The member's OpenID in the group
    pub member_openid: Option<String>,
    /// The union OpenID
    pub union_openid: Option<String>,
}

impl GroupMessageUser {
    /// Creates a new group message user from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            member_openid: data
                .get("member_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            union_openid: data
                .get("union_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// User information in a C2C message.
/// Represents a user in a C2C message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct C2CMessageUser {
    /// The user's OpenID
    pub user_openid: Option<String>,
}

impl C2CMessageUser {
    /// Creates a new C2C message user from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            user_openid: data
                .get("user_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// Member information in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageMember {
    /// The member's nickname
    pub nick: Option<String>,
    /// The member's roles
    pub roles: Option<Vec<Snowflake>>,
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
}

/// Member information in a direct message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectMessageMember {
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
}

impl DirectMessageMember {
    /// Creates a new direct message member from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            joined_at: data
                .get("joined_at")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
        }
    }
}

/// Reference to another message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReference {
    /// The ID of the referenced message
    pub message_id: Option<Snowflake>,
}

impl MessageReference {
    /// Creates a new message reference from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            message_id: data
                .get("message_id")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// Attachment in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageAttachment {
    /// The attachment's ID
    pub id: Option<Snowflake>,
    /// The attachment's filename
    pub filename: Option<String>,
    /// The attachment's content type
    pub content_type: Option<String>,
    /// The attachment's size in bytes
    pub size: Option<u64>,
    /// The attachment's URL
    pub url: Option<String>,
    /// The attachment's width (for images)
    pub width: Option<u32>,
    /// The attachment's height (for images)
    pub height: Option<u32>,
}

impl MessageAttachment {
    /// Creates a new message attachment from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            filename: data
                .get("filename")
                .and_then(|v| v.as_str())
                .map(String::from),
            content_type: data
                .get("content_type")
                .and_then(|v| v.as_str())
                .map(String::from),
            size: data.get("size").and_then(|v| v.as_u64()),
            url: data.get("url").and_then(|v| v.as_str()).map(String::from),
            width: data.get("width").and_then(|v| v.as_u64()).map(|w| w as u32),
            height: data
                .get("height")
                .and_then(|v| v.as_u64())
                .map(|h| h as u32),
        }
    }

    /// Returns true if this attachment is an image.
    pub fn is_image(&self) -> bool {
        self.content_type
            .as_ref()
            .map_or(false, |ct| ct.starts_with("image/"))
    }

    /// Returns true if this attachment is a video.
    pub fn is_video(&self) -> bool {
        self.content_type
            .as_ref()
            .map_or(false, |ct| ct.starts_with("video/"))
    }

    /// Returns true if this attachment is an audio file.
    pub fn is_audio(&self) -> bool {
        self.content_type
            .as_ref()
            .map_or(false, |ct| ct.starts_with("audio/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new();
        assert!(message.id.is_none());
        assert!(message.content.is_none());
        assert!(!message.has_content());
        assert!(!message.has_attachments());
        assert!(!message.has_mentions());
    }

    #[test]
    fn test_message_with_content() {
        let mut message = Message::new();
        message.content = Some("Hello, world!".to_string());
        assert!(message.has_content());
    }

    #[test]
    fn test_message_attachment_types() {
        let mut attachment = MessageAttachment {
            id: Some("123".to_string()),
            filename: Some("image.png".to_string()),
            content_type: Some("image/png".to_string()),
            size: Some(1024),
            url: Some("https://example.com/image.png".to_string()),
            width: Some(800),
            height: Some(600),
        };

        assert!(attachment.is_image());
        assert!(!attachment.is_video());
        assert!(!attachment.is_audio());

        attachment.content_type = Some("video/mp4".to_string());
        assert!(!attachment.is_image());
        assert!(attachment.is_video());
        assert!(!attachment.is_audio());
    }

    #[test]
    fn test_bot_detection() {
        let mut message = Message::new();
        message.author = Some(MessageUser {
            id: Some("123".to_string()),
            username: Some("Bot".to_string()),
            bot: Some(true),
            avatar: None,
        });

        assert!(message.is_from_bot());

        message.author.as_mut().unwrap().bot = Some(false);
        assert!(!message.is_from_bot());
    }
}

/// Ark template message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ark {
    /// Template ID
    pub template_id: Option<u32>,
    /// Keyboard data
    pub kv: Option<Vec<ArkKv>>,
}

/// Ark key-value pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArkKv {
    /// Key
    pub key: Option<String>,
    /// Value
    pub value: Option<String>,
    /// Object data
    pub obj: Option<Vec<ArkObj>>,
}

/// Ark object structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArkObj {
    /// Object key-value pairs
    pub obj_kv: Option<Vec<ArkObjKv>>,
}

/// Ark object key-value pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArkObjKv {
    /// Key
    pub key: Option<String>,
    /// Value
    pub value: Option<String>,
}

/// Embed message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embed {
    /// Title of the embed
    pub title: Option<String>,
    /// Description of the embed
    pub description: Option<String>,
    /// URL of the embed
    pub url: Option<String>,
    /// Timestamp of the embed
    pub timestamp: Option<String>,
    /// Color of the embed
    pub color: Option<u32>,
    /// Footer information
    pub footer: Option<EmbedFooter>,
    /// Image information
    pub image: Option<EmbedImage>,
    /// Thumbnail information
    pub thumbnail: Option<EmbedThumbnail>,
    /// Video information
    pub video: Option<EmbedVideo>,
    /// Provider information
    pub provider: Option<EmbedProvider>,
    /// Author information
    pub author: Option<EmbedAuthor>,
    /// Fields in the embed
    pub fields: Option<Vec<EmbedField>>,
}

/// Embed footer structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedFooter {
    /// Footer text
    pub text: Option<String>,
    /// Footer icon URL
    pub icon_url: Option<String>,
}

/// Embed image structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedImage {
    /// Image URL
    pub url: Option<String>,
    /// Image width
    pub width: Option<u32>,
    /// Image height
    pub height: Option<u32>,
}

/// Embed thumbnail structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    /// Thumbnail URL
    pub url: Option<String>,
    /// Thumbnail width
    pub width: Option<u32>,
    /// Thumbnail height
    pub height: Option<u32>,
}

/// Embed video structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedVideo {
    /// Video URL
    pub url: Option<String>,
    /// Video width
    pub width: Option<u32>,
    /// Video height
    pub height: Option<u32>,
}

/// Embed provider structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedProvider {
    /// Provider name
    pub name: Option<String>,
    /// Provider URL
    pub url: Option<String>,
}

/// Embed author structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedAuthor {
    /// Author name
    pub name: Option<String>,
    /// Author URL
    pub url: Option<String>,
    /// Author icon URL
    pub icon_url: Option<String>,
}

/// Embed field structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbedField {
    /// Field name
    pub name: Option<String>,
    /// Field value
    pub value: Option<String>,
    /// Whether field is inline
    pub inline: Option<bool>,
}

/// Keyboard message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyboard {
    /// Keyboard content
    pub content: Option<KeyboardContent>,
}

/// Keyboard content structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardContent {
    /// Rows of buttons
    pub rows: Option<Vec<KeyboardRow>>,
}

/// Keyboard row structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardRow {
    /// Buttons in this row
    pub buttons: Option<Vec<KeyboardButton>>,
}

/// Keyboard button structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Button ID
    pub id: Option<String>,
    /// Button render data
    pub render_data: Option<KeyboardButtonRenderData>,
    /// Button action
    pub action: Option<KeyboardButtonAction>,
}

/// Keyboard button render data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRenderData {
    /// Button label
    pub label: Option<String>,
    /// Button visited label
    pub visited_label: Option<String>,
    /// Button style
    pub style: Option<u32>,
}

/// Keyboard button action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonAction {
    /// Action type
    #[serde(rename = "type")]
    pub action_type: Option<u32>,
    /// Permission data
    pub permission: Option<KeyboardButtonPermission>,
    /// Click limit per user
    pub click_limit: Option<u32>,
    /// Action data
    pub data: Option<String>,
    /// Reply flag
    pub reply: Option<bool>,
    /// Enter flag
    pub enter: Option<bool>,
}

/// Keyboard button permission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonPermission {
    /// Permission type
    #[serde(rename = "type")]
    pub permission_type: Option<u32>,
    /// Specify role IDs
    pub specify_role_ids: Option<Vec<String>>,
    /// Specify user IDs
    pub specify_user_ids: Option<Vec<String>>,
}

/// Keyboard payload structure for API requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardPayload {
    /// Keyboard content
    pub content: serde_json::Value,
}

/// Markdown message payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkdownPayload {
    /// Template ID
    pub template_id: Option<String>,
    /// Custom template ID
    pub custom_template_id: Option<String>,
    /// Template parameters
    pub params: Option<Vec<MarkdownParam>>,
    /// Markdown content
    pub content: Option<String>,
}

/// Markdown parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkdownParam {
    /// Parameter key
    pub key: Option<String>,
    /// Parameter values
    pub values: Option<Vec<String>>,
}

/// Media message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Media {
    /// File info
    pub file_info: Option<String>,
    /// TTL (time to live)
    pub ttl: Option<u32>,
}

/// Message reference structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    /// Referenced message ID
    pub message_id: Option<String>,
    /// Whether to ignore getting reference message error
    pub ignore_get_message_error: Option<bool>,
}
