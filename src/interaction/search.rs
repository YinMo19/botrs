use serde::{Deserialize, Serialize};

/// Search input resolved data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchInputResolved {
    /// Search keyword
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub keyword: String,
}

/// Search interaction response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchRsp {
    /// Search layouts
    #[serde(default)]
    pub layouts: Vec<SearchLayout>,
}

/// Search result layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchLayout {
    /// Layout type
    #[serde(rename = "LayoutType")]
    pub layout_type: u32,
    /// Action type
    #[serde(rename = "ActionType")]
    pub action_type: u32,
    /// Layout title
    #[serde(rename = "Title")]
    pub title: String,
    /// Search records
    #[serde(rename = "Records", default)]
    pub records: Vec<SearchRecord>,
}

/// Search result record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchRecord {
    /// Cover URL
    #[serde(default)]
    pub cover: String,
    /// Title
    #[serde(default)]
    pub title: String,
    /// Tips
    #[serde(default)]
    pub tips: String,
    /// Target URL
    #[serde(rename = "url", alias = "URL", default)]
    pub url: String,
}
