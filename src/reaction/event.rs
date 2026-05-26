use serde::{Deserialize, Serialize};

use super::{Emoji, ReactionTarget};
use crate::models::Snowflake;

/// Message reaction DTO.
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
