use serde::{Deserialize, Serialize};

/// Identifies a specific API for permission demand requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissionDemandIdentify {
    /// The API path/endpoint
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// The HTTP method for this API
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub method: String,
}

impl APIPermissionDemandIdentify {
    /// Creates an API permission identifier from method and path.
    pub fn new(path: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            method: method.into(),
        }
    }

    /// Creates an identifier for the guild members API.
    pub fn guild_members() -> Self {
        Self::new("/guilds/{guild_id}/members/{user_id}", "GET")
    }

    /// Creates an identifier for the guild channels API.
    pub fn guild_channels() -> Self {
        Self::new("/guilds/{guild_id}/channels", "GET")
    }

    /// Creates an identifier for posting messages API.
    pub fn post_messages() -> Self {
        Self::new("/channels/{channel_id}/messages", "POST")
    }

    /// Creates an identifier for managing guild roles API.
    pub fn guild_roles() -> Self {
        Self::new("/guilds/{guild_id}/roles", "POST")
    }
}

impl std::fmt::Display for APIPermissionDemandIdentify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.method, self.path)
    }
}
