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
    /// Creates an identifier for the guild members API.
    pub fn guild_members() -> Self {
        Self {
            path: "/guilds/{guild_id}/members/{user_id}".to_string(),
            method: "GET".to_string(),
        }
    }

    /// Creates an identifier for the guild channels API.
    pub fn guild_channels() -> Self {
        Self {
            path: "/guilds/{guild_id}/channels".to_string(),
            method: "GET".to_string(),
        }
    }

    /// Creates an identifier for posting messages API.
    pub fn post_messages() -> Self {
        Self {
            path: "/channels/{channel_id}/messages".to_string(),
            method: "POST".to_string(),
        }
    }

    /// Creates an identifier for managing guild roles API.
    pub fn guild_roles() -> Self {
        Self {
            path: "/guilds/{guild_id}/roles".to_string(),
            method: "POST".to_string(),
        }
    }
}

impl std::fmt::Display for APIPermissionDemandIdentify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.method, self.path)
    }
}
