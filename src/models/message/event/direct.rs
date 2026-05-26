use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

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
