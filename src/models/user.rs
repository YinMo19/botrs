//! User-related data models for the QQ Guild Bot API.

use crate::models::{HasId, Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// Represents a user in the QQ Guild system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct User {
    /// The user's unique ID
    pub id: Snowflake,
    /// The user's username
    pub username: String,
    /// The user's avatar hash
    pub avatar: Option<String>,
    /// Whether the user is a bot
    #[serde(default)]
    pub bot: bool,
    /// The user's union openid (for group/C2C messages)
    pub union_openid: Option<String>,
    /// The user's union user account
    pub union_user_account: Option<String>,
}

impl User {
    /// Creates a new user.
    pub fn new(id: impl Into<Snowflake>, username: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            avatar: None,
            bot: false,
            union_openid: None,
            union_user_account: None,
        }
    }

    /// Creates a new user from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        Self {
            id: data
                .get("id")
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_default(),
            username: data
                .get("username")
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_default(),
            avatar: data
                .get("avatar")
                .and_then(|v| v.as_str())
                .map(String::from),
            bot: data.get("bot").and_then(|v| v.as_bool()).unwrap_or(false),
            union_openid: data
                .get("union_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            union_user_account: data
                .get("union_user_account")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }

    /// Gets the user's avatar URL if they have one.
    ///
    /// Returns the full URL to the user's avatar image.
    pub fn avatar_url(&self) -> Option<String> {
        self.avatar.as_ref().map(|_hash| {
            format!(
                "https://thirdqq.qlogo.cn/headimg_dl?dst_uin={}&spec=640",
                self.id
            )
        })
    }

    /// Gets the user's display name.
    ///
    /// This is the same as the username for regular users.
    pub fn display_name(&self) -> &str {
        &self.username
    }

    /// Returns true if this user is a bot.
    pub fn is_bot(&self) -> bool {
        self.bot
    }

    /// Returns true if this user is a human.
    pub fn is_human(&self) -> bool {
        !self.bot
    }

    /// Gets the user's mention string.
    ///
    /// Returns a string that can be used to mention this user in messages.
    pub fn mention(&self) -> String {
        format!("<@!{}>", self.id)
    }
}

impl HasId for User {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.id)
    }
}

/// Represents a guild member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    /// The underlying user object
    #[serde(flatten)]
    pub user: User,
    /// The member's nickname in the guild
    pub nick: Option<String>,
    /// Array of role IDs
    pub roles: Vec<Snowflake>,
    /// When the user joined the guild
    pub joined_at: Timestamp,
    /// Whether the user is deafened in voice channels
    #[serde(default)]
    pub deaf: bool,
    /// Whether the user is muted in voice channels
    #[serde(default)]
    pub mute: bool,
}

impl Member {
    /// Creates a new member from a user.
    pub fn new(user: User, joined_at: Timestamp) -> Self {
        Self {
            user,
            nick: None,
            roles: Vec::new(),
            joined_at,
            deaf: false,
            mute: false,
        }
    }

    /// Creates a new member from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        let user = data
            .get("user")
            .map(|v| User::from_data(v.clone()))
            .unwrap_or_default();
        let joined_at = data
            .get("joined_at")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);

        Self {
            user,
            nick: data.get("nick").and_then(|v| v.as_str()).map(String::from),
            roles: data
                .get("roles")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default(),
            joined_at,
            deaf: data.get("deaf").and_then(|v| v.as_bool()).unwrap_or(false),
            mute: data.get("mute").and_then(|v| v.as_bool()).unwrap_or(false),
        }
    }

    /// Gets the member's display name (nickname if set, otherwise username).
    pub fn display_name(&self) -> &str {
        self.nick.as_deref().unwrap_or(&self.user.username)
    }

    /// Gets the member's mention string.
    pub fn mention(&self) -> String {
        self.user.mention()
    }

    /// Returns true if the member has the specified role.
    pub fn has_role(&self, role_id: &Snowflake) -> bool {
        self.roles.contains(role_id)
    }

    /// Returns true if the member has any of the specified roles.
    pub fn has_any_role(&self, role_ids: &[Snowflake]) -> bool {
        role_ids.iter().any(|role_id| self.has_role(role_id))
    }

    /// Returns true if the member has all of the specified roles.
    pub fn has_all_roles(&self, role_ids: &[Snowflake]) -> bool {
        role_ids.iter().all(|role_id| self.has_role(role_id))
    }

    /// Gets the member's avatar URL.
    pub fn avatar_url(&self) -> Option<String> {
        self.user.avatar_url()
    }

    /// Returns true if this member is a bot.
    pub fn is_bot(&self) -> bool {
        self.user.is_bot()
    }
}

impl HasId for Member {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.user.id)
    }
}

impl std::ops::Deref for Member {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

/// Represents a role in a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    /// The role's unique ID
    pub id: Snowflake,
    /// The role's name
    pub name: String,
    /// The role's color
    pub color: u32,
    /// Whether this role is hoisted (displayed separately in the member list)
    #[serde(default)]
    pub hoist: bool,
    /// The role's position in the hierarchy
    pub position: i32,
    /// The role's permissions
    pub permissions: String,
    /// Whether this role is managed by an integration
    #[serde(default)]
    pub managed: bool,
    /// Whether this role is mentionable
    #[serde(default)]
    pub mentionable: bool,
}

impl Role {
    /// Creates a new role.
    pub fn new(id: impl Into<Snowflake>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            color: 0,
            hoist: false,
            position: 0,
            permissions: "0".to_string(),
            managed: false,
            mentionable: false,
        }
    }

    /// Gets the role's mention string.
    pub fn mention(&self) -> String {
        format!("<@&{}>", self.id)
    }

    /// Gets the role's color as RGB values.
    pub fn rgb(&self) -> (u8, u8, u8) {
        let r = ((self.color >> 16) & 0xFF) as u8;
        let g = ((self.color >> 8) & 0xFF) as u8;
        let b = (self.color & 0xFF) as u8;
        (r, g, b)
    }

    /// Gets the role's color as a hex string.
    pub fn hex_color(&self) -> String {
        format!("#{:06X}", self.color)
    }
}

impl HasId for Role {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.id)
    }
}

impl crate::models::HasName for Role {
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_user_creation() {
        let user = User::new("123456789", "TestUser");
        assert_eq!(user.id, "123456789");
        assert_eq!(user.username, "TestUser");
        assert!(!user.is_bot());
        assert!(user.is_human());
    }

    #[test]
    fn test_user_mention() {
        let user = User::new("123456789", "TestUser");
        assert_eq!(user.mention(), "<@!123456789>");
    }

    #[test]
    fn test_member_display_name() {
        let user = User::new("123456789", "TestUser");
        let mut member = Member::new(user, Utc::now());

        // Without nickname, should return username
        assert_eq!(member.display_name(), "TestUser");

        // With nickname, should return nickname
        member.nick = Some("Nickname".to_string());
        assert_eq!(member.display_name(), "Nickname");
    }

    #[test]
    fn test_member_roles() {
        let user = User::new("123456789", "TestUser");
        let mut member = Member::new(user, Utc::now());

        member.roles = vec!["role1".to_string(), "role2".to_string()];

        assert!(member.has_role(&"role1".to_string()));
        assert!(!member.has_role(&"role3".to_string()));

        assert!(member.has_any_role(&["role1".to_string(), "role3".to_string()]));
        assert!(member.has_all_roles(&["role1".to_string(), "role2".to_string()]));
        assert!(!member.has_all_roles(&["role1".to_string(), "role3".to_string()]));
    }

    #[test]
    fn test_role_creation() {
        let role = Role::new("123456789", "TestRole");
        assert_eq!(role.id, "123456789");
        assert_eq!(role.name, "TestRole");
        assert_eq!(role.mention(), "<@&123456789>");
    }

    #[test]
    fn test_role_color() {
        let mut role = Role::new("123456789", "TestRole");
        role.color = 0xFF5733; // Orange color

        let (r, g, b) = role.rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 87);
        assert_eq!(b, 51);

        assert_eq!(role.hex_color(), "#FF5733");
    }
}
