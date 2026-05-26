use crate::models::serde_helpers::option_is_none_or_default;
use crate::models::{HasId, Snowflake, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Ark, C2CMessageParams, Embed, GroupMessageParams, MessageParams};

/// Message scene metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageScene {
    /// Message source, for example realtime voice or AI search scenes
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub source: Option<String>,
    /// Callback data returned with the message scene
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub callback_data: Option<String>,
}

/// Message delete event payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDelete {
    /// Deleted message
    pub message: Message,
    /// User who performed the operation
    pub op_user: crate::models::User,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl MessageDelete {
    /// Creates a message delete payload from gateway data.
    pub fn from_data(api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        let message_data = data.get("message").cloned().unwrap_or_else(|| data.clone());
        let op_user = data
            .get("op_user")
            .cloned()
            .map(crate::models::User::from_data)
            .unwrap_or_default();
        Self {
            message: Message::from_data(api, event_id.clone(), message_data),
            op_user,
            event_id: Some(event_id),
        }
    }
}

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
    /// Group ID for group messages when present in API responses
    pub group_id: Option<Snowflake>,
    /// The author of this message
    pub author: Option<MessageUser>,
    /// The member information of the author
    pub member: Option<MessageMember>,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Users mentioned in this message
    #[serde(default)]
    pub mentions: Vec<MessageUser>,
    /// Attachments in this message
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    /// Structured embeds in this message
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Ark payload in this message
    pub ark: Option<Ark>,
    /// Whether this is a direct message
    pub direct_message: Option<bool>,
    /// Global message sequence number
    pub seq: Option<u64>,
    /// Channel-specific message sequence number
    pub seq_in_channel: Option<String>,
    /// When this message was sent
    pub timestamp: Option<Timestamp>,
    /// When this message was edited
    pub edited_timestamp: Option<Timestamp>,
    /// Whether this message mentions everyone
    pub mention_everyone: Option<bool>,
    /// Source guild ID for direct-message scenes
    pub src_guild_id: Option<Snowflake>,
    /// Uploaded rich media file info
    pub file_info: Option<String>,
    /// Rich media file TTL in seconds
    pub ttl: Option<u32>,
    /// Message scene information
    pub message_scene: Option<MessageScene>,
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
            group_id: None,
            author: None,
            member: None,
            message_reference: None,
            mentions: Vec::new(),
            attachments: Vec::new(),
            embeds: Vec::new(),
            ark: None,
            direct_message: None,
            seq: None,
            seq_in_channel: None,
            timestamp: None,
            edited_timestamp: None,
            mention_everyone: None,
            src_guild_id: None,
            file_info: None,
            ttl: None,
            message_scene: None,
            event_id: None,
        }
    }

    /// Creates a new message from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        let mut message: Self = serde_json::from_value(data).unwrap_or_default();
        message.event_id = Some(event_id);
        message
    }

    /// Reply to this message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        token: &crate::token::Token,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(channel_id), Some(msg_id)) = (&self.channel_id, &self.id) {
            let params = MessageParams {
                content: Some(content.to_string()),
                msg_id: Some(msg_id.clone()),
                event_id: self.event_id.clone(),
                ..Default::default()
            };
            api.post_message_with_params(token, channel_id, params)
                .await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing channel_id or message_id for reply".to_string(),
            ))
        }
    }

    /// Returns true if this message has content.
    pub fn has_content(&self) -> bool {
        self.content.as_ref().is_some_and(|c| !c.is_empty())
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
        self.author.as_ref().is_some_and(|a| a.bot.unwrap_or(false))
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

/// Direct message session returned by the direct-message OpenAPI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DirectMessage {
    /// Guild ID of the DM session
    pub guild_id: Option<Snowflake>,
    /// Channel ID of the DM session
    pub channel_id: Option<Snowflake>,
    /// Creation timestamp
    pub create_time: Option<String>,
}

/// Backward-compatible alias for the direct-message session DTO.
pub type DirectMessageSession = DirectMessage;

/// Payload for creating a direct message session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DirectMessageToCreate {
    /// Source guild ID
    pub source_guild_id: String,
    /// Recipient user ID
    pub recipient_id: String,
}

impl DirectMessageToCreate {
    /// Creates a direct-message session payload.
    pub fn new(source_guild_id: impl Into<String>, recipient_id: impl Into<String>) -> Self {
        Self {
            source_guild_id: source_guild_id.into(),
            recipient_id: recipient_id.into(),
        }
    }
}

impl DirectMessage {
    /// Creates a new direct-message session.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new direct-message session from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        serde_json::from_value(data).unwrap_or_default()
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
        let mut message: Self = serde_json::from_value(data).unwrap_or_default();
        message.event_id = Some(event_id);
        message
    }

    /// Reply to this group message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        token: &crate::token::Token,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(group_openid), Some(msg_id)) = (&self.group_openid, &self.id) {
            let params = GroupMessageParams {
                msg_type: 0,
                content: Some(content.to_string()),
                msg_id: Some(msg_id.clone()),
                event_id: self.event_id.clone(),
                ..Default::default()
            };
            api.post_group_message_with_params(token, group_openid, params)
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
    pub id: Option<String>,
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
    /// Message scene information
    pub message_scene: Option<Value>,
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
            message_scene: None,
            event_id: None,
        }
    }

    /// Creates a new C2C message from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        let mut message: Self = serde_json::from_value(data).unwrap_or_default();
        message.event_id = Some(event_id);
        message
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
            let params = C2CMessageParams {
                msg_type: 0,
                content: Some(content.to_string()),
                msg_id: Some(msg_id.clone()),
                msg_seq: Some(1),
                event_id: self.event_id.clone(),
                ..Default::default()
            };
            api.post_c2c_message_with_params(token, user_openid, params)
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageAudit {
    /// The audit ID
    #[serde(default)]
    pub audit_id: Snowflake,
    /// The message ID that was audited
    #[serde(default)]
    pub message_id: Snowflake,
    /// The guild ID where the message was posted
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The channel ID where the message was posted
    #[serde(default)]
    pub channel_id: Snowflake,
    /// The audit time
    #[serde(default)]
    pub audit_time: Timestamp,
    /// The create time
    #[serde(default)]
    pub create_time: Timestamp,
    /// Channel-specific sequence number for ordering audited messages
    #[serde(default)]
    pub seq_in_channel: String,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl MessageAudit {
    /// Creates a new message audit.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new message audit from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        let mut audit: Self = serde_json::from_value(data).unwrap_or_default();
        audit.event_id = Some(event_id);
        audit
    }
}

/// User information in a regular message.
/// Represents a user mentioned in a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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
        serde_json::from_value(data).unwrap_or_default()
    }
}

/// User information in a direct message.
/// Represents a user in a direct message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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
        serde_json::from_value(data).unwrap_or_default()
    }
}

/// User information in a group message.
/// Represents a user in a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
        serde_json::from_value(data).unwrap_or_default()
    }
}

/// User information in a C2C message.
/// Represents a user in a C2C message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct C2CMessageUser {
    /// The user's ID
    pub id: Option<String>,
    /// The user's union openid
    pub union_openid: Option<String>,
    /// The user's openid
    pub user_openid: Option<String>,
}

impl C2CMessageUser {
    /// Creates a new C2C message user from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        serde_json::from_value(data).unwrap_or_default()
    }
}

/// Member information in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageMember {
    /// The member's nickname
    pub nick: Option<String>,
    /// The member's roles
    pub roles: Option<Vec<Snowflake>>,
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
}

/// Member information in a direct message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectMessageMember {
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
}

impl DirectMessageMember {
    /// Creates a new direct message member from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        serde_json::from_value(data).unwrap_or_default()
    }
}

/// Reference to another message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageReference {
    /// The ID of the referenced message
    pub message_id: Option<Snowflake>,
    /// Whether reference message fetch errors should be ignored
    pub ignore_get_message_error: Option<bool>,
}

impl MessageReference {
    /// Creates a new message reference from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        serde_json::from_value(data).unwrap_or_default()
    }
}

/// Attachment in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
        serde_json::from_value(data).unwrap_or_default()
    }

    /// Returns true if this attachment is an image.
    pub fn is_image(&self) -> bool {
        self.content_type
            .as_ref()
            .is_some_and(|ct| ct.starts_with("image/"))
    }

    /// Returns true if this attachment is a video.
    pub fn is_video(&self) -> bool {
        self.content_type
            .as_ref()
            .is_some_and(|ct| ct.starts_with("video/"))
    }

    /// Returns true if this attachment is an audio file.
    pub fn is_audio(&self) -> bool {
        self.content_type
            .as_ref()
            .is_some_and(|ct| ct.starts_with("audio/"))
    }
}
