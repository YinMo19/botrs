use crate::models::channel::Channel;
use crate::models::serde_helpers::is_default;
use serde::{Deserialize, Serialize};

/// Default role color used by the platform.
pub const DEFAULT_ROLE_COLOR: u32 = 4_278_245_297;

/// Guild roles list response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildRoles {
    #[serde(default)]
    pub guild_id: String,
    #[serde(default)]
    pub roles: Vec<Role>,
    #[serde(default)]
    pub role_num_limit: String,
}

/// Guild role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Role {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: u32,
    #[serde(default)]
    pub hoist: u32,
    #[serde(default, rename = "number", skip_serializing_if = "is_default")]
    pub member_count: u32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub member_limit: u32,
}

/// Role update filter.
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

/// Role create/update body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct UpdateRoleBody {
    pub guild_id: String,
    pub filter: UpdateRoleFilter,
    #[serde(rename = "info")]
    pub role: Role,
}

/// Role mutation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateRoleResult {
    #[serde(default)]
    pub role_id: String,
    #[serde(default)]
    pub guild_id: String,
    #[serde(default)]
    pub role: Option<Role>,
}

/// Extra body for adding/removing a member role scoped to a channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberRoleParams {
    #[serde(default)]
    pub channel: Option<Channel>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_role_params_keep_null_channel_on_the_wire() {
        let value = serde_json::to_value(MemberRoleParams::default()).unwrap();

        assert!(value.get("channel").is_some());
        assert!(value["channel"].is_null());
    }
}
