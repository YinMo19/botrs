use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// Represents a message audit event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageAudit {
    /// The audit ID
    pub audit_id: Snowflake,
    /// The message ID that was audited
    pub message_id: Snowflake,
    /// The guild ID where the message was posted
    pub guild_id: Snowflake,
    /// The channel ID where the message was posted
    pub channel_id: Snowflake,
    /// The audit time
    pub audit_time: Timestamp,
    /// The create time
    pub create_time: Timestamp,
    /// Channel-specific sequence number for ordering audited messages
    pub seq_in_channel: String,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}
