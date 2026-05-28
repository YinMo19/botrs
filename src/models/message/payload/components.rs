use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

use super::super::Media;

/// Rich media info used after uploading media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct MediaInfo {
    /// Uploaded rich media file info
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) file_info: Option<String>,
}

impl From<Media> for MediaInfo {
    fn from(media: Media) -> Self {
        Self {
            file_info: media.file_info,
        }
    }
}
