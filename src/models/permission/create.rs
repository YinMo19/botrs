use super::APIPermissionDemandIdentify;
use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Body for creating an API permission demand.
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
    /// Creates a new permission demand creation body.
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
