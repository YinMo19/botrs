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

wire_enum!(RemindType, u8, Unknown, {
    None = 0,
    OnStart = 1,
    Before5Minutes = 2,
    Before15Minutes = 3,
    Before30Minutes = 4,
    Before1Hour = 5,
    Before2Hours = 6,
    Before1Day = 7,
    Before2Days = 8,
});

impl RemindType {
    /// Returns the wire value for this reminder type.
    pub fn to_wire_string(self) -> String {
        u8::from(self).to_string()
    }
}
