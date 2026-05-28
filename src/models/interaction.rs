//! Interaction gateway event payloads and OpenAPI response models.

pub use crate::interaction::{
    Interaction, InteractionData, InteractionDataType, InteractionType, Resolved,
};

use serde::{Deserialize, Serialize};

/// Inline search response payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchResponse {
    pub layouts: Vec<SearchLayout>,
}

/// Inline search layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SearchLayout {
    pub layout_type: SearchLayoutType,
    pub action_type: SearchActionType,
    pub title: String,
    pub records: Vec<SearchRecord>,
}

/// Inline search layout type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum SearchLayoutType {
    /// Left-image, right-text layout.
    #[default]
    ImageText = 0,
    /// Unknown platform value.
    Unknown(u32),
}

wire_enum!(SearchLayoutType, u32, Unknown, {
    ImageText = 0,
});

/// Inline search action type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum SearchActionType {
    /// Send an Ark message from the selected record.
    #[default]
    SendArk = 0,
    /// Unknown platform value.
    Unknown(u32),
}

wire_enum!(SearchActionType, u32, Unknown, {
    SendArk = 0,
});

/// Inline search record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchRecord {
    pub cover: String,
    pub title: String,
    pub tips: String,
    pub url: String,
}
