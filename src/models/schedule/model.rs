use super::RemindType;
use crate::models::{Snowflake, user::Member as UserMember};
use serde::{Deserialize, Serialize};

/// Represents a schedule event in a channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    /// Unique identifier for the schedule
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: Snowflake,
    /// Name of the schedule event
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Description of the schedule event
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Start timestamp (Unix timestamp as string)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub start_timestamp: String,
    /// End timestamp (Unix timestamp as string)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub end_timestamp: String,
    /// Channel ID to jump to when the event starts
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub jump_channel_id: Snowflake,
    /// Reminder type for the schedule
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub remind_type: String,
    /// Creator of the schedule
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserMember>,
}

impl Schedule {
    /// Creates a schedule with Unix timestamp strings and a reminder type.
    pub fn new(
        name: impl Into<String>,
        start_timestamp: impl Into<String>,
        end_timestamp: impl Into<String>,
        jump_channel_id: Option<String>,
        remind_type: RemindType,
    ) -> Self {
        Self {
            id: String::new(),
            name: name.into(),
            description: String::new(),
            start_timestamp: start_timestamp.into(),
            end_timestamp: end_timestamp.into(),
            jump_channel_id: jump_channel_id.unwrap_or_default(),
            remind_type: remind_type.to_wire_string(),
            creator: None,
        }
    }
}
