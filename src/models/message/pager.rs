use serde::{Deserialize, Serialize};

fn insert_query_param(
    query: &mut std::collections::HashMap<String, String>,
    key: &str,
    value: &Option<String>,
) {
    if let Some(value) = value.as_ref().filter(|value| !value.is_empty()) {
        query.insert(key.to_string(), value.clone());
    }
}

/// Message list pagination mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessagePagerType {
    /// Pull messages around the given message ID
    Around,
    /// Pull messages before the given message ID
    Before,
    /// Pull messages after the given message ID
    After,
}

impl MessagePagerType {
    /// Returns the query parameter name for this pager type.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Around => "around",
            Self::Before => "before",
            Self::After => "after",
        }
    }
}

/// Pager for pulling channel messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessagesPager {
    /// Pull direction
    #[serde(skip)]
    pub pager_type: Option<MessagePagerType>,
    /// Message ID used by the pull direction
    #[serde(skip)]
    pub id: Option<String>,
    /// Page size, max 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl MessagesPager {
    /// Converts the pager to query parameters.
    pub fn query_params(&self) -> std::collections::HashMap<String, String> {
        let mut query = std::collections::HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        if let Some(pager_type) = self.pager_type {
            insert_query_param(&mut query, pager_type.as_str(), &self.id);
        }
        query
    }
}

/// Setting guide payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingGuide {
    /// Guild ID for DM setting guide jumps
    pub guild_id: String,
}

/// Body for setting guide messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingGuideToCreate {
    /// Content used by channel setting guides, usually mentions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Setting guide jump target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_guide: Option<SettingGuide>,
}
