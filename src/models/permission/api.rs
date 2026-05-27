use crate::models::serde_helpers::is_zero_i32;
use serde::{Deserialize, Serialize};

/// API permissions response wrapper as returned by the QQ Bot Open API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissions {
    /// API permission list.
    #[serde(default, rename = "apis", skip_serializing_if = "Vec::is_empty")]
    pub api_list: Vec<APIPermission>,
}

/// Represents an API permission for a bot in a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermission {
    /// The API path/endpoint
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// The HTTP method for this API
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub method: String,
    /// Description of what this API does
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,
    /// Authorization status for this API
    /// 0: Unauthorized, 1: Authorized
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub auth_status: i32,
}

impl APIPermission {
    /// Gets the authorization status as a string.
    pub fn auth_status_string(&self) -> &'static str {
        match self.auth_status {
            0 => "Unauthorized",
            1 => "Authorized",
            _ => "Unknown",
        }
    }
}
