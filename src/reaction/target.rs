use serde::{Deserialize, Serialize};

use super::ReactionTargetType;

/// Reaction target structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReactionTarget {
    /// Target ID
    #[serde(default)]
    pub id: String,
    /// Target type (message, post, comment, reply)
    #[serde(default, rename = "type")]
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
