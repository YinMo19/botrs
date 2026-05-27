use super::Schedule;
use serde::{Deserialize, Serialize};

/// Wrapper used by schedule create and update endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleWrapper {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}
