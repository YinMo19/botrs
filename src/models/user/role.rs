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
