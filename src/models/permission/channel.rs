use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

/// Member permission on a channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChannelPermissions {
    #[serde(default)]
    pub channel_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub permissions: String,
}

/// Role permission on a channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChannelRolePermissions {
    #[serde(default)]
    pub channel_id: String,
    #[serde(default)]
    pub role_id: String,
    #[serde(default)]
    pub permissions: String,
}

/// Permission update body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateChannelPermissions {
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub add: Option<String>,
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub remove: Option<String>,
}
