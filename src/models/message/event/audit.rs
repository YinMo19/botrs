use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

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
    pub fn from_data(event_id: String, data: serde_json::Value) -> Self {
        let mut audit: Self = serde_json::from_value(data).unwrap_or_default();
        audit.event_id = Some(event_id);
        audit
    }
}
