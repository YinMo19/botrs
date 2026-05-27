use crate::models::Snowflake;
use serde::Serialize;
use serde_json::Value;

use super::{Emoji, MessageReaction, ReactionTarget, ReactionTargetType};

/// Reaction structure representing emoji reactions to messages or posts
#[derive(Debug, Clone, Serialize)]
pub struct Reaction {
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
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl Reaction {
    /// Parses a reaction event from the gateway payload.
    pub(crate) fn new(event_id: Option<String>, data: &Value) -> crate::Result<Self> {
        let message_reaction = serde_json::from_value(data.clone())?;
        Ok(Self::from_message_reaction(event_id, message_reaction))
    }

    /// Creates a new Reaction instance from the structured DTO.
    pub fn from_message_reaction(
        event_id: Option<String>,
        message_reaction: MessageReaction,
    ) -> Self {
        Self {
            event_id,
            user_id: message_reaction.user_id,
            channel_id: message_reaction.channel_id,
            guild_id: message_reaction.guild_id,
            emoji: message_reaction.emoji,
            target: message_reaction.target,
        }
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
