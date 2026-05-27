use crate::models::serde_helpers::is_zero_u32;
use crate::models::{HasId, HasName, Snowflake};
use serde::{Deserialize, Serialize};

/// Guild roles response wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildRoles {
    /// Guild ID
    #[serde(default)]
    pub guild_id: Snowflake,
    /// List of roles in the guild
    #[serde(default)]
    pub roles: Vec<GuildRole>,
    /// Number of roles
    #[serde(default, rename = "role_num_limit")]
    pub num_limit: String,
}

impl GuildRoles {
    /// Creates a new guild roles wrapper.
    pub fn new(roles: Vec<GuildRole>) -> Self {
        Self {
            guild_id: String::new(),
            roles,
            num_limit: String::new(),
        }
    }
}

/// Default role color used when creating or updating roles.
pub const DEFAULT_ROLE_COLOR: u32 = 4_278_245_297;

/// Role update info body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateRoleInfo {
    pub name: String,
    pub color: u32,
    pub hoist: u32,
}

/// Represents a role in a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildRole {
    /// The role's unique ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: Snowflake,
    /// The role's name
    #[serde(default)]
    pub name: String,
    /// The role's color (ARGB hex as decimal)
    #[serde(default)]
    pub color: u32,
    /// Whether this role is displayed separately in the member list
    #[serde(default)]
    pub hoist: u32,
    /// The number of members with this role
    #[serde(default, rename = "number", skip_serializing_if = "is_zero_u32")]
    pub member_count: u32,
    /// The number of online members with this role
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub member_limit: u32,
}

impl GuildRole {
    fn default_color() -> u32 {
        DEFAULT_ROLE_COLOR
    }

    fn hoist_value(&self) -> u32 {
        self.hoist
    }
}

/// Filter identifying which role fields are being updated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRoleFilter {
    pub name: u32,
    pub color: u32,
    pub hoist: u32,
}

impl Default for UpdateRoleFilter {
    fn default() -> Self {
        Self {
            name: 1,
            color: 1,
            hoist: 1,
        }
    }
}

/// Role update body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRole {
    pub guild_id: String,
    pub filter: UpdateRoleFilter,
    #[serde(rename = "info")]
    pub update: GuildRole,
}

impl UpdateRole {
    /// Creates a role update body with sensible defaults.
    pub fn new(guild_id: impl Into<String>, mut role: GuildRole) -> Self {
        if role.color == 0 {
            role.color = GuildRole::default_color();
        }
        Self {
            guild_id: guild_id.into(),
            filter: UpdateRoleFilter::default(),
            update: role,
        }
    }
}

/// Result returned from role create/update APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateResult {
    #[serde(default)]
    pub role_id: Snowflake,
    #[serde(default)]
    pub guild_id: Snowflake,
    #[serde(default)]
    pub role: Option<GuildRole>,
}

impl GuildRole {
    /// Creates a new role.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if this role is hoisted (displayed separately).
    pub fn is_hoisted(&self) -> bool {
        self.hoist != 0
    }

    /// Converts the role to the numeric hoist value expected by the API.
    pub fn hoist_as_u32(&self) -> u32 {
        self.hoist_value()
    }

    /// Gets the role's color as a hex value.
    pub fn color_hex(&self) -> Option<String> {
        (self.color != 0).then(|| format!("#{:06X}", self.color))
    }

    /// Gets the number of members with this role.
    pub fn member_count(&self) -> u32 {
        self.member_count
    }

    /// Gets the member limit for this role.
    pub fn get_member_limit(&self) -> u32 {
        self.member_limit
    }

    /// Returns true if the role has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        self.member_limit > 0 && self.member_count >= self.member_limit
    }
}

impl HasId for GuildRole {
    fn id(&self) -> Option<&Snowflake> {
        (!self.id.is_empty()).then_some(&self.id)
    }
}

impl HasName for GuildRole {
    fn name(&self) -> &str {
        &self.name
    }
}
