use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

/// Guild/member mute update body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildMute {
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub mute_end_timestamp: Option<String>,
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub mute_seconds: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_ids: Vec<String>,
}

/// Batch member mute response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildMuteResponse {
    #[serde(default)]
    pub user_ids: Vec<String>,
}
