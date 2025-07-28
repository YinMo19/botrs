//! Guild-related data models for the QQ Guild Bot API.
//!
//! This module contains guild types that correspond to the Python botpy implementation.

use crate::models::{HasId, HasName, Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// Represents a guild (server) in the QQ Guild system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Guild {
    /// The guild's unique ID
    pub id: Option<Snowflake>,
    /// The guild's name
    pub name: Option<String>,
    /// The guild's icon hash
    pub icon: Option<String>,
    /// The ID of the guild owner
    pub owner_id: Option<Snowflake>,
    /// Whether the current user is the owner of this guild
    pub is_owner: Option<bool>,
    /// The number of members in this guild
    pub member_count: Option<u32>,
    /// The maximum number of members for this guild
    pub max_members: Option<u32>,
    /// The guild's description
    pub description: Option<String>,
    /// When the current user joined this guild
    pub joined_at: Option<Timestamp>,
}

impl Guild {
    /// Creates a new guild.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            icon: None,
            owner_id: None,
            is_owner: None,
            member_count: None,
            max_members: None,
            description: None,
            joined_at: None,
        }
    }

    /// Creates a new guild from API data.
    pub fn from_data(_api: crate::api::BotApi, id: String, data: serde_json::Value) -> Self {
        Self {
            id: Some(id),
            name: data.get("name").and_then(|v| v.as_str()).map(String::from),
            icon: data.get("icon").and_then(|v| v.as_str()).map(String::from),
            owner_id: data
                .get("owner_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            is_owner: data.get("is_owner").and_then(|v| v.as_bool()),
            member_count: data
                .get("member_count")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
            max_members: data
                .get("max_members")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
            description: data
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from),
            joined_at: data
                .get("joined_at")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
        }
    }

    /// Gets the guild's icon URL if it has one.
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            format!(
                "https://groupprofile.qq.com/groupicon/{}/{}",
                self.id.as_ref().unwrap_or(&String::new()),
                hash
            )
        })
    }

    /// Returns true if the current user owns this guild.
    pub fn is_owned_by_current_user(&self) -> bool {
        self.is_owner.unwrap_or(false)
    }

    /// Gets the guild's member count.
    pub fn get_member_count(&self) -> u32 {
        self.member_count.unwrap_or(0)
    }

    /// Gets the guild's maximum member count.
    pub fn get_max_members(&self) -> u32 {
        self.max_members.unwrap_or(0)
    }

    /// Returns true if the guild has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        match (self.member_count, self.max_members) {
            (Some(current), Some(max)) => current >= max,
            _ => false,
        }
    }

    /// Gets the guild's display name (same as name for guilds).
    pub fn display_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns true if the guild has a description.
    pub fn has_description(&self) -> bool {
        self.description
            .as_ref()
            .map_or(false, |desc| !desc.is_empty())
    }
}

impl Default for Guild {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Guild {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

impl HasName for Guild {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

/// Guild roles response wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildRoles {
    /// List of roles in the guild
    pub roles: Vec<GuildRole>,
    /// Number of roles
    pub role_num_limit: Option<String>,
}

impl GuildRoles {
    /// Creates a new guild roles wrapper.
    pub fn new(roles: Vec<GuildRole>) -> Self {
        Self {
            roles,
            role_num_limit: None,
        }
    }
}

/// Represents a role in a guild.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildRole {
    /// The role's unique ID
    pub id: Option<Snowflake>,
    /// The role's name
    pub name: Option<String>,
    /// The role's color (ARGB hex as decimal)
    pub color: Option<u32>,
    /// Whether this role is displayed separately in the member list
    pub hoist: Option<bool>,
    /// The number of members with this role
    pub number: Option<u32>,
    /// The number of online members with this role
    pub member_limit: Option<u32>,
}

impl GuildRole {
    /// Creates a new role.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            color: None,
            hoist: None,
            number: None,
            member_limit: None,
        }
    }

    /// Returns true if this role is hoisted (displayed separately).
    pub fn is_hoisted(&self) -> bool {
        self.hoist.unwrap_or(false)
    }

    /// Gets the role's color as a hex value.
    pub fn color_hex(&self) -> Option<String> {
        self.color.map(|c| format!("#{:06X}", c))
    }

    /// Gets the number of members with this role.
    pub fn member_count(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    /// Gets the member limit for this role.
    pub fn get_member_limit(&self) -> u32 {
        self.member_limit.unwrap_or(0)
    }

    /// Returns true if the role has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        match (self.number, self.member_limit) {
            (Some(current), Some(limit)) => current >= limit,
            _ => false,
        }
    }
}

impl Default for GuildRole {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for GuildRole {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

impl HasName for GuildRole {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

/// Represents a role in a guild (legacy type alias).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    /// The role's unique ID
    pub id: Option<Snowflake>,
    /// The role's name
    pub name: Option<String>,
    /// The role's color (ARGB hex as decimal)
    pub color: Option<u32>,
    /// Whether this role is displayed separately in the member list
    pub hoist: Option<bool>,
    /// The number of members with this role
    pub number: Option<u32>,
    /// The number of online members with this role
    pub member_limit: Option<u32>,
}

impl Role {
    /// Creates a new role.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            color: None,
            hoist: None,
            number: None,
            member_limit: None,
        }
    }

    /// Returns true if this role is hoisted (displayed separately).
    pub fn is_hoisted(&self) -> bool {
        self.hoist.unwrap_or(false)
    }

    /// Gets the role's color as a hex value.
    pub fn color_hex(&self) -> Option<String> {
        self.color.map(|c| format!("#{:06X}", c))
    }

    /// Gets the number of members with this role.
    pub fn member_count(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    /// Gets the member limit for this role.
    pub fn get_member_limit(&self) -> u32 {
        self.member_limit.unwrap_or(0)
    }

    /// Returns true if the role has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        match (self.number, self.member_limit) {
            (Some(current), Some(limit)) => current >= limit,
            _ => false,
        }
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Role {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

impl HasName for Role {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

// Type alias for backward compatibility
pub type Roles = Vec<Role>;

/// Represents a member of a guild.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    /// The user information
    pub user: Option<crate::models::User>,
    /// The member's nickname in the guild
    pub nick: Option<String>,
    /// The member's roles in the guild
    pub roles: Option<Vec<Snowflake>>,
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
}

impl Member {
    /// Creates a new member.
    pub fn new() -> Self {
        Self {
            user: None,
            nick: None,
            roles: None,
            joined_at: None,
        }
    }

    /// Gets the member's display name (nickname or username).
    pub fn display_name(&self) -> Option<&str> {
        self.nick
            .as_deref()
            .or_else(|| self.user.as_ref().map(|u| u.username.as_str()))
    }

    /// Gets the member's username.
    pub fn username(&self) -> Option<&str> {
        self.user.as_ref().map(|u| u.username.as_str())
    }

    /// Gets the member's user ID.
    pub fn user_id(&self) -> Option<&Snowflake> {
        self.user.as_ref().map(|u| &u.id)
    }

    /// Returns true if the member is a bot.
    pub fn is_bot(&self) -> bool {
        self.user.as_ref().map_or(false, |u| u.is_bot())
    }

    /// Gets the member's roles.
    pub fn role_ids(&self) -> &[Snowflake] {
        self.roles.as_deref().unwrap_or(&[])
    }

    /// Returns true if the member has a specific role.
    pub fn has_role(&self, role_id: &str) -> bool {
        self.role_ids().iter().any(|id| id == role_id)
    }
}

impl Default for Member {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Member {
    fn id(&self) -> Option<&Snowflake> {
        self.user_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guild_creation() {
        let guild = Guild::new();
        assert!(guild.id.is_none());
        assert!(guild.name.is_none());
        assert!(!guild.is_owned_by_current_user());
        assert_eq!(guild.get_member_count(), 0);
        assert_eq!(guild.get_max_members(), 0);
    }

    #[test]
    fn test_guild_with_data() {
        let mut guild = Guild::new();
        guild.id = Some("123456789".to_string());
        guild.name = Some("Test Guild".to_string());
        guild.is_owner = Some(true);
        guild.member_count = Some(100);
        guild.max_members = Some(500);
        guild.description = Some("A test guild".to_string());

        assert_eq!(guild.id(), Some(&"123456789".to_string()));
        assert_eq!(guild.name(), "Test Guild");
        assert!(guild.is_owned_by_current_user());
        assert_eq!(guild.get_member_count(), 100);
        assert_eq!(guild.get_max_members(), 500);
        assert!(!guild.is_at_member_limit());
        assert!(guild.has_description());
        assert_eq!(guild.display_name(), Some("Test Guild"));
    }

    #[test]
    fn test_member_limit() {
        let mut guild = Guild::new();
        guild.member_count = Some(500);
        guild.max_members = Some(500);
        assert!(guild.is_at_member_limit());

        guild.member_count = Some(499);
        assert!(!guild.is_at_member_limit());

        guild.member_count = Some(501);
        assert!(guild.is_at_member_limit());
    }

    #[test]
    fn test_icon_url() {
        let mut guild = Guild::new();
        assert!(guild.icon_url().is_none());

        guild.id = Some("123456789".to_string());
        guild.icon = Some("abc123".to_string());
        let url = guild.icon_url().unwrap();
        assert!(url.contains("123456789"));
        assert!(url.contains("abc123"));
    }

    #[test]
    fn test_role_creation() {
        let role = Role::new();
        assert!(role.id.is_none());
        assert!(role.name.is_none());
        assert!(!role.is_hoisted());
        assert_eq!(role.member_count(), 0);
    }

    #[test]
    fn test_role_with_data() {
        let mut role = Role::new();
        role.id = Some("role123".to_string());
        role.name = Some("Admin".to_string());
        role.color = Some(0xFF0000);
        role.hoist = Some(true);
        role.number = Some(5);
        role.member_limit = Some(10);

        assert_eq!(role.id(), Some(&"role123".to_string()));
        assert_eq!(role.name(), "Admin");
        assert_eq!(role.color_hex(), Some("#FF0000".to_string()));
        assert!(role.is_hoisted());
        assert_eq!(role.member_count(), 5);
        assert_eq!(role.get_member_limit(), 10);
        assert!(!role.is_at_member_limit());
    }

    #[test]
    fn test_member_creation() {
        let member = Member::new();
        assert!(member.user.is_none());
        assert!(member.nick.is_none());
        assert_eq!(member.role_ids().len(), 0);
    }

    #[test]
    fn test_member_with_roles() {
        let mut member = Member::new();
        member.roles = Some(vec!["role1".to_string(), "role2".to_string()]);

        assert!(member.has_role("role1"));
        assert!(member.has_role("role2"));
        assert!(!member.has_role("role3"));
        assert_eq!(member.role_ids().len(), 2);
    }
}
