use crate::models::Snowflake;
use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

/// Attachment in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageAttachment {
    /// The attachment's ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub id: Option<Snowflake>,
    /// The attachment's filename
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub filename: Option<String>,
    /// The attachment's content type
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub content_type: Option<String>,
    /// The attachment's size in bytes
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub size: Option<u64>,
    /// The attachment's URL
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub url: Option<String>,
    /// The attachment's width (for images)
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub width: Option<u32>,
    /// The attachment's height (for images)
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub height: Option<u32>,
}
