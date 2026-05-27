use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

use super::SendType;

/// Rich media upload/direct-send payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RichMediaMessage {
    /// File type, 1=image, 2=video, 3=audio
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub file_type: Option<u64>,
    /// Rich media URL
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub url: Option<String>,
    /// Whether the server should send the message directly
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub srv_send_msg: Option<bool>,
    /// Optional text content
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub content: Option<String>,
    /// Message sequence number
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub msg_seq: Option<i64>,
}

impl RichMediaMessage {
    /// Creates a rich media payload with file type and URL.
    pub fn new(file_type: u64, url: impl Into<String>) -> Self {
        Self {
            file_type: Some(file_type),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Returns the send type for route selection.
    pub const fn send_type(&self) -> SendType {
        SendType::RichMedia
    }

    /// Returns the event ID used for passive replies.
    ///
    /// Botgo intentionally returns an empty value for rich media payloads.
    pub const fn event_id(&self) -> &str {
        ""
    }
}
