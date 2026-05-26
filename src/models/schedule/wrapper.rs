use super::Schedule;
use serde::{Deserialize, Serialize};

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

    /// Creates an empty wrapper with zero-value defaults.
    pub fn empty() -> Self {
        Self::default()
    }
}
