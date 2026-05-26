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
    /// Returns the wire value for this reminder type.
    pub fn to_wire_string(self) -> String {
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
