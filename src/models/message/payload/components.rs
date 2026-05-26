use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

use super::super::{Keyboard, Media};

/// Input status notification payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InputNotify {
    /// Input status type
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub input_type: Option<i32>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub input_second: Option<i32>,
}

/// Rich media info used after uploading media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaInfo {
    /// Uploaded rich media file info
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub file_info: Option<String>,
}

impl From<Media> for MediaInfo {
    fn from(media: Media) -> Self {
        Self {
            file_info: media.file_info,
        }
    }
}

impl From<MediaInfo> for Media {
    fn from(media: MediaInfo) -> Self {
        Self {
            file_uuid: None,
            file_info: media.file_info,
            ttl: None,
        }
    }
}

/// Streamed message fragment metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Stream {
    /// Stream state
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub state: Option<i32>,
    /// Stream ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub id: Option<String>,
    /// Fragment index
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub index: Option<i32>,
    /// Whether to reset an unfinished stream
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub reset: Option<bool>,
}

/// Prompt keyboard wrapper used by message extension areas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PromptKeyboard {
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
}

/// Message action button configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionButton {
    /// Action bar template ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub template_id: Option<i32>,
    /// Callback payload
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub callback_data: Option<String>,
    /// Feedback button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub feedback: Option<bool>,
    /// TTS button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub tts: Option<bool>,
    /// Regenerate button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub re_generate: Option<bool>,
    /// Stop generation button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub stop_generate: Option<bool>,
}
