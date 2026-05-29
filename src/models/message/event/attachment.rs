use crate::models::Snowflake;
use crate::models::serde_helpers::is_default;
use serde::{Deserialize, Serialize};

/// Attachment in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageAttachment {
    /// The attachment's ID, when supplied by the platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    /// The attachment's filename
    #[serde(skip_serializing_if = "is_default")]
    pub filename: String,
    /// The attachment's content type
    #[serde(skip_serializing_if = "is_default")]
    pub content_type: String,
    /// The attachment's content marker from open-message events.
    #[serde(default, skip_serializing_if = "is_default")]
    pub content: String,
    /// The attachment's size in bytes
    #[serde(skip_serializing_if = "is_default")]
    pub size: u64,
    /// The attachment's URL
    #[serde(skip_serializing_if = "is_default")]
    pub url: String,
    /// The attachment's width (for images)
    #[serde(default, skip_serializing_if = "is_default")]
    pub width: u32,
    /// The attachment's height (for images)
    #[serde(default, skip_serializing_if = "is_default")]
    pub height: u32,
}
