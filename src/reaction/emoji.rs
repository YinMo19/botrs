use serde::{Deserialize, Serialize};

/// Emoji structure for reactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Emoji {
    /// Emoji ID
    #[serde(default)]
    pub id: String,
    /// Emoji type
    #[serde(default, rename = "type")]
    pub emoji_type: i32,
}
