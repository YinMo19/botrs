use crate::models::Snowflake;
use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

/// Channel permissions for a user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChannelPermissions {
    /// The channel ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub channel_id: Snowflake,
    /// The user ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user_id: Snowflake,
    /// The permissions string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub permissions: String,
}

impl ChannelPermissions {
    /// Creates new channel permissions.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Channel role permissions response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChannelRolesPermissions {
    /// The channel ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub channel_id: Snowflake,
    /// The role ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub role_id: Snowflake,
    /// The permissions string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub permissions: String,
}

/// Body for updating channel user or role permissions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateChannelPermissions {
    /// Permissions to add
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub add: Option<String>,
    /// Permissions to remove
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub remove: Option<String>,
}

impl UpdateChannelPermissions {
    /// Creates update-channel-permissions parameters.
    pub fn new(add: Option<impl ToString>, remove: Option<impl ToString>) -> Self {
        Self {
            add: add.map(|value| value.to_string()),
            remove: remove.map(|value| value.to_string()),
        }
    }

    /// Validates that the permission strings can be parsed as unsigned integers.
    pub fn validate(&self) -> crate::error::Result<()> {
        if let Some(add) = self.add.as_deref().filter(|value| !value.is_empty()) {
            add.parse::<u64>().map_err(|err| {
                crate::error::BotError::invalid_data(format!("invalid parameter add: {err}"))
            })?;
        }
        if let Some(remove) = self.remove.as_deref().filter(|value| !value.is_empty()) {
            remove.parse::<u64>().map_err(|err| {
                crate::error::BotError::invalid_data(format!("invalid parameter remove: {err}"))
            })?;
        }
        Ok(())
    }
}
