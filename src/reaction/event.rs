use serde::{Deserialize, Serialize};

use super::{Emoji, ReactionTarget};
use crate::models::Snowflake;

/// Message reaction DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageReaction {
    /// User ID who made the reaction.
    #[serde(default)]
    pub user_id: Snowflake,
    /// Channel ID where the reaction occurred.
    #[serde(default)]
    pub channel_id: Snowflake,
    /// Guild ID where the reaction occurred.
    #[serde(default)]
    pub guild_id: Snowflake,
    /// Target of the reaction.
    #[serde(default)]
    pub target: ReactionTarget,
    /// Emoji used for the reaction.
    #[serde(default)]
    pub emoji: Emoji,
}
