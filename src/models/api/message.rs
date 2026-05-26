use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Response from message sending operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageResponse {
    /// The ID of the sent message
    pub id: Option<Snowflake>,
    /// The timestamp when the message was sent
    pub timestamp: Option<Timestamp>,
    /// Additional response data
    #[serde(default, flatten, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Pinned messages response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PinsMessage {
    /// Guild ID
    #[serde(default)]
    pub guild_id: Snowflake,
    /// Channel ID
    #[serde(default)]
    pub channel_id: Snowflake,
    /// Pinned message IDs
    #[serde(default)]
    pub message_ids: Vec<Snowflake>,
}

impl MessageResponse {
    /// Creates a new message response
    pub fn new(id: impl Into<Snowflake>) -> Self {
        Self {
            id: Some(id.into()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            extra: BTreeMap::new(),
        }
    }
}
