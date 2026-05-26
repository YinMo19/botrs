use serde::{Deserialize, Serialize};

/// Emoji structure for reactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Emoji {
    /// Emoji ID
    pub id: String,
    /// Emoji type
    #[serde(rename = "type")]
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

    /// Creates a new Emoji instance.
    pub fn with_type(id: impl Into<String>, emoji_type: i32) -> Self {
        Self::new(id, emoji_type)
    }
}
