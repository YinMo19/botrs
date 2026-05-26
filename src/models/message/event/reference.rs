use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

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
