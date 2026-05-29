use crate::models::serde_helpers::is_default;
use serde::{Deserialize, Serialize};

/// Message scene metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageScene {
    /// Message source, for example realtime voice or AI search scenes
    #[serde(default, skip_serializing_if = "is_default")]
    pub source: String,
    /// Callback data returned with the message scene
    #[serde(default, skip_serializing_if = "is_default")]
    pub callback_data: String,
    /// Extra scene metadata returned by open-message events.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ext: Vec<String>,
}
