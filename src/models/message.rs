//! Message-related data models for the QQ Guild Bot API.
//!
//! This module contains message types that correspond to the Python botpy implementation.
//!
//! # Migration Guide: New Message Parameter API
//!
//! Starting from version 0.2.0, this module introduces cleaner parameter structs for message sending
//! to replace functions with many `Option<T>` parameters.
//!
//! ## Benefits
//!
//! - **Cleaner code**: Use `..Default::default()` instead of many `None` parameters
//! - **Better readability**: Named fields instead of positional parameters
//! - **Type safety**: Structured parameters prevent parameter ordering mistakes
//! - **Extensibility**: Easy to add new fields without breaking existing code
//! - **Builder patterns**: Convenient methods for common operations
//!
//! ## Migration Examples
//!
//! ### Channel Messages
//!
//! **Old API (deprecated):**
//! ```rust,no_run
//! # use botrs::*;
//! # async fn example(api: &BotApi, token: &Token, channel_id: &str) -> Result<()> {
//! api.post_message(
//!     token,
//!     channel_id,
//!     Some("Hello!"),    // content
//!     None,              // embed
//!     None,              // ark
//!     None,              // message_reference
//!     None,              // image
//!     None,              // file_image
//!     None,              // msg_id
//!     None,              // event_id
//!     None,              // markdown
//!     None,              // keyboard
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! **New API:**
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token, channel_id: &str) -> Result<()> {
//! // Simple text message
//! let params = MessageParams::new_text("Hello!");
//! api.post_message_with_params(token, channel_id, params).await?;
//!
//! // Message with embed
//! // Message with embed
//! # let my_embed = Default::default();
//! let params = MessageParams {
//!     content: Some("Check this out!".to_string()),
//!     embed: Some(my_embed),
//!     ..Default::default()
//! };
//! api.post_message_with_params(token, channel_id, params).await?;
//!
//! // Reply to a message
//! # let message_id = "123456";
//! let params = MessageParams::new_text("Reply content").with_reply(message_id);
//! api.post_message_with_params(token, channel_id, params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Group Messages
//!
//! **Old API (deprecated):**
//! ```rust,no_run
//! # use botrs::*;
//! # async fn example(api: &BotApi, token: &Token, group_openid: &str) -> Result<()> {
//! api.post_group_message(
//!     token,
//!     group_openid,
//!     Some(0),           // msg_type
//!     Some("Hello!"),    // content
//!     None,              // embed
//!     None,              // ark
//!     None,              // message_reference
//!     None,              // media
//!     None,              // msg_id
//!     None,              // msg_seq
//!     None,              // event_id
//!     None,              // markdown
//!     None,              // keyboard
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! **New API:**
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::GroupMessageParams;
//! # async fn example(api: &BotApi, token: &Token, group_openid: &str) -> Result<()> {
//! let params = GroupMessageParams::new_text("Hello!");
//! api.post_group_message_with_params(token, group_openid, params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Parameter Structs
//!
//! - [`MessageParams`] - For channel messages
//! - [`GroupMessageParams`] - For group messages
//! - [`C2CMessageParams`] - For C2C (client-to-client) messages
//! - [`DirectMessageParams`] - For direct messages
//!
//! Each struct provides:
//! - `new_text(content)` - Create simple text message
//! - `with_reply(message_id)` - Add reply reference
//! - `with_file_image(&bytes)` - Add file attachment (MessageParams/DirectMessageParams only)
//! - `Default` implementation for easy struct building
//!
//! ## Breaking Changes
//!
//! - Old message sending functions are **deprecated** but still functional
//! - They will be removed in version 1.0.0
//! - No immediate breaking changes - old code compiles with warnings
//!
//! See the examples in `/examples` directory for comprehensive usage patterns.

use crate::models::{HasId, Pager, Snowflake, Timestamp};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::Value;

fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

fn is_botgo_space(c: char) -> bool {
    matches!(c, ' ' | '\u{00a0}')
}

fn remove_botgo_at_mentions(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(start) = rest.find("<@!") {
        output.push_str(&rest[..start]);
        let after_marker = &rest[start + 3..];
        let digit_len = after_marker
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .map(char::len_utf8)
            .sum::<usize>();

        if digit_len > 0 && after_marker[digit_len..].starts_with('>') {
            rest = &after_marker[digit_len + 1..];
        } else {
            output.push_str("<@!");
            rest = after_marker;
        }
    }

    output.push_str(rest);
    output
}

/// Parsed command data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CMD {
    pub cmd: String,
    pub content: String,
}

pub fn mention_user(user_id: impl std::fmt::Display) -> String {
    format!("<@{user_id}>")
}

#[allow(non_snake_case)]
pub fn MentionUser(user_id: impl std::fmt::Display) -> String {
    mention_user(user_id)
}

pub fn mention_all_user() -> &'static str {
    "@everyone"
}

#[allow(non_snake_case)]
pub fn MentionAllUser() -> &'static str {
    mention_all_user()
}

pub fn mention_channel(channel_id: impl std::fmt::Display) -> String {
    format!("<#{channel_id}>")
}

#[allow(non_snake_case)]
pub fn MentionChannel(channel_id: impl std::fmt::Display) -> String {
    mention_channel(channel_id)
}

pub fn emoji(id: impl std::fmt::Display) -> String {
    format!("<emoji:{id}>")
}

#[allow(non_snake_case)]
pub fn Emoji(id: impl std::fmt::Display) -> String {
    emoji(id)
}

#[allow(non_snake_case)]
pub fn ETLInput(input: &str) -> String {
    remove_botgo_at_mentions(input)
        .trim_matches(is_botgo_space)
        .to_string()
}

pub fn parse_command(input: &str) -> CMD {
    let cleaned = ETLInput(input);
    match cleaned.split_once(' ') {
        Some((cmd, content)) => CMD {
            cmd: cmd.trim_matches(is_botgo_space).to_string(),
            content: content.to_string(),
        },
        None => CMD {
            cmd: cleaned.trim_matches(is_botgo_space).to_string(),
            content: String::new(),
        },
    }
}

#[allow(non_snake_case)]
pub fn ParseCommand(input: &str) -> CMD {
    parse_command(input)
}

/// Message scene metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageScene {
    /// Message source, for example realtime voice or AI search scenes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Callback data returned with the message scene
    #[serde(skip_serializing_if = "Option::is_none")]
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
            group_id: data
                .get("group_id")
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
            embeds: data
                .get("embeds")
                .cloned()
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
            ark: data
                .get("ark")
                .cloned()
                .and_then(|v| serde_json::from_value(v).ok()),
            direct_message: data.get("direct_message").and_then(|v| v.as_bool()),
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
            edited_timestamp: data
                .get("edited_timestamp")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            mention_everyone: data.get("mention_everyone").and_then(|v| v.as_bool()),
            src_guild_id: data
                .get("src_guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            file_info: data
                .get("file_info")
                .and_then(|v| v.as_str())
                .map(String::from),
            ttl: data.get("ttl").and_then(|v| v.as_u64()).map(|v| v as u32),
            message_scene: data
                .get("message_scene")
                .cloned()
                .and_then(|v| serde_json::from_value(v).ok()),
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

/// Direct message session returned by the direct-message OpenAPI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DirectMessageSession {
    /// Guild ID of the DM session
    pub guild_id: Option<Snowflake>,
    /// Channel ID of the DM session
    pub channel_id: Option<Snowflake>,
    /// Creation timestamp
    pub create_time: Option<String>,
}

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
        if let Some(guild_id) = &self.guild_id {
            let params = DirectMessageParams {
                content: Some(content.to_string()),
                msg_id: self.id.clone(),
                event_id: self.event_id.clone(),
                ..Default::default()
            };
            api.post_dms_with_params(token, guild_id, params).await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing guild_id for DM reply".to_string(),
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
            message_scene: data.get("message_scene").cloned(),
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
/// Represents a user in a C2C message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            union_openid: data
                .get("union_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_botgo_message_helpers() {
        assert_eq!(MentionUser("123"), "<@123>");
        assert_eq!(MentionAllUser(), "@everyone");
        assert_eq!(MentionChannel("456"), "<#456>");
        assert_eq!(Emoji(1), "<emoji:1>");
        assert_eq!(ETLInput("<@!123>  ping value"), "ping value");
        assert_eq!(
            ETLInput("\u{00a0}<@!123> ping  value\u{00a0}"),
            "ping  value"
        );
        assert_eq!(ETLInput("<@123> ping"), "<@123> ping");
        assert_eq!(ETLInput("<@!abc> ping"), "<@!abc> ping");

        let command = ParseCommand("<@!123>  /ping value");
        assert_eq!(command.cmd, "/ping");
        assert_eq!(command.content, "value");

        let command = ParseCommand("<@!123> /ping value");
        assert_eq!(command.cmd, "/ping");
        assert_eq!(command.content, "value");

        let command = ParseCommand("/ping\tvalue");
        assert_eq!(command.cmd, "/ping\tvalue");
        assert_eq!(command.content, "");
    }

    #[test]
    fn botgo_api_message_helpers_match_send_type_contract() {
        let message = MessageToCreate {
            event_id: Some("event-1".to_string()),
            ..Default::default()
        };
        let rich_media = RichMediaMessage {
            event_id: Some("ignored".to_string()),
            ..Default::default()
        };
        let reference = Reference {
            message_id: Some("message-1".to_string()),
            ignore_get_message_error: Some(true),
        };

        assert_eq!(message.GetEventID(), "event-1");
        assert_eq!(message.GetSendType(), SendType::Text);
        assert_eq!(rich_media.GetEventID(), "");
        assert_eq!(rich_media.GetSendType(), SendType::RichMedia);
        assert_eq!(reference.GetEventID(), "message-1");
        assert_eq!(reference.GetSendType(), SendType::Text);

        let api_message = ApiMessage::from(message);
        assert_eq!(api_message.GetEventID(), "event-1");
        assert_eq!(api_message.GetSendType(), SendType::Text);
    }

    #[test]
    fn botgo_messages_pager_query_params() {
        let pager = MessagesPager::new(Some(MPTBefore), Some("msg-1"), Some(20));
        let query = pager.QueryParams();

        assert_eq!(query.get("before").map(String::as_str), Some("msg-1"));
        assert_eq!(query.get("limit").map(String::as_str), Some("20"));
    }

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

/// Botgo-compatible Ark message wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageArk {
    /// Ark payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
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

pub type ArkKV = ArkKv;

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

pub type ArkObjKV = ArkObjKv;

/// Embed message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

pub type MessageEmbedThumbnail = EmbedThumbnail;

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Keyboard {
    /// Keyboard template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Keyboard content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<KeyboardContent>,
}

pub type ActionType = u32;
pub type PermissionType = u32;

pub const ACTION_TYPE_URL: ActionType = 0;
pub const ACTION_TYPE_CALLBACK: ActionType = 1;
pub const ACTION_TYPE_AT_BOT: ActionType = 2;
pub const ACTION_TYPE_MQQ_API: ActionType = 3;
pub const ACTION_TYPE_SUBSCRIBE: ActionType = 4;
#[allow(non_upper_case_globals)]
pub const ActionTypeURL: ActionType = ACTION_TYPE_URL;
#[allow(non_upper_case_globals)]
pub const ActionTypeCallback: ActionType = ACTION_TYPE_CALLBACK;
#[allow(non_upper_case_globals)]
pub const ActionTypeAtBot: ActionType = ACTION_TYPE_AT_BOT;
#[allow(non_upper_case_globals)]
pub const ActionTypeMQQAPI: ActionType = ACTION_TYPE_MQQ_API;
#[allow(non_upper_case_globals)]
pub const ActionTypeSubscribe: ActionType = ACTION_TYPE_SUBSCRIBE;

pub const PERMISSION_TYPE_SPECIFY_USER_IDS: PermissionType = 0;
pub const PERMISSION_TYPE_MANAGER: PermissionType = 1;
pub const PERMISSION_TYPE_ALL: PermissionType = 2;
pub const PERMISSION_TYPE_SPECIFY_ROLE_IDS: PermissionType = 3;
#[allow(non_upper_case_globals)]
pub const PermissionTypeSpecifyUserIDs: PermissionType = PERMISSION_TYPE_SPECIFY_USER_IDS;
#[allow(non_upper_case_globals)]
pub const PermissionTypManager: PermissionType = PERMISSION_TYPE_MANAGER;
#[allow(non_upper_case_globals)]
pub const PermissionTypAll: PermissionType = PERMISSION_TYPE_ALL;
#[allow(non_upper_case_globals)]
pub const PermissionTypSpecifyRoleIDs: PermissionType = PERMISSION_TYPE_SPECIFY_ROLE_IDS;

pub type MessageKeyboard = Keyboard;
pub type CustomKeyboard = KeyboardContent;
pub type Row = KeyboardRow;
pub type Button = KeyboardButton;
pub type RenderData = KeyboardButtonRenderData;
pub type Action = KeyboardButtonAction;
pub type Permission = KeyboardButtonPermission;
pub type SubscribeData = KeyboardSubscribeData;
pub type TemplateID = KeyboardTemplateId;
pub type Modal = KeyboardModal;

/// Keyboard content structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardContent {
    /// Rows of buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<KeyboardRow>>,
    /// Keyboard style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<KeyboardStyle>,
}

/// Keyboard style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardStyle {
    /// Font size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
}

/// Keyboard row structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardRow {
    /// Buttons in this row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<KeyboardButton>>,
}

/// Keyboard button structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButton {
    /// Button ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Button render data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_data: Option<KeyboardButtonRenderData>,
    /// Button action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<KeyboardButtonAction>,
    /// Button group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

/// Keyboard button render data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButtonRenderData {
    /// Button label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Button visited label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visited_label: Option<String>,
    /// Button style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<u32>,
}

/// Keyboard button action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButtonAction {
    /// Action type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub action_type: Option<u32>,
    /// Permission data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<KeyboardButtonPermission>,
    /// Click limit per user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_limit: Option<u32>,
    /// Action data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Reply flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<bool>,
    /// Enter flag
    pub enter: Option<bool>,
    /// Whether to show channel selection when at-bot action is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_bot_show_channel_list: Option<bool>,
    /// Subscribe button data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_data: Option<KeyboardSubscribeData>,
    /// Secondary confirmation modal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modal: Option<KeyboardModal>,
}

/// Keyboard button permission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButtonPermission {
    /// Permission type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<u32>,
    /// Specify role IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specify_role_ids: Option<Vec<String>>,
    /// Specify user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specify_user_ids: Option<Vec<String>>,
}

/// Keyboard subscribe data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardSubscribeData {
    /// Subscription template IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_ids: Option<Vec<KeyboardTemplateId>>,
}

/// Keyboard template ID wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeyboardTemplateId {
    /// Official template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<u32>,
    /// Custom template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_template_id: Option<String>,
}

/// Keyboard secondary confirmation modal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeyboardModal {
    /// Confirmation content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Confirm button text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_text: Option<String>,
    /// Cancel button text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_text: Option<String>,
}

/// Keyboard payload structure for API requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardPayload {
    /// Keyboard content
    pub content: serde_json::Value,
}

pub type Markdown = MarkdownPayload;
pub type MarkdownParams = MarkdownParam;

/// Markdown message payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarkdownPayload {
    /// Template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i32>,
    /// Custom template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_template_id: Option<String>,
    /// Template parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<MarkdownParam>>,
    /// Markdown content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Markdown style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<MarkdownStyle>,
    /// Markdown guide message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_msg: Option<String>,
}

/// Markdown style.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MarkdownStyle {
    /// Body font size, for example small/middle/large.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_font_size: Option<String>,
    /// Layout, for example hide_avatar_and_center.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

/// Message send type used to select botgo-compatible routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum SendType {
    /// Regular text/message route
    Text = 1,
    /// Rich media file route
    RichMedia = 2,
    /// Unknown send type
    Unknown(u8),
}

impl From<u8> for SendType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Text,
            2 => Self::RichMedia,
            other => Self::Unknown(other),
        }
    }
}

impl From<SendType> for u8 {
    fn from(send_type: SendType) -> Self {
        match send_type {
            SendType::Text => 1,
            SendType::RichMedia => 2,
            SendType::Unknown(value) => value,
        }
    }
}

/// Message type used by the message create APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum MessageCreateType {
    /// Text message
    Text = 0,
    /// Markdown message
    Markdown = 2,
    /// Ark message
    Ark = 3,
    /// Embed message
    Embed = 4,
    /// Mention message
    At = 5,
    /// Input status notification
    InputNotify = 6,
    /// Rich media message
    RichMedia = 7,
    /// Unknown message type
    Unknown(u32),
}

#[allow(non_upper_case_globals)]
pub const TextMsg: MessageCreateType = MessageCreateType::Text;
#[allow(non_upper_case_globals)]
pub const MarkdownMsg: MessageCreateType = MessageCreateType::Markdown;
#[allow(non_upper_case_globals)]
pub const ArkMsg: MessageCreateType = MessageCreateType::Ark;
#[allow(non_upper_case_globals)]
pub const EmbedMsg: MessageCreateType = MessageCreateType::Embed;
#[allow(non_upper_case_globals)]
pub const ATMsg: MessageCreateType = MessageCreateType::At;
#[allow(non_upper_case_globals)]
pub const InputNotifyMsg: MessageCreateType = MessageCreateType::InputNotify;
#[allow(non_upper_case_globals)]
pub const RichMediaMsg: MessageCreateType = MessageCreateType::RichMedia;
#[allow(non_upper_case_globals)]
pub const RichMedia: SendType = SendType::RichMedia;

impl From<u32> for MessageCreateType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Text,
            2 => Self::Markdown,
            3 => Self::Ark,
            4 => Self::Embed,
            5 => Self::At,
            6 => Self::InputNotify,
            7 => Self::RichMedia,
            other => Self::Unknown(other),
        }
    }
}

impl From<MessageCreateType> for u32 {
    fn from(message_type: MessageCreateType) -> Self {
        match message_type {
            MessageCreateType::Text => 0,
            MessageCreateType::Markdown => 2,
            MessageCreateType::Ark => 3,
            MessageCreateType::Embed => 4,
            MessageCreateType::At => 5,
            MessageCreateType::InputNotify => 6,
            MessageCreateType::RichMedia => 7,
            MessageCreateType::Unknown(value) => value,
        }
    }
}

/// Input status notification payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InputNotify {
    /// Input status type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<i32>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_second: Option<i32>,
}

/// Rich media info used after uploading media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaInfo {
    /// Uploaded rich media file info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_info: Option<String>,
}

impl From<Media> for MediaInfo {
    fn from(media: Media) -> Self {
        Self {
            file_info: media.file_info,
        }
    }
}

impl From<MediaInfo> for Media {
    fn from(media: MediaInfo) -> Self {
        Self {
            file_info: media.file_info,
            ttl: None,
        }
    }
}

/// Streamed message fragment metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Stream {
    /// Stream state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    /// Stream ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Fragment index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Whether to reset an unfinished stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<bool>,
}

/// Prompt keyboard wrapper used by message extension areas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PromptKeyboard {
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
}

/// Message action button configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionButton {
    /// Action bar template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i32>,
    /// Callback payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Feedback button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<bool>,
    /// TTS button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    /// Regenerate button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub re_generate: Option<bool>,
    /// Stop generation button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_generate: Option<bool>,
}

/// Botgo-compatible channel/direct message create payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageToCreate {
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Message type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<MessageCreateType>,
    /// Message embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    /// Ark template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
    /// Image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Message ID to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    /// Message reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Reference>,
    /// Markdown payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownPayload>,
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
    /// Event ID to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Deprecated timestamp field kept for botgo parity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Message sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_seq: Option<u32>,
    /// Subscription ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<String>,
    /// Input status notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_notify: Option<InputNotify>,
    /// Rich media info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaInfo>,
    /// Prompt keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_keyboard: Option<PromptKeyboard>,
    /// Action button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<ActionButton>,
    /// Stream info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
    /// Feature control ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<u32>,
    /// Base64 encoded file image, supported by the legacy Rust API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_image: Option<String>,
}

impl MessageToCreate {
    /// Creates a text message payload.
    pub fn new_text(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// Sets file image data, automatically encoding to base64.
    pub fn with_file_image(mut self, data: &[u8]) -> Self {
        self.file_image = Some(base64::engine::general_purpose::STANDARD.encode(data));
        self
    }

    /// Sets the message ID to reply to.
    pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
        self.msg_id = Some(message_id.into());
        self
    }

    /// Returns the send type for route selection.
    pub const fn send_type(&self) -> SendType {
        SendType::Text
    }

    /// Botgo-compatible event ID accessor.
    #[allow(non_snake_case)]
    pub fn GetEventID(&self) -> &str {
        self.event_id.as_deref().unwrap_or("")
    }

    /// Botgo-compatible send type accessor.
    #[allow(non_snake_case)]
    pub const fn GetSendType(&self) -> SendType {
        self.send_type()
    }
}

/// Botgo-compatible rich media upload/direct-send payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RichMediaMessage {
    /// Deprecated event ID field, kept for API compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// File type, 1=image, 2=video, 3=audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<u64>,
    /// Rich media URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether the server should send the message directly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srv_send_msg: Option<bool>,
    /// Optional text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Message sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_seq: Option<i64>,
}

impl RichMediaMessage {
    /// Creates a rich media payload with file type and URL.
    pub fn new(file_type: u64, url: impl Into<String>) -> Self {
        Self {
            file_type: Some(file_type),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Returns the send type for route selection.
    pub const fn send_type(&self) -> SendType {
        SendType::RichMedia
    }

    /// Botgo-compatible event ID accessor. Botgo intentionally returns
    /// an empty value for rich media payloads.
    #[allow(non_snake_case)]
    pub const fn GetEventID(&self) -> &str {
        ""
    }

    /// Botgo-compatible send type accessor.
    #[allow(non_snake_case)]
    pub const fn GetSendType(&self) -> SendType {
        self.send_type()
    }
}

/// Botgo-compatible message interface.
pub trait APIMessage: Serialize {
    /// Returns the event ID used for passive replies.
    #[allow(non_snake_case)]
    fn GetEventID(&self) -> &str;

    /// Returns the route family for this message payload.
    #[allow(non_snake_case)]
    fn GetSendType(&self) -> SendType;
}

/// API message envelope used by group and C2C message APIs.
#[derive(Debug, Clone, PartialEq)]
pub enum ApiMessage {
    /// Regular message create payload
    Message(Box<MessageToCreate>),
    /// Rich media payload
    RichMedia(RichMediaMessage),
}

impl ApiMessage {
    /// Returns the send type for botgo-compatible route selection.
    pub const fn send_type(&self) -> SendType {
        match self {
            Self::Message(message) => message.send_type(),
            Self::RichMedia(message) => message.send_type(),
        }
    }

    /// Botgo-compatible event ID accessor.
    #[allow(non_snake_case)]
    pub fn GetEventID(&self) -> &str {
        match self {
            Self::Message(message) => message.GetEventID(),
            Self::RichMedia(message) => message.GetEventID(),
        }
    }

    /// Botgo-compatible send type accessor.
    #[allow(non_snake_case)]
    pub const fn GetSendType(&self) -> SendType {
        self.send_type()
    }
}

impl APIMessage for MessageToCreate {
    fn GetEventID(&self) -> &str {
        self.GetEventID()
    }

    fn GetSendType(&self) -> SendType {
        self.GetSendType()
    }
}

impl APIMessage for RichMediaMessage {
    fn GetEventID(&self) -> &str {
        self.GetEventID()
    }

    fn GetSendType(&self) -> SendType {
        self.GetSendType()
    }
}

impl APIMessage for ApiMessage {
    fn GetEventID(&self) -> &str {
        self.GetEventID()
    }

    fn GetSendType(&self) -> SendType {
        self.GetSendType()
    }
}

impl Serialize for ApiMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Message(message) => message.serialize(serializer),
            Self::RichMedia(message) => message.serialize(serializer),
        }
    }
}

impl From<MessageToCreate> for ApiMessage {
    fn from(message: MessageToCreate) -> Self {
        Self::Message(Box::new(message))
    }
}

impl From<RichMediaMessage> for ApiMessage {
    fn from(message: RichMediaMessage) -> Self {
        Self::RichMedia(message)
    }
}

impl APIMessage for Reference {
    fn GetEventID(&self) -> &str {
        self.message_id.as_deref().unwrap_or("")
    }

    fn GetSendType(&self) -> SendType {
        SendType::Text
    }
}

impl From<KeyboardPayload> for Keyboard {
    fn from(payload: KeyboardPayload) -> Self {
        serde_json::from_value(payload.content).unwrap_or_default()
    }
}

/// Message list pagination mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessagePagerType {
    /// Pull messages around the given message ID
    Around,
    /// Pull messages before the given message ID
    Before,
    /// Pull messages after the given message ID
    After,
}

#[allow(non_upper_case_globals)]
pub const MPTAround: MessagePagerType = MessagePagerType::Around;
#[allow(non_upper_case_globals)]
pub const MPTBefore: MessagePagerType = MessagePagerType::Before;
#[allow(non_upper_case_globals)]
pub const MPTAfter: MessagePagerType = MessagePagerType::After;

impl MessagePagerType {
    /// Returns the query parameter name for this pager type.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Around => "around",
            Self::Before => "before",
            Self::After => "after",
        }
    }
}

/// Pager for pulling channel messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessagesPager {
    /// Pull direction
    #[serde(skip)]
    pub pager_type: Option<MessagePagerType>,
    /// Message ID used by the pull direction
    #[serde(skip)]
    pub id: Option<String>,
    /// Page size, max 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl MessagesPager {
    /// Creates a new messages pager.
    pub fn new(
        pager_type: Option<MessagePagerType>,
        id: Option<impl Into<String>>,
        limit: Option<impl ToString>,
    ) -> Self {
        Self {
            pager_type,
            id: id.map(Into::into),
            limit: limit.map(|value| value.to_string()),
        }
    }

    /// Converts the pager to botgo-compatible query parameters.
    pub fn query_params(&self) -> std::collections::HashMap<String, String> {
        let mut query = std::collections::HashMap::new();
        if let Some(limit) = &self.limit {
            query.insert("limit".to_string(), limit.clone());
        }
        if let (Some(pager_type), Some(id)) = (self.pager_type, &self.id) {
            query.insert(pager_type.as_str().to_string(), id.clone());
        }
        query
    }

    /// Botgo-compatible query parameter accessor.
    #[allow(non_snake_case)]
    pub fn QueryParams(&self) -> std::collections::HashMap<String, String> {
        self.query_params()
    }
}

impl Pager for MessagesPager {
    fn query_params(&self) -> std::collections::HashMap<String, String> {
        MessagesPager::query_params(self)
    }
}

/// Setting guide payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingGuide {
    /// Guild ID for DM setting guide jumps
    pub guild_id: String,
}

/// Body for setting guide messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingGuideToCreate {
    /// Content used by channel setting guides, usually mentions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Setting guide jump target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_guide: Option<SettingGuide>,
}

/// Parameters for sending a message to a channel.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageParams {
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Message type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<MessageCreateType>,
    /// Message embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    /// Ark template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
    /// Message reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Reference>,
    /// Image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Base64 encoded file image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_image: Option<String>,
    /// Message ID to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    /// Event ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Markdown payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownPayload>,
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
    /// Deprecated timestamp field kept for botgo parity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Message sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_seq: Option<u32>,
    /// Subscription ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<String>,
    /// Input status notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_notify: Option<InputNotify>,
    /// Rich media info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaInfo>,
    /// Prompt keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_keyboard: Option<PromptKeyboard>,
    /// Action button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<ActionButton>,
    /// Stream info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
    /// Feature control ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<u32>,
}

/// Parameters for sending a group message.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupMessageParams {
    /// Message type (0=text, 1=rich text, 2=markdown, 3=ark, 4=embed, 7=media)
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub msg_type: u32,
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Message embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    /// Ark template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
    /// Message reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Reference>,
    /// Media attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    /// Message ID to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    /// Message sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_seq: Option<u32>,
    /// Event ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Markdown payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownPayload>,
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<KeyboardPayload>,
    /// Deprecated timestamp field kept for botgo parity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Subscription ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<String>,
    /// Input status notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_notify: Option<InputNotify>,
    /// Prompt keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_keyboard: Option<PromptKeyboard>,
    /// Action button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<ActionButton>,
    /// Stream info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
    /// Feature control ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<u32>,
}

/// Parameters for sending a C2C (client-to-client) message.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct C2CMessageParams {
    /// Message type (0=text, 1=rich text, 2=markdown, 3=ark, 4=embed, 7=media)
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub msg_type: u32,
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Message embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    /// Ark template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
    /// Message reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Reference>,
    /// Media attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    /// Message ID to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    /// Message sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_seq: Option<u32>,
    /// Event ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Markdown payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownPayload>,
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<KeyboardPayload>,
    /// Deprecated timestamp field kept for botgo parity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Subscription ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<String>,
    /// Input status notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_notify: Option<InputNotify>,
    /// Prompt keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_keyboard: Option<PromptKeyboard>,
    /// Action button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<ActionButton>,
    /// Stream info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
    /// Feature control ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<u32>,
}

/// Parameters for sending a direct message.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectMessageParams {
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Message type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<MessageCreateType>,
    /// Message embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    /// Ark template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
    /// Message reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Reference>,
    /// Image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Base64 encoded file image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_image: Option<String>,
    /// Message ID to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    /// Event ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Markdown payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownPayload>,
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
    /// Deprecated timestamp field kept for botgo parity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Message sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_seq: Option<u32>,
    /// Subscription ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<String>,
    /// Input status notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_notify: Option<InputNotify>,
    /// Rich media info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaInfo>,
    /// Prompt keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_keyboard: Option<PromptKeyboard>,
    /// Action button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<ActionButton>,
    /// Stream info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
    /// Feature control ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<u32>,
}

impl MessageParams {
    /// Creates a new MessageParams with text content.
    pub fn new_text(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// Sets file image data, automatically encoding to base64.
    pub fn with_file_image(mut self, data: &[u8]) -> Self {
        self.file_image = Some(base64::engine::general_purpose::STANDARD.encode(data));
        self
    }

    /// Sets the message reference for replying.
    pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
        self.msg_id = Some(message_id.into());
        self
    }

    /// Converts this payload into the botgo-compatible message create body.
    pub fn into_message_to_create(self) -> MessageToCreate {
        self.into()
    }
}

impl GroupMessageParams {
    /// Creates a new GroupMessageParams with text content.
    pub fn new_text(content: impl Into<String>) -> Self {
        Self {
            msg_type: 0,
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// Sets the message reference for replying.
    pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
        self.msg_id = Some(message_id.into());
        self
    }

    /// Converts this payload into the botgo-compatible message create body.
    pub fn into_message_to_create(self) -> MessageToCreate {
        self.into()
    }
}

impl C2CMessageParams {
    /// Creates a new C2CMessageParams with text content.
    pub fn new_text(content: impl Into<String>) -> Self {
        Self {
            msg_type: 0,
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// Sets the message reference for replying.
    pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
        self.msg_id = Some(message_id.into());
        self
    }

    /// Converts this payload into the botgo-compatible message create body.
    pub fn into_message_to_create(self) -> MessageToCreate {
        self.into()
    }
}

impl DirectMessageParams {
    /// Creates a new DirectMessageParams with text content.
    pub fn new_text(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// Sets file image data, automatically encoding to base64.
    pub fn with_file_image(mut self, data: &[u8]) -> Self {
        self.file_image = Some(base64::engine::general_purpose::STANDARD.encode(data));
        self
    }

    /// Sets the message reference for replying.
    pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
        self.msg_id = Some(message_id.into());
        self
    }

    /// Converts this payload into the botgo-compatible message create body.
    pub fn into_message_to_create(self) -> MessageToCreate {
        self.into()
    }
}

impl From<MessageParams> for MessageToCreate {
    fn from(params: MessageParams) -> Self {
        Self {
            content: params.content,
            msg_type: params.msg_type,
            embed: params.embed,
            ark: params.ark,
            image: params.image,
            msg_id: params.msg_id,
            message_reference: params.message_reference,
            markdown: params.markdown,
            keyboard: params.keyboard,
            event_id: params.event_id,
            timestamp: params.timestamp,
            msg_seq: params.msg_seq,
            subscribe_id: params.subscribe_id,
            input_notify: params.input_notify,
            media: params.media,
            prompt_keyboard: params.prompt_keyboard,
            action_button: params.action_button,
            stream: params.stream,
            feature_id: params.feature_id,
            file_image: params.file_image,
        }
    }
}

impl From<DirectMessageParams> for MessageToCreate {
    fn from(params: DirectMessageParams) -> Self {
        Self {
            content: params.content,
            msg_type: params.msg_type,
            embed: params.embed,
            ark: params.ark,
            image: params.image,
            msg_id: params.msg_id,
            message_reference: params.message_reference,
            markdown: params.markdown,
            keyboard: params.keyboard,
            event_id: params.event_id,
            timestamp: params.timestamp,
            msg_seq: params.msg_seq,
            subscribe_id: params.subscribe_id,
            input_notify: params.input_notify,
            media: params.media,
            prompt_keyboard: params.prompt_keyboard,
            action_button: params.action_button,
            stream: params.stream,
            feature_id: params.feature_id,
            file_image: params.file_image,
        }
    }
}

impl From<GroupMessageParams> for MessageToCreate {
    fn from(params: GroupMessageParams) -> Self {
        Self {
            content: params.content,
            msg_type: Some(MessageCreateType::from(params.msg_type)),
            embed: params.embed,
            ark: params.ark,
            msg_id: params.msg_id,
            message_reference: params.message_reference,
            markdown: params.markdown,
            keyboard: params.keyboard.map(Into::into),
            event_id: params.event_id,
            timestamp: params.timestamp,
            msg_seq: params.msg_seq,
            subscribe_id: params.subscribe_id,
            input_notify: params.input_notify,
            media: params.media.map(Into::into),
            prompt_keyboard: params.prompt_keyboard,
            action_button: params.action_button,
            stream: params.stream,
            feature_id: params.feature_id,
            ..Default::default()
        }
    }
}

impl From<C2CMessageParams> for MessageToCreate {
    fn from(params: C2CMessageParams) -> Self {
        Self {
            content: params.content,
            msg_type: Some(MessageCreateType::from(params.msg_type)),
            embed: params.embed,
            ark: params.ark,
            msg_id: params.msg_id,
            message_reference: params.message_reference,
            markdown: params.markdown,
            keyboard: params.keyboard.map(Into::into),
            event_id: params.event_id,
            timestamp: params.timestamp,
            msg_seq: params.msg_seq,
            subscribe_id: params.subscribe_id,
            input_notify: params.input_notify,
            media: params.media.map(Into::into),
            prompt_keyboard: params.prompt_keyboard,
            action_button: params.action_button,
            stream: params.stream,
            feature_id: params.feature_id,
            ..Default::default()
        }
    }
}
