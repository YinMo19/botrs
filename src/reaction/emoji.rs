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

impl Emoji {
    /// Creates a new Emoji instance.
    pub fn new(id: impl Into<String>, emoji_type: i32) -> Self {
        Self {
            id: id.into(),
            emoji_type,
        }
    }
}
