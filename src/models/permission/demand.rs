use super::APIPermissionDemandIdentify;
use crate::models::{HasId, Snowflake};
use serde::{Deserialize, Serialize};

/// Represents a permission demand request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissionDemand {
    /// The guild ID where permission is requested
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub guild_id: Snowflake,
    /// The channel ID where the permission request will be sent
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub channel_id: Snowflake,
    /// The API identifier for which permission is requested
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_identify: Option<APIPermissionDemandIdentify>,
    /// The title of the permission request
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub title: String,
    /// Description explaining why the permission is needed
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub desc: String,
}

impl HasId for APIPermissionDemand {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.guild_id)
    }
}
