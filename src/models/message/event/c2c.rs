use super::{C2CMessageUser, MessageAttachment, MessageReference, MessageScene};
use crate::models::Timestamp;
use serde::{Deserialize, Serialize};

/// Represents a C2C (client-to-client) message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct C2CMessage {
    /// The message's unique ID
    pub id: String,
    /// The message content
    pub content: String,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Users mentioned in this message
    #[serde(default)]
    pub mentions: Vec<C2CMessageUser>,
    /// Attachments in this message
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    /// Global message sequence number
    pub msg_seq: Option<u64>,
    /// When this message was sent
    pub timestamp: Timestamp,
    /// The author of this message
    pub author: C2CMessageUser,
    /// Message scene information
    #[serde(default)]
    pub message_scene: MessageScene,
    /// Open-message type code.
    pub message_type: u32,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}
