//! Emoji-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for handling emojis in reactions and messages.

use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Types of emojis supported by the QQ Guild API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum EmojiType {
    /// System emoji (built-in emojis)
    System = 1,
    /// Custom emoji (user-uploaded emojis)
    Custom = 2,
    /// Unknown emoji type
    Unknown(u8),
}

wire_enum!(EmojiType, u8, Unknown, {
    System = 1,
    Custom = 2,
});

/// Represents an emoji used in reactions or messages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Emoji {
    /// Unique identifier for the emoji
    pub id: Snowflake,
    /// Type of the emoji (system or custom)
    #[serde(rename = "type")]
    pub emoji_type: EmojiType,
    /// Name of the emoji (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL to the emoji image (for custom emojis)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_type_conversion() {
        assert_eq!(EmojiType::from(1), EmojiType::System);
        assert_eq!(EmojiType::from(2), EmojiType::Custom);
        assert_eq!(u8::from(EmojiType::System), 1);
        assert_eq!(u8::from(EmojiType::Custom), 2);

        assert_eq!(EmojiType::from(99), EmojiType::Unknown(99));
        assert_eq!(u8::from(EmojiType::Unknown(99)), 99);
    }

    #[test]
    fn emoji_omits_absent_metadata_like_official_dto() {
        let emoji = Emoji {
            id: "123".to_string(),
            emoji_type: EmojiType::System,
            name: None,
            url: None,
        };
        assert_eq!(
            serde_json::to_value(&emoji).unwrap(),
            serde_json::json!({
                "id": "123",
                "type": 1
            })
        );

        let custom = Emoji {
            id: "789".to_string(),
            emoji_type: EmojiType::Custom,
            name: Some("happy".to_string()),
            url: Some("https://example.com/happy.png".to_string()),
        };
        assert_eq!(
            serde_json::to_value(&custom).unwrap(),
            serde_json::json!({
                "id": "789",
                "type": 2,
                "name": "happy",
                "url": "https://example.com/happy.png"
            })
        );
    }

    #[test]
    fn test_emoji_id() {
        let emoji = Emoji {
            id: "test_id".to_string(),
            emoji_type: EmojiType::System,
            name: None,
            url: None,
        };
        assert_eq!(emoji.id, "test_id");
    }
}
