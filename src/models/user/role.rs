use crate::models::{HasId, Snowflake};
use serde::{Deserialize, Serialize};

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
