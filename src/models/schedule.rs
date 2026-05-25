//! Schedule-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for creating and managing channel schedules
//! in QQ Guild bots.

use crate::models::{HasId, HasName, Member, Snowflake};
use serde::{Deserialize, Serialize};

/// Reminder types for schedule events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum RemindType {
    /// No reminder
    None = 0,
    /// Remind when event starts
    OnStart = 1,
    /// Remind 5 minutes before start
    Before5Minutes = 2,
    /// Remind 15 minutes before start
    Before15Minutes = 3,
    /// Remind 30 minutes before start
    Before30Minutes = 4,
    /// Remind 1 hour before start
    Before1Hour = 5,
    /// Remind 2 hours before start
    Before2Hours = 6,
    /// Remind 1 day before start
    Before1Day = 7,
    /// Remind 2 days before start
    Before2Days = 8,
    /// Unknown reminder type
    Unknown(u8),
}

impl From<u8> for RemindType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::OnStart,
            2 => Self::Before5Minutes,
            3 => Self::Before15Minutes,
            4 => Self::Before30Minutes,
            5 => Self::Before1Hour,
            6 => Self::Before2Hours,
            7 => Self::Before1Day,
            8 => Self::Before2Days,
            other => Self::Unknown(other),
        }
    }
}

impl From<RemindType> for u8 {
    fn from(remind_type: RemindType) -> Self {
        match remind_type {
            RemindType::None => 0,
            RemindType::OnStart => 1,
            RemindType::Before5Minutes => 2,
            RemindType::Before15Minutes => 3,
            RemindType::Before30Minutes => 4,
            RemindType::Before1Hour => 5,
            RemindType::Before2Hours => 6,
            RemindType::Before1Day => 7,
            RemindType::Before2Days => 8,
            RemindType::Unknown(value) => value,
        }
    }
}

impl RemindType {
    /// Returns the botgo wire value for this reminder type.
    pub fn to_botgo_string(self) -> String {
        u8::from(self).to_string()
    }

    /// Returns a human-readable description of the reminder type.
    pub fn description(&self) -> &'static str {
        match self {
            RemindType::None => "No reminder",
            RemindType::OnStart => "When event starts",
            RemindType::Before5Minutes => "5 minutes before",
            RemindType::Before15Minutes => "15 minutes before",
            RemindType::Before30Minutes => "30 minutes before",
            RemindType::Before1Hour => "1 hour before",
            RemindType::Before2Hours => "2 hours before",
            RemindType::Before1Day => "1 day before",
            RemindType::Before2Days => "2 days before",
            RemindType::Unknown(_) => "Unknown",
        }
    }

    /// Returns the minutes before the event when the reminder should be sent.
    /// Returns None for RemindType::None and RemindType::OnStart.
    pub fn minutes_before(&self) -> Option<u32> {
        match self {
            RemindType::None | RemindType::OnStart => None,
            RemindType::Before5Minutes => Some(5),
            RemindType::Before15Minutes => Some(15),
            RemindType::Before30Minutes => Some(30),
            RemindType::Before1Hour => Some(60),
            RemindType::Before2Hours => Some(120),
            RemindType::Before1Day => Some(24 * 60),
            RemindType::Before2Days => Some(48 * 60),
            RemindType::Unknown(_) => None,
        }
    }
}

impl std::fmt::Display for RemindType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

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
            remind_type: remind_type.to_botgo_string(),
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

/// Wrapper used by schedule create and update endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleWrapper {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}

impl ScheduleWrapper {
    /// Creates a new schedule wrapper.
    pub fn new(schedule: Schedule) -> Self {
        Self {
            schedule: Some(schedule),
        }
    }

    /// Creates an empty wrapper matching botgo's zero-value body.
    pub fn empty() -> Self {
        Self::default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remind_type_conversion() {
        assert_eq!(RemindType::from(0), RemindType::None);
        assert_eq!(RemindType::from(1), RemindType::OnStart);
        assert_eq!(RemindType::from(5), RemindType::Before1Hour);
        assert_eq!(u8::from(RemindType::Before15Minutes), 3);
        assert_eq!(u8::from(RemindType::Before1Day), 7);

        assert_eq!(RemindType::from(99), RemindType::Unknown(99));
        assert_eq!(u8::from(RemindType::Unknown(99)), 99);
    }

    #[test]
    fn test_remind_type_description() {
        assert_eq!(RemindType::None.description(), "No reminder");
        assert_eq!(
            RemindType::Before30Minutes.description(),
            "30 minutes before"
        );
        assert_eq!(RemindType::Before1Day.description(), "1 day before");
    }

    #[test]
    fn test_remind_type_minutes_before() {
        assert_eq!(RemindType::None.minutes_before(), None);
        assert_eq!(RemindType::OnStart.minutes_before(), None);
        assert_eq!(RemindType::Before5Minutes.minutes_before(), Some(5));
        assert_eq!(RemindType::Before1Hour.minutes_before(), Some(60));
        assert_eq!(RemindType::Before1Day.minutes_before(), Some(24 * 60));
    }

    #[test]
    fn test_schedule_creation() {
        let schedule = Schedule::new(
            "Team Meeting",
            "1640995200",
            "1640998800",
            Some("channel123".to_string()),
            RemindType::Before15Minutes,
        );

        assert_eq!(schedule.name, "Team Meeting");
        assert_eq!(schedule.start_timestamp, "1640995200");
        assert_eq!(schedule.end_timestamp, "1640998800");
        assert_eq!(schedule.jump_channel_id, "channel123");
        assert_eq!(schedule.remind_type, "3");
        assert!(schedule.has_reminder());
        assert!(schedule.has_jump_channel());
    }

    #[test]
    fn test_schedule_with_description() {
        let schedule = Schedule::new(
            "Daily Standup",
            "1640995200",
            "1640996400",
            None,
            RemindType::Before5Minutes,
        )
        .with_description("Daily team standup meeting");

        assert_eq!(
            schedule.description,
            "Daily team standup meeting".to_string()
        );
    }

    #[test]
    fn test_schedule_duration() {
        let schedule = Schedule::new(
            "Test Event",
            "1640995200", // Start
            "1640998800", // End (1 hour later)
            None,
            RemindType::None,
        );

        assert_eq!(schedule.duration_seconds(), Some(3600)); // 1 hour = 3600 seconds
    }

    #[test]
    fn test_schedule_no_reminder() {
        let schedule = Schedule::new(
            "No Reminder Event",
            "1640995200",
            "1640998800",
            None,
            RemindType::None,
        );

        assert!(!schedule.has_reminder());
        assert_eq!(schedule.reminder_description(), "No reminder");
    }

    #[test]
    fn test_schedule_display() {
        let schedule = Schedule::new(
            "Test Meeting",
            "1640995200",
            "1640998800",
            Some("channel456".to_string()),
            RemindType::Before30Minutes,
        );

        let display = format!("{}", schedule);
        assert!(display.contains("Test Meeting"));
        assert!(display.contains("1640995200"));
        assert!(display.contains("30 minutes before"));
    }

    #[test]
    fn test_schedule_timestamp_parsing() {
        let schedule = Schedule::new(
            "Parse Test",
            "1640995200",
            "invalid_timestamp",
            None,
            RemindType::None,
        );

        assert!(schedule.start_timestamp_parsed().is_ok());
        assert!(schedule.end_timestamp_parsed().is_err());
        assert_eq!(schedule.duration_seconds(), None);
    }

    #[test]
    fn botgo_schedule_uses_required_zero_value_fields() {
        let schedule: Schedule = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(schedule.id, "");
        assert_eq!(schedule.name, "");
        assert_eq!(schedule.description, "");
        assert_eq!(schedule.start_timestamp, "");
        assert_eq!(schedule.end_timestamp, "");
        assert_eq!(schedule.jump_channel_id, "");
        assert_eq!(schedule.remind_type, "");
        assert!(schedule.creator.is_none());
        assert!(!schedule.has_jump_channel());
        assert!(!schedule.has_reminder());

        let value = serde_json::to_value(&schedule).unwrap();
        assert!(value.as_object().unwrap().is_empty());
    }

    #[test]
    fn botgo_schedule_keeps_official_json_shape() {
        let schedule = Schedule {
            id: "schedule-1".to_string(),
            name: "meeting".to_string(),
            description: "planning".to_string(),
            start_timestamp: "1640995200".to_string(),
            end_timestamp: "1640998800".to_string(),
            jump_channel_id: "channel-1".to_string(),
            remind_type: "3".to_string(),
            creator: None,
        };
        let value = serde_json::to_value(&schedule).unwrap();

        assert_eq!(value["id"], "schedule-1");
        assert_eq!(value["name"], "meeting");
        assert_eq!(value["description"], "planning");
        assert_eq!(value["start_timestamp"], "1640995200");
        assert_eq!(value["end_timestamp"], "1640998800");
        assert_eq!(value["jump_channel_id"], "channel-1");
        assert_eq!(value["remind_type"], "3");
    }

    #[test]
    fn botgo_schedule_wrapper_allows_empty_zero_value_body() {
        let wrapper = ScheduleWrapper::empty();
        let value = serde_json::to_value(&wrapper).unwrap();

        assert!(value.as_object().unwrap().is_empty());
    }
}
