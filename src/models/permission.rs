//! Permission-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for managing API permissions and permission demands
//! in QQ Guild bots.

use crate::models::{HasId, Snowflake};
use serde::{Deserialize, Serialize};

/// Botgo-compatible API permissions response wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissions {
    /// API permission list.
    #[serde(default, rename = "apis")]
    pub api_list: Vec<APIPermission>,
}

/// Represents an API permission for a bot in a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermission {
    /// The API path/endpoint
    #[serde(default)]
    pub path: String,
    /// The HTTP method for this API
    #[serde(default)]
    pub method: String,
    /// Description of what this API does
    #[serde(default)]
    pub desc: String,
    /// Authorization status for this API
    /// 0: Unauthorized, 1: Authorized
    #[serde(default)]
    pub auth_status: i32,
}

impl APIPermission {
    /// Creates a new APIPermission instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `method` - The HTTP method (GET, POST, etc.)
    /// * `desc` - Optional description of the API
    /// * `auth_status` - Authorization status (0 = unauthorized, 1 = authorized)
    pub fn new(
        path: impl Into<String>,
        method: impl Into<String>,
        desc: Option<String>,
        auth_status: Option<i32>,
    ) -> Self {
        Self {
            path: path.into(),
            method: method.into(),
            desc: desc.unwrap_or_default(),
            auth_status: auth_status.unwrap_or_default(),
        }
    }

    /// Returns true if this API is authorized for use.
    pub fn is_authorized(&self) -> bool {
        self.auth_status == 1
    }

    /// Returns true if this API is unauthorized.
    pub fn is_unauthorized(&self) -> bool {
        self.auth_status == 0
    }

    /// Gets the authorization status as a string.
    pub fn auth_status_string(&self) -> &'static str {
        match self.auth_status {
            0 => "Unauthorized",
            1 => "Authorized",
            _ => "Unknown",
        }
    }
}

/// Identifies a specific API for permission demand requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissionDemandIdentify {
    /// The API path/endpoint
    #[serde(default)]
    pub path: String,
    /// The HTTP method for this API
    #[serde(default)]
    pub method: String,
}

impl APIPermissionDemandIdentify {
    /// Creates a new APIPermissionDemandIdentify instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `method` - The HTTP method (GET, POST, etc.)
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

/// Represents a permission demand request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissionDemand {
    /// The guild ID where permission is requested
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The channel ID where the permission request will be sent
    #[serde(default)]
    pub channel_id: Snowflake,
    /// The API identifier for which permission is requested
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_identify: Option<APIPermissionDemandIdentify>,
    /// The title of the permission request
    #[serde(default)]
    pub title: String,
    /// Description explaining why the permission is needed
    #[serde(default)]
    pub desc: String,
}

/// Botgo-compatible body for creating an API permission demand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissionDemandToCreate {
    /// The channel ID where the permission request will be sent
    #[serde(default)]
    pub channel_id: Snowflake,
    /// The API identifier for which permission is requested
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_identify: Option<APIPermissionDemandIdentify>,
    /// Description explaining why the permission is needed
    #[serde(default)]
    pub desc: String,
}

impl APIPermissionDemandToCreate {
    /// Creates a new botgo-compatible permission demand creation body.
    pub fn new(
        channel_id: impl Into<String>,
        api_identify: APIPermissionDemandIdentify,
        desc: impl Into<String>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            api_identify: Some(api_identify),
            desc: desc.into(),
        }
    }
}

impl APIPermissionDemand {
    /// Creates a new APIPermissionDemand instance.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID where permission is requested
    /// * `channel_id` - The channel ID where the request will be sent
    /// * `api_identify` - The API identifier for which permission is requested
    /// * `desc` - Description explaining why the permission is needed
    pub fn new(
        guild_id: impl Into<String>,
        channel_id: impl Into<String>,
        api_identify: APIPermissionDemandIdentify,
        desc: impl Into<String>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            api_identify: Some(api_identify),
            title: String::new(),
            desc: desc.into(),
        }
    }

    /// Sets the title for this permission demand.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Gets the API path being requested.
    pub fn api_path(&self) -> &str {
        self.api_identify
            .as_ref()
            .map(|identify| identify.path.as_str())
            .unwrap_or_default()
    }

    /// Gets the HTTP method being requested.
    pub fn api_method(&self) -> &str {
        self.api_identify
            .as_ref()
            .map(|identify| identify.method.as_str())
            .unwrap_or_default()
    }
}

impl HasId for APIPermissionDemand {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.guild_id)
    }
}

impl std::fmt::Display for APIPermissionDemand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PermissionDemand {{ guild_id: {}, api: {}, desc: {} }}",
            self.guild_id,
            self.api_identify
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
            self.desc.chars().take(50).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_permission() {
        let permission = APIPermission::new(
            "/guilds/123/members/456",
            "GET",
            Some("Get guild member".to_string()),
            Some(1),
        );

        assert_eq!(permission.path, "/guilds/123/members/456");
        assert_eq!(permission.method, "GET");
        assert_eq!(permission.desc, "Get guild member");
        assert_eq!(permission.auth_status, 1);
        assert!(permission.is_authorized());
        assert!(!permission.is_unauthorized());
        assert_eq!(permission.auth_status_string(), "Authorized");
    }

    #[test]
    fn test_api_permission_unauthorized() {
        let permission = APIPermission::new(
            "/guilds/123/roles",
            "POST",
            Some("Create guild role".to_string()),
            Some(0),
        );

        assert!(!permission.is_authorized());
        assert!(permission.is_unauthorized());
        assert_eq!(permission.auth_status_string(), "Unauthorized");
    }

    #[test]
    fn test_api_permission_unknown_status() {
        let permission = APIPermission::new("/guilds/123/channels", "GET", None, Some(2));

        assert!(!permission.is_authorized());
        assert!(!permission.is_unauthorized());
        assert_eq!(permission.auth_status_string(), "Unknown");
    }

    #[test]
    fn botgo_api_permission_uses_required_zero_value_fields() {
        let permission: APIPermission = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(permission.path, "");
        assert_eq!(permission.method, "");
        assert_eq!(permission.desc, "");
        assert_eq!(permission.auth_status, 0);
    }

    #[test]
    fn botgo_api_permissions_keep_official_json_shape() {
        let permissions = APIPermissions {
            api_list: vec![APIPermission::new(
                "/guilds/{guild_id}/members/{user_id}",
                "GET",
                Some("Get member".to_string()),
                Some(1),
            )],
        };
        let value = serde_json::to_value(&permissions).unwrap();

        assert!(value.get("api_list").is_none());
        assert_eq!(
            value["apis"][0]["path"],
            "/guilds/{guild_id}/members/{user_id}"
        );
        assert_eq!(value["apis"][0]["method"], "GET");
        assert_eq!(value["apis"][0]["desc"], "Get member");
        assert_eq!(value["apis"][0]["auth_status"], 1);
    }

    #[test]
    fn test_api_permission_demand_identify() {
        let identify = APIPermissionDemandIdentify::new("/guilds/{guild_id}/members", "GET");
        assert_eq!(identify.path, "/guilds/{guild_id}/members");
        assert_eq!(identify.method, "GET");
        assert_eq!(format!("{}", identify), "GET /guilds/{guild_id}/members");
    }

    #[test]
    fn test_api_permission_demand_identify_presets() {
        let guild_members = APIPermissionDemandIdentify::guild_members();
        assert_eq!(guild_members.path, "/guilds/{guild_id}/members/{user_id}");
        assert_eq!(guild_members.method, "GET");

        let post_messages = APIPermissionDemandIdentify::post_messages();
        assert_eq!(post_messages.path, "/channels/{channel_id}/messages");
        assert_eq!(post_messages.method, "POST");
    }

    #[test]
    fn test_api_permission_demand() {
        let identify = APIPermissionDemandIdentify::guild_members();
        let demand = APIPermissionDemand::new(
            "guild123",
            "channel456",
            identify,
            "Need access to get guild member information",
        );

        assert_eq!(demand.guild_id, "guild123");
        assert_eq!(demand.channel_id, "channel456");
        assert_eq!(demand.api_path(), "/guilds/{guild_id}/members/{user_id}");
        assert_eq!(demand.api_method(), "GET");
        assert_eq!(demand.desc, "Need access to get guild member information");
        assert_eq!(demand.title, "");
    }

    #[test]
    fn test_api_permission_demand_with_title() {
        let identify = APIPermissionDemandIdentify::post_messages();
        let demand = APIPermissionDemand::new(
            "guild123",
            "channel456",
            identify,
            "Need to send automated messages",
        )
        .with_title("Message Posting Permission");

        assert_eq!(demand.title, "Message Posting Permission");
        assert_eq!(demand.id(), Some(&"guild123".to_string()));
    }

    #[test]
    fn botgo_api_permission_demand_uses_zero_values() {
        let demand: APIPermissionDemand = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(demand.guild_id, "");
        assert_eq!(demand.channel_id, "");
        assert!(demand.api_identify.is_none());
        assert_eq!(demand.title, "");
        assert_eq!(demand.desc, "");
        assert_eq!(demand.api_path(), "");
        assert_eq!(demand.api_method(), "");
    }

    #[test]
    fn botgo_api_permission_demand_to_create_keeps_official_json_shape() {
        let demand = APIPermissionDemandToCreate::new(
            "channel-1",
            APIPermissionDemandIdentify::post_messages(),
            "Need to send messages",
        );
        let value = serde_json::to_value(&demand).unwrap();

        assert_eq!(value["channel_id"], "channel-1");
        assert_eq!(
            value["api_identify"]["path"],
            "/channels/{channel_id}/messages"
        );
        assert_eq!(value["api_identify"]["method"], "POST");
        assert_eq!(value["desc"], "Need to send messages");
    }

    #[test]
    fn test_api_permission_demand_display() {
        let identify = APIPermissionDemandIdentify::guild_channels();
        let demand = APIPermissionDemand::new(
            "guild999",
            "channel888",
            identify,
            "This is a very long description that should be truncated when displayed",
        );

        let display = format!("{}", demand);
        assert!(display.contains("guild999"));
        assert!(display.contains("GET /guilds/{guild_id}/channels"));
        // Should be truncated to 50 characters
        assert!(display.len() < 200);
    }
}
