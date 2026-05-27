//! Emoji-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for handling emojis in reactions and messages.

use crate::models::{HasId, Snowflake};
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

impl EmojiType {
    /// Returns a human-readable description of the emoji type.
    pub fn description(&self) -> &'static str {
        match self {
            EmojiType::System => "System emoji",
            EmojiType::Custom => "Custom emoji",
            EmojiType::Unknown(_) => "Unknown emoji type",
        }
    }
}

impl std::fmt::Display for EmojiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

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

impl HasId for Emoji {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.id)
    }
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, ":{}: ({})", name, self.emoji_type)
        } else {
            write!(f, "Emoji {} ({})", self.id, self.emoji_type)
        }
    }
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
    fn test_emoji_type_description() {
        assert_eq!(EmojiType::System.description(), "System emoji");
        assert_eq!(EmojiType::Custom.description(), "Custom emoji");
        assert_eq!(EmojiType::Unknown(5).description(), "Unknown emoji type");
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
    fn test_emoji_has_id() {
        let emoji = Emoji {
            id: "test_id".to_string(),
            emoji_type: EmojiType::System,
            name: None,
            url: None,
        };
        assert_eq!(emoji.id(), Some(&"test_id".to_string()));
    }

    #[test]
    fn test_emoji_display() {
        let named_emoji = Emoji {
            id: "123".to_string(),
            emoji_type: EmojiType::Custom,
            name: Some("happy".to_string()),
            url: None,
        };
        let display = format!("{}", named_emoji);
        assert!(display.contains(":happy:"));
        assert!(display.contains("Custom emoji"));

        let unnamed_emoji = Emoji {
            id: "456".to_string(),
            emoji_type: EmojiType::System,
            name: None,
            url: None,
        };
        let display = format!("{}", unnamed_emoji);
        assert!(display.contains("Emoji 456"));
        assert!(display.contains("System emoji"));
    }
}
