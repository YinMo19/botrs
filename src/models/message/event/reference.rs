use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Reference to another message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageReference {
    /// The ID of the referenced message
    pub message_id: Snowflake,
    /// Whether reference message fetch errors should be ignored
    #[serde(default)]
    pub ignore_get_message_error: bool,
}
