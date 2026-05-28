use super::{MessageAttachment, MessageMember, MessageReference, MessageScene, MessageUser};
use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

use crate::models::message::{Ark, Embed};

/// Represents a message in a guild channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(skip)]
    pub event_id: Option<String>,
}
