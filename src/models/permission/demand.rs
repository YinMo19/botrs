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

impl APIPermissionDemand {
    /// Creates a permission demand request.
    pub fn new(
        guild_id: impl Into<String>,
        channel_id: impl Into<String>,
        api_identify: APIPermissionDemandIdentify,
        desc: impl Into<String>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            api_identify: Some(api_identify),
            title: String::new(),
            desc: desc.into(),
        }
    }

    /// Sets the title for this permission demand.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Gets the API path being requested.
    pub fn api_path(&self) -> &str {
        self.api_identify
            .as_ref()
            .map(|identify| identify.path.as_str())
            .unwrap_or_default()
    }

    /// Gets the HTTP method being requested.
    pub fn api_method(&self) -> &str {
        self.api_identify
            .as_ref()
            .map(|identify| identify.method.as_str())
            .unwrap_or_default()
    }
}

impl HasId for APIPermissionDemand {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.guild_id)
    }
}

impl std::fmt::Display for APIPermissionDemand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PermissionDemand {{ guild_id: {}, api: {}, desc: {} }}",
            self.guild_id,
            self.api_identify
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
            self.desc.chars().take(50).collect::<String>()
        )
    }
}
