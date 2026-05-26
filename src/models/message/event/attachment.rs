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

impl MessageAttachment {
    /// Creates a new message attachment from API data.
    pub fn from_data(data: serde_json::Value) -> Self {
        serde_json::from_value(data).unwrap_or_default()
    }

    /// Returns true if this attachment is an image.
    pub fn is_image(&self) -> bool {
        self.content_type
            .as_ref()
            .is_some_and(|ct| ct.starts_with("image/"))
    }

    /// Returns true if this attachment is a video.
    pub fn is_video(&self) -> bool {
        self.content_type
            .as_ref()
            .is_some_and(|ct| ct.starts_with("video/"))
    }

    /// Returns true if this attachment is an audio file.
    pub fn is_audio(&self) -> bool {
        self.content_type
            .as_ref()
            .is_some_and(|ct| ct.starts_with("audio/"))
    }
}
