use super::RemindType;
use crate::models::{HasId, HasName, Member, Snowflake};
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
    pub creator: Option<Member>,
}

impl Schedule {
    /// Creates a new Schedule instance.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the schedule event
    /// * `start_timestamp` - Start time as Unix timestamp string
    /// * `end_timestamp` - End time as Unix timestamp string
    /// * `jump_channel_id` - Optional channel ID to jump to
    /// * `remind_type` - Type of reminder to set
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

    /// Sets the description for this schedule.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the creator for this schedule.
    pub fn with_creator(mut self, creator: Member) -> Self {
        self.creator = Some(creator);
        self
    }

    /// Sets the ID for this schedule.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Returns true if the schedule has a reminder set.
    pub fn has_reminder(&self) -> bool {
        !self.remind_type.is_empty() && self.remind_type != "0"
    }

    /// Gets the reminder description.
    pub fn reminder_description(&self) -> &'static str {
        self.remind_type
            .parse::<u8>()
            .map(RemindType::from)
            .map(|remind_type| remind_type.description())
            .unwrap_or("No reminder")
    }

    /// Attempts to parse the start timestamp as a Unix timestamp.
    pub fn start_timestamp_parsed(&self) -> Result<i64, std::num::ParseIntError> {
        self.start_timestamp.parse::<i64>()
    }

    /// Attempts to parse the end timestamp as a Unix timestamp.
    pub fn end_timestamp_parsed(&self) -> Result<i64, std::num::ParseIntError> {
        self.end_timestamp.parse::<i64>()
    }

    /// Returns the duration of the event in seconds, if timestamps can be parsed.
    pub fn duration_seconds(&self) -> Option<i64> {
        let start = self.start_timestamp_parsed().ok()?;
        let end = self.end_timestamp_parsed().ok()?;
        Some(end - start)
    }

    /// Returns true if this schedule has a jump channel set.
    pub fn has_jump_channel(&self) -> bool {
        !self.jump_channel_id.is_empty()
    }
}

impl HasId for Schedule {
    fn id(&self) -> Option<&Snowflake> {
        (!self.id.is_empty()).then_some(&self.id)
    }
}

impl HasName for Schedule {
    fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Schedule {{ id: {:?}, name: {}, start: {}, end: {}, reminder: {} }}",
            self.id(),
            self.name,
            self.start_timestamp,
            self.end_timestamp,
            self.reminder_description()
        )
    }
}
