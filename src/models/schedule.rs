//! Schedule-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for creating and managing channel schedules
//! in QQ Guild bots.

use crate::models::{HasId, HasName, Snowflake};
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

/// Represents a creator of a schedule (Member information).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleCreator {
    /// User ID of the creator
    pub user_id: Option<Snowflake>,
    /// Nickname of the creator
    pub nick: Option<String>,
    /// Username of the creator
    pub username: Option<String>,
    /// Avatar URL of the creator
    pub avatar: Option<String>,
}

impl ScheduleCreator {
    /// Creates a new ScheduleCreator instance.
    pub fn new(
        user_id: Option<String>,
        nick: Option<String>,
        username: Option<String>,
        avatar: Option<String>,
    ) -> Self {
        Self {
            user_id,
            nick,
            username,
            avatar,
        }
    }
}

impl HasId for ScheduleCreator {
    fn id(&self) -> Option<&Snowflake> {
        self.user_id.as_ref()
    }
}

impl HasName for ScheduleCreator {
    fn name(&self) -> &str {
        self.nick
            .as_deref()
            .or(self.username.as_deref())
            .unwrap_or("Unknown")
    }
}

/// Represents a schedule event in a channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schedule {
    /// Unique identifier for the schedule
    pub id: Option<Snowflake>,
    /// Name of the schedule event
    pub name: String,
    /// Description of the schedule event
    pub description: Option<String>,
    /// Start timestamp (Unix timestamp as string)
    pub start_timestamp: String,
    /// End timestamp (Unix timestamp as string)
    pub end_timestamp: String,
    /// Creator of the schedule
    pub creator: Option<ScheduleCreator>,
    /// Channel ID to jump to when the event starts
    pub jump_channel_id: Option<Snowflake>,
    /// Reminder type for the schedule
    pub remind_type: Option<RemindType>,
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
            id: None,
            name: name.into(),
            description: None,
            start_timestamp: start_timestamp.into(),
            end_timestamp: end_timestamp.into(),
            creator: None,
            jump_channel_id,
            remind_type: Some(remind_type),
        }
    }

    /// Sets the description for this schedule.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the creator for this schedule.
    pub fn with_creator(mut self, creator: ScheduleCreator) -> Self {
        self.creator = Some(creator);
        self
    }

    /// Sets the ID for this schedule.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Returns true if the schedule has a reminder set.
    pub fn has_reminder(&self) -> bool {
        !matches!(self.remind_type, Some(RemindType::None) | None)
    }

    /// Gets the reminder description.
    pub fn reminder_description(&self) -> &'static str {
        self.remind_type
            .as_ref()
            .map(|r| r.description())
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
        self.jump_channel_id.is_some()
    }
}

impl HasId for Schedule {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
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
            self.id,
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
    fn test_schedule_creator() {
        let creator = ScheduleCreator::new(
            Some("user123".to_string()),
            Some("TestUser".to_string()),
            Some("testuser".to_string()),
            Some("https://example.com/avatar.png".to_string()),
        );

        assert_eq!(creator.id(), Some(&"user123".to_string()));
        assert_eq!(creator.name(), "TestUser");
    }

    #[test]
    fn test_schedule_creator_fallback_name() {
        let creator = ScheduleCreator::new(
            Some("user123".to_string()),
            None,
            Some("testuser".to_string()),
            None,
        );

        assert_eq!(creator.name(), "testuser");

        let creator_no_name = ScheduleCreator::new(None, None, None, None);
        assert_eq!(creator_no_name.name(), "Unknown");
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
        assert_eq!(schedule.jump_channel_id, Some("channel123".to_string()));
        assert_eq!(schedule.remind_type, Some(RemindType::Before15Minutes));
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
            Some("Daily team standup meeting".to_string())
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
}
