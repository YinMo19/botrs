//! Permission-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for managing API permissions and permission demands
//! in QQ Guild bots.

use crate::models::{HasId, Snowflake};
use serde::{Deserialize, Serialize};

/// Represents an API permission for a bot in a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIPermission {
    /// The API path/endpoint
    pub path: String,
    /// The HTTP method for this API
    pub method: String,
    /// Description of what this API does
    pub desc: Option<String>,
    /// Authorization status for this API
    /// 0: Unauthorized, 1: Authorized
    pub auth_status: Option<i32>,
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
            desc,
            auth_status,
        }
    }

    /// Returns true if this API is authorized for use.
    pub fn is_authorized(&self) -> bool {
        self.auth_status == Some(1)
    }

    /// Returns true if this API is unauthorized.
    pub fn is_unauthorized(&self) -> bool {
        self.auth_status == Some(0)
    }

    /// Gets the authorization status as a string.
    pub fn auth_status_string(&self) -> &'static str {
        match self.auth_status {
            Some(0) => "Unauthorized",
            Some(1) => "Authorized",
            _ => "Unknown",
        }
    }
}

/// Identifies a specific API for permission demand requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIPermissionDemandIdentify {
    /// The API path/endpoint
    pub path: String,
    /// The HTTP method for this API
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIPermissionDemand {
    /// The guild ID where permission is requested
    pub guild_id: Option<Snowflake>,
    /// The channel ID where the permission request will be sent
    pub channel_id: Option<Snowflake>,
    /// The API identifier for which permission is requested
    pub api_identify: APIPermissionDemandIdentify,
    /// The title of the permission request
    pub title: Option<String>,
    /// Description explaining why the permission is needed
    pub desc: String,
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
            guild_id: Some(guild_id.into()),
            channel_id: Some(channel_id.into()),
            api_identify,
            title: None,
            desc: desc.into(),
        }
    }

    /// Sets the title for this permission demand.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Gets the API path being requested.
    pub fn api_path(&self) -> &str {
        &self.api_identify.path
    }

    /// Gets the HTTP method being requested.
    pub fn api_method(&self) -> &str {
        &self.api_identify.method
    }
}

impl HasId for APIPermissionDemand {
    fn id(&self) -> Option<&Snowflake> {
        self.guild_id.as_ref()
    }
}

impl std::fmt::Display for APIPermissionDemand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PermissionDemand {{ guild_id: {:?}, api: {}, desc: {} }}",
            self.guild_id,
            self.api_identify,
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
        assert_eq!(permission.desc, Some("Get guild member".to_string()));
        assert_eq!(permission.auth_status, Some(1));
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
        let permission = APIPermission::new("/guilds/123/channels", "GET", None, None);

        assert!(!permission.is_authorized());
        assert!(!permission.is_unauthorized());
        assert_eq!(permission.auth_status_string(), "Unknown");
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

        assert_eq!(demand.guild_id, Some("guild123".to_string()));
        assert_eq!(demand.channel_id, Some("channel456".to_string()));
        assert_eq!(demand.api_path(), "/guilds/{guild_id}/members/{user_id}");
        assert_eq!(demand.api_method(), "GET");
        assert_eq!(demand.desc, "Need access to get guild member information");
        assert_eq!(demand.title, None);
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

        assert_eq!(demand.title, Some("Message Posting Permission".to_string()));
        assert_eq!(demand.id(), Some(&"guild123".to_string()));
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
