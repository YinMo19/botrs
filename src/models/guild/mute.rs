use crate::models::serde_helpers::is_empty_vec;
use serde::{Deserialize, Serialize};

/// Body for guild mute endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateGuildMute {
    /// Mute end timestamp in seconds
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mute_end_timestamp: String,
    /// Mute duration in seconds
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mute_seconds: String,
    /// User IDs for batch mute
    #[serde(default, skip_serializing_if = "is_empty_vec")]
    pub user_ids: Vec<String>,
}

impl UpdateGuildMute {
    /// Creates a mute request body.
    pub fn new(mute_end_timestamp: Option<&str>, mute_seconds: Option<&str>) -> Self {
        Self {
            mute_end_timestamp: mute_end_timestamp.unwrap_or_default().to_string(),
            mute_seconds: mute_seconds.unwrap_or_default().to_string(),
            user_ids: Vec::new(),
        }
    }

    /// Creates a batch mute request body.
    pub fn new_multi(
        user_ids: Vec<String>,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Self {
        Self {
            mute_end_timestamp: mute_end_timestamp.unwrap_or_default().to_string(),
            mute_seconds: mute_seconds.unwrap_or_default().to_string(),
            user_ids,
        }
    }

    /// Creates a request body that cancels mute.
    pub fn cancel() -> Self {
        Self {
            mute_end_timestamp: "0".to_string(),
            mute_seconds: "0".to_string(),
            user_ids: Vec::new(),
        }
    }

    /// Creates a request body that cancels mute for multiple users.
    pub fn cancel_multi(user_ids: Vec<String>) -> Self {
        Self {
            mute_end_timestamp: "0".to_string(),
            mute_seconds: "0".to_string(),
            user_ids,
        }
    }
}

/// Response for batch guild mute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateGuildMuteResponse {
    /// Successfully muted user IDs
    #[serde(default)]
    pub user_ids: Vec<String>,
}
