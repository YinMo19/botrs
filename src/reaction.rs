//! Reaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling message reactions,
//! emoji reactions, and reaction-related events.

use crate::api::BotApi;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Reaction target type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ReactionTargetType {
    /// Message reaction
    Message = 0,
    /// Post reaction
    Post = 1,
    /// Comment reaction
    Comment = 2,
    /// Reply reaction
    Reply = 3,
}

impl From<u8> for ReactionTargetType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Message,
            1 => Self::Post,
            2 => Self::Comment,
            3 => Self::Reply,
            _ => Self::Message, // Default fallback
        }
    }
}

/// Emoji structure for reactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emoji {
    /// Emoji ID
    pub id: Option<String>,
    /// Emoji type
    pub emoji_type: Option<u8>,
}

impl Emoji {
    /// Create a new Emoji instance from JSON data
    pub fn new(data: &Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            emoji_type: data.get("type").and_then(|v| v.as_u64()).map(|v| v as u8),
        }
    }
}

/// Reaction target structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionTarget {
    /// Target ID
    pub id: Option<String>,
    /// Target type (message, post, comment, reply)
    pub target_type: Option<ReactionTargetType>,
}

impl ReactionTarget {
    /// Create a new ReactionTarget instance from JSON data
    pub fn new(data: &Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            target_type: data
                .get("type")
                .and_then(|v| v.as_u64())
                .map(|v| ReactionTargetType::from(v as u8)),
        }
    }
}

/// Reaction structure representing emoji reactions to messages or posts
#[derive(Debug, Clone, Serialize)]
pub struct Reaction {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// User ID who made the reaction
    pub user_id: Option<String>,
    /// Channel ID where the reaction occurred
    pub channel_id: Option<String>,
    /// Guild ID where the reaction occurred
    pub guild_id: Option<String>,
    /// Emoji used for the reaction
    pub emoji: Emoji,
    /// Target of the reaction (message, post, etc.)
    pub target: ReactionTarget,
    /// Event ID
    pub event_id: Option<String>,
}

impl Reaction {
    /// Create a new Reaction instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `event_id` - Optional event ID
    /// * `data` - Reaction data from the gateway
    pub fn new(api: BotApi, event_id: Option<String>, data: &Value) -> Self {
        Self {
            api,
            event_id,
            user_id: data
                .get("user_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            emoji: Emoji::new(
                data.get("emoji")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
            target: ReactionTarget::new(
                data.get("target")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Check if this is a message reaction
    pub fn is_message_reaction(&self) -> bool {
        matches!(self.target.target_type, Some(ReactionTargetType::Message))
    }

    /// Check if this is a post reaction
    pub fn is_post_reaction(&self) -> bool {
        matches!(self.target.target_type, Some(ReactionTargetType::Post))
    }

    /// Check if this is a comment reaction
    pub fn is_comment_reaction(&self) -> bool {
        matches!(self.target.target_type, Some(ReactionTargetType::Comment))
    }

    /// Check if this is a reply reaction
    pub fn is_reply_reaction(&self) -> bool {
        matches!(self.target.target_type, Some(ReactionTargetType::Reply))
    }

    /// Get the target ID
    pub fn target_id(&self) -> Option<&str> {
        self.target.id.as_deref()
    }

    /// Get the emoji ID
    pub fn emoji_id(&self) -> Option<&str> {
        self.emoji.id.as_deref()
    }
}

impl std::fmt::Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Reaction {{ user_id: {:?}, channel_id: {:?}, guild_id: {:?}, target_type: {:?}, event_id: {:?} }}",
            self.user_id, self.channel_id, self.guild_id, self.target.target_type, self.event_id
        )
    }
}

/// User structure for reaction users list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionUser {
    /// User ID
    pub id: Option<String>,
    /// Username
    pub username: Option<String>,
    /// User avatar URL
    pub avatar: Option<String>,
}

impl ReactionUser {
    /// Create a new ReactionUser instance from JSON data
    pub fn new(data: &Value) -> Self {
        Self {
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            username: data
                .get("username")
                .and_then(|v| v.as_str())
                .map(String::from),
            avatar: data
                .get("avatar")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// Reaction users response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionUsers {
    /// List of users who reacted
    pub users: Vec<ReactionUser>,
    /// Pagination cookie for next page
    pub cookie: Option<String>,
    /// Whether this is the last page
    pub is_end: bool,
}

impl ReactionUsers {
    /// Create a new ReactionUsers instance from JSON data
    pub fn new(data: &Value) -> Self {
        let users = data
            .get("users")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().map(ReactionUser::new).collect())
            .unwrap_or_default();

        Self {
            users,
            cookie: data
                .get("cookie")
                .and_then(|v| v.as_str())
                .map(String::from),
            is_end: data.get("is_end").and_then(|v| v.as_bool()).unwrap_or(true),
        }
    }

    /// Check if there are more pages available
    pub fn has_more_pages(&self) -> bool {
        !self.is_end
    }

    /// Get the number of users in this page
    pub fn user_count(&self) -> usize {
        self.users.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_target_type() {
        assert_eq!(ReactionTargetType::Message as u8, 0);
        assert_eq!(ReactionTargetType::Post as u8, 1);
        assert_eq!(ReactionTargetType::Comment as u8, 2);
        assert_eq!(ReactionTargetType::Reply as u8, 3);
    }

    #[test]
    fn test_reaction_target_type_from() {
        assert_eq!(ReactionTargetType::from(0), ReactionTargetType::Message);
        assert_eq!(ReactionTargetType::from(1), ReactionTargetType::Post);
        assert_eq!(ReactionTargetType::from(2), ReactionTargetType::Comment);
        assert_eq!(ReactionTargetType::from(3), ReactionTargetType::Reply);
        assert_eq!(ReactionTargetType::from(99), ReactionTargetType::Message); // Default fallback
    }

    #[test]
    fn test_emoji_creation() {
        let data = serde_json::json!({
            "id": "emoji123",
            "type": 1
        });
        let emoji = Emoji::new(&data);
        assert_eq!(emoji.id, Some("emoji123".to_string()));
        assert_eq!(emoji.emoji_type, Some(1));
    }

    #[test]
    fn test_reaction_target_creation() {
        let data = serde_json::json!({
            "id": "target123",
            "type": 0
        });
        let target = ReactionTarget::new(&data);
        assert_eq!(target.id, Some("target123".to_string()));
        assert_eq!(target.target_type, Some(ReactionTargetType::Message));
    }

    #[test]
    fn test_reaction_user_creation() {
        let data = serde_json::json!({
            "id": "user123",
            "username": "testuser",
            "avatar": "https://example.com/avatar.png"
        });
        let user = ReactionUser::new(&data);
        assert_eq!(user.id, Some("user123".to_string()));
        assert_eq!(user.username, Some("testuser".to_string()));
        assert_eq!(
            user.avatar,
            Some("https://example.com/avatar.png".to_string())
        );
    }
}
