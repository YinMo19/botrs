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
