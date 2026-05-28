use crate::models::Snowflake;
use crate::models::emoji::Emoji;
use serde::Serialize;
use serde_json::Value;

use super::{MessageReaction, ReactionTarget};

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

    /// Creates a Reaction from the structured gateway DTO.
    pub(crate) fn from_message_reaction(
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
}
