//! Reaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling message reactions,
//! emoji reactions, and reaction-related events.

use crate::api::BotApi;
use crate::models::user::User;
use crate::models::{Pager, Snowflake};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Reaction target type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
#[repr(i32)]
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

pub const REACTION_TARGET_TYPE_MSG: i32 = 0;
pub const REACTION_TARGET_TYPE_FEED: i32 = 1;
pub const REACTION_TARGET_TYPE_COMMENT: i32 = 2;
pub const REACTION_TARGET_TYPE_REPLY: i32 = 3;
#[allow(non_upper_case_globals)]
pub const ReactionTargetTypeMsg: i32 = REACTION_TARGET_TYPE_MSG;
#[allow(non_upper_case_globals)]
pub const ReactionTargetTypeFeed: i32 = REACTION_TARGET_TYPE_FEED;
#[allow(non_upper_case_globals)]
pub const ReactionTargetTypeComment: i32 = REACTION_TARGET_TYPE_COMMENT;
#[allow(non_upper_case_globals)]
pub const ReactionTargetTypeReply: i32 = REACTION_TARGET_TYPE_REPLY;

impl From<i32> for ReactionTargetType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Message,
            1 => Self::Post,
            2 => Self::Comment,
            3 => Self::Reply,
            _ => Self::Message,
        }
    }
}

impl From<ReactionTargetType> for i32 {
    fn from(value: ReactionTargetType) -> Self {
        match value {
            ReactionTargetType::Message => 0,
            ReactionTargetType::Post => 1,
            ReactionTargetType::Comment => 2,
            ReactionTargetType::Reply => 3,
        }
    }
}

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

/// Reaction target structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionTarget {
    /// Target ID
    pub id: String,
    /// Target type (message, post, comment, reply)
    #[serde(rename = "type")]
    pub target_type: ReactionTargetType,
}

impl ReactionTarget {
    /// Creates a new ReactionTarget instance.
    pub fn new(id: impl Into<String>, target_type: ReactionTargetType) -> Self {
        Self {
            id: id.into(),
            target_type,
        }
    }
}

/// Botgo-compatible message reaction DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageReaction {
    /// User ID who made the reaction.
    pub user_id: Snowflake,
    /// Channel ID where the reaction occurred.
    pub channel_id: Snowflake,
    /// Guild ID where the reaction occurred.
    pub guild_id: Snowflake,
    /// Target of the reaction.
    pub target: ReactionTarget,
    /// Emoji used for the reaction.
    pub emoji: Emoji,
}

impl MessageReaction {
    /// Creates a message reaction DTO.
    pub fn new(
        user_id: impl Into<Snowflake>,
        channel_id: impl Into<Snowflake>,
        guild_id: impl Into<Snowflake>,
        target: ReactionTarget,
        emoji: Emoji,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            channel_id: channel_id.into(),
            guild_id: guild_id.into(),
            target,
            emoji,
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
    pub user_id: Snowflake,
    /// Channel ID where the reaction occurred
    pub channel_id: Snowflake,
    /// Guild ID where the reaction occurred
    pub guild_id: Snowflake,
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
    pub fn new(api: BotApi, event_id: Option<String>, data: &Value) -> crate::Result<Self> {
        let message_reaction = serde_json::from_value(data.clone())?;
        Ok(Self::from_message_reaction(api, event_id, message_reaction))
    }

    /// Creates a new Reaction instance from the botgo-compatible DTO.
    pub fn from_message_reaction(
        api: BotApi,
        event_id: Option<String>,
        message_reaction: MessageReaction,
    ) -> Self {
        Self {
            api,
            event_id,
            user_id: message_reaction.user_id,
            channel_id: message_reaction.channel_id,
            guild_id: message_reaction.guild_id,
            emoji: message_reaction.emoji,
            target: message_reaction.target,
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Check if this is a message reaction
    pub fn is_message_reaction(&self) -> bool {
        self.target.target_type == ReactionTargetType::Message
    }

    /// Check if this is a post reaction
    pub fn is_post_reaction(&self) -> bool {
        self.target.target_type == ReactionTargetType::Post
    }

    /// Check if this is a comment reaction
    pub fn is_comment_reaction(&self) -> bool {
        self.target.target_type == ReactionTargetType::Comment
    }

    /// Check if this is a reply reaction
    pub fn is_reply_reaction(&self) -> bool {
        self.target.target_type == ReactionTargetType::Reply
    }

    /// Get the target ID
    pub fn target_id(&self) -> &str {
        &self.target.id
    }

    /// Get the emoji ID
    pub fn emoji_id(&self) -> &str {
        &self.emoji.id
    }
}

impl std::fmt::Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Reaction {{ user_id: {}, channel_id: {}, guild_id: {}, target_type: {:?}, event_id: {:?} }}",
            self.user_id, self.channel_id, self.guild_id, self.target.target_type, self.event_id
        )
    }
}

pub type ReactionUser = User;

/// Reaction users response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionUsers {
    /// List of users who reacted
    #[serde(default)]
    pub users: Vec<User>,
    /// Pagination cookie for next page
    pub cookie: Option<String>,
    /// Whether this is the last page
    #[serde(default)]
    pub is_end: bool,
}

pub type MessageReactionUsers = ReactionUsers;

/// Pager for message reaction users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageReactionPager {
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// Page size, 1-1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl MessageReactionPager {
    /// Creates a new message reaction pager.
    pub fn new(cookie: Option<impl Into<String>>, limit: Option<impl ToString>) -> Self {
        Self {
            cookie: cookie.map(Into::into),
            limit: limit.map(|value| value.to_string()),
        }
    }

    /// Converts the pager to query parameters.
    pub fn query_params(&self) -> std::collections::HashMap<String, String> {
        let mut query = std::collections::HashMap::new();
        if let Some(limit) = &self.limit {
            query.insert("limit".to_string(), limit.clone());
        }
        if let Some(cookie) = &self.cookie {
            query.insert("cookie".to_string(), cookie.clone());
        }
        query
    }

    /// Botgo-compatible query parameter accessor.
    #[allow(non_snake_case)]
    pub fn QueryParams(&self) -> std::collections::HashMap<String, String> {
        self.query_params()
    }
}

impl Pager for MessageReactionPager {
    fn query_params(&self) -> std::collections::HashMap<String, String> {
        MessageReactionPager::query_params(self)
    }
}

impl ReactionUsers {
    /// Create a new ReactionUsers instance from JSON data
    pub fn new(data: &Value) -> crate::Result<Self> {
        Ok(serde_json::from_value(data.clone())?)
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
        assert_eq!(i32::from(ReactionTargetType::Message), 0);
        assert_eq!(i32::from(ReactionTargetType::Post), 1);
        assert_eq!(i32::from(ReactionTargetType::Comment), 2);
        assert_eq!(i32::from(ReactionTargetType::Reply), 3);
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
        let emoji = Emoji::new("emoji123", 1);
        assert_eq!(emoji.id, "emoji123");
        assert_eq!(emoji.emoji_type, 1);
    }

    #[test]
    fn test_reaction_target_creation() {
        let target = ReactionTarget::new("target123", ReactionTargetType::Message);
        assert_eq!(target.id, "target123");
        assert_eq!(target.target_type, ReactionTargetType::Message);
    }

    #[test]
    fn test_reaction_user_creation() {
        let data = serde_json::json!({
            "id": "user123",
            "username": "testuser",
            "avatar": "https://example.com/avatar.png"
        });
        let user: ReactionUser = serde_json::from_value(data).unwrap();
        assert_eq!(user.id, "user123");
        assert_eq!(user.username, "testuser");
        assert_eq!(user.avatar, "https://example.com/avatar.png");
    }

    #[test]
    fn botgo_message_reaction_keeps_official_dto_shape() {
        let reaction = MessageReaction::new(
            "user-1",
            "channel-1",
            "guild-1",
            ReactionTarget::new("message-1", ReactionTargetType::Message),
            Emoji::new("43", 1),
        );
        let value = serde_json::to_value(&reaction).unwrap();

        assert_eq!(value["user_id"], "user-1");
        assert_eq!(value["channel_id"], "channel-1");
        assert_eq!(value["guild_id"], "guild-1");
        assert_eq!(value["target"]["id"], "message-1");
        assert_eq!(value["target"]["type"], 0);
        assert_eq!(value["emoji"]["id"], "43");
        assert_eq!(value["emoji"]["type"], 1);
        assert!(value.get("event_id").is_none());
    }

    #[test]
    fn botgo_reaction_pager_query_params() {
        let pager = MessageReactionPager::new(Some("cursor-1"), Some(20));
        let query = pager.QueryParams();

        assert_eq!(query.get("cookie").map(String::as_str), Some("cursor-1"));
        assert_eq!(query.get("limit").map(String::as_str), Some("20"));
    }
}
