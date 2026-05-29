use super::{GroupMessageUser, MessageAttachment, MessageReference, MessageScene};
use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// Represents a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupMessage {
    /// The message's unique ID
    pub id: Snowflake,
    /// The message content
    pub content: String,
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
    pub timestamp: Timestamp,
    /// The author of this message
    pub author: GroupMessageUser,
    /// Group ID
    pub group_id: Snowflake,
    /// Group OpenID
    pub group_openid: String,
    /// Message scene information
    pub message_scene: MessageScene,
    /// Open-message type code.
    pub message_type: u32,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}
