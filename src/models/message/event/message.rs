use super::{MessageAttachment, MessageMember, MessageReference, MessageScene, MessageUser};
use crate::models::{HasId, Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

use crate::models::message::{Ark, Embed, MessageParams};

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
    #[serde(skip)]
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
    pub fn from_data(event_id: String, data: serde_json::Value) -> Self {
        let mut message: Self = serde_json::from_value(data).unwrap_or_default();
        message.event_id = Some(event_id);
        message
    }

    /// Reply to this message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(channel_id), Some(msg_id)) = (&self.channel_id, &self.id) {
            let params = MessageParams {
                content: Some(content.to_string()),
                msg_id: Some(msg_id.clone()),
                event_id: self.event_id.clone(),
                ..Default::default()
            };
            api.send_message(channel_id, params).await
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
