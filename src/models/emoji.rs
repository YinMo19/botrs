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

impl From<u8> for EmojiType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::System,
            2 => Self::Custom,
            other => Self::Unknown(other),
        }
    }
}

impl From<EmojiType> for u8 {
    fn from(emoji_type: EmojiType) -> Self {
        match emoji_type {
            EmojiType::System => 1,
            EmojiType::Custom => 2,
            EmojiType::Unknown(value) => value,
        }
    }
}

impl EmojiType {
    /// Returns a human-readable description of the emoji type.
    pub fn description(&self) -> &'static str {
        match self {
            EmojiType::System => "System emoji",
            EmojiType::Custom => "Custom emoji",
            EmojiType::Unknown(_) => "Unknown emoji type",
        }
    }

    /// Returns true if this is a system emoji.
    pub fn is_system(&self) -> bool {
        matches!(self, EmojiType::System)
    }

    /// Returns true if this is a custom emoji.
    pub fn is_custom(&self) -> bool {
        matches!(self, EmojiType::Custom)
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
    pub name: Option<String>,
    /// URL to the emoji image (for custom emojis)
    pub url: Option<String>,
}

impl Emoji {
    /// Creates a new Emoji instance.
    ///
    /// # Arguments
    ///
    /// * `id` - The emoji ID
    /// * `emoji_type` - The type of emoji (system or custom)
    pub fn new(id: impl Into<String>, emoji_type: EmojiType) -> Self {
        Self {
            id: id.into(),
            emoji_type,
            name: None,
            url: None,
        }
    }

    /// Creates a new system emoji.
    ///
    /// # Arguments
    ///
    /// * `id` - The system emoji ID
    pub fn system(id: impl Into<String>) -> Self {
        Self::new(id, EmojiType::System)
    }

    /// Creates a new custom emoji.
    ///
    /// # Arguments
    ///
    /// * `id` - The custom emoji ID
    /// * `name` - Optional name for the emoji
    /// * `url` - Optional URL to the emoji image
    pub fn custom(id: impl Into<String>, name: Option<String>, url: Option<String>) -> Self {
        Self {
            id: id.into(),
            emoji_type: EmojiType::Custom,
            name,
            url,
        }
    }

    /// Sets the name for this emoji.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the URL for this emoji.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Returns true if this is a system emoji.
    pub fn is_system(&self) -> bool {
        self.emoji_type.is_system()
    }

    /// Returns true if this is a custom emoji.
    pub fn is_custom(&self) -> bool {
        self.emoji_type.is_custom()
    }

    /// Gets the display name of the emoji.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.id)
    }
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
    fn test_emoji_type_checks() {
        assert!(EmojiType::System.is_system());
        assert!(!EmojiType::System.is_custom());
        assert!(!EmojiType::Custom.is_system());
        assert!(EmojiType::Custom.is_custom());
    }

    #[test]
    fn test_emoji_creation() {
        let emoji = Emoji::new("123", EmojiType::System);
        assert_eq!(emoji.id, "123");
        assert_eq!(emoji.emoji_type, EmojiType::System);
        assert_eq!(emoji.name, None);
        assert_eq!(emoji.url, None);
        assert!(emoji.is_system());
    }

    #[test]
    fn test_system_emoji() {
        let emoji = Emoji::system("456");
        assert_eq!(emoji.id, "456");
        assert!(emoji.is_system());
        assert!(!emoji.is_custom());
    }

    #[test]
    fn test_custom_emoji() {
        let emoji = Emoji::custom(
            "789",
            Some("happy".to_string()),
            Some("https://example.com/happy.png".to_string()),
        );
        assert_eq!(emoji.id, "789");
        assert_eq!(emoji.name, Some("happy".to_string()));
        assert_eq!(emoji.url, Some("https://example.com/happy.png".to_string()));
        assert!(emoji.is_custom());
        assert!(!emoji.is_system());
    }

    #[test]
    fn test_emoji_builder_methods() {
        let emoji = Emoji::system("111")
            .with_name("smile")
            .with_url("https://example.com/smile.png");

        assert_eq!(emoji.name, Some("smile".to_string()));
        assert_eq!(emoji.url, Some("https://example.com/smile.png".to_string()));
    }

    #[test]
    fn test_emoji_display_name() {
        let emoji_with_name = Emoji::custom("123", Some("test".to_string()), None);
        assert_eq!(emoji_with_name.display_name(), "test");

        let emoji_without_name = Emoji::system("456");
        assert_eq!(emoji_without_name.display_name(), "456");
    }

    #[test]
    fn test_emoji_has_id() {
        let emoji = Emoji::system("test_id");
        assert_eq!(emoji.id(), Some(&"test_id".to_string()));
    }

    #[test]
    fn test_emoji_display() {
        let named_emoji = Emoji::custom("123", Some("happy".to_string()), None);
        let display = format!("{}", named_emoji);
        assert!(display.contains(":happy:"));
        assert!(display.contains("Custom emoji"));

        let unnamed_emoji = Emoji::system("456");
        let display = format!("{}", unnamed_emoji);
        assert!(display.contains("Emoji 456"));
        assert!(display.contains("System emoji"));
    }
}
