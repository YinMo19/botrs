use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

/// Message scene metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageScene {
    /// Message source, for example realtime voice or AI search scenes
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub source: Option<String>,
    /// Callback data returned with the message scene
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub callback_data: Option<String>,
}
