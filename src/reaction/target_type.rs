use serde::{Deserialize, Serialize};

/// Reaction target type enumeration
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
#[repr(i32)]
pub enum ReactionTargetType {
    /// Message reaction
    #[default]
    Message = 0,
    /// Post reaction
    Post = 1,
    /// Comment reaction
    Comment = 2,
    /// Reply reaction
    Reply = 3,
    /// Unknown reaction target type.
    Unknown(i32),
}

pub const REACTION_TARGET_TYPE_MSG: i32 = 0;
pub const REACTION_TARGET_TYPE_FEED: i32 = 1;
pub const REACTION_TARGET_TYPE_COMMENT: i32 = 2;
pub const REACTION_TARGET_TYPE_REPLY: i32 = 3;

wire_enum!(ReactionTargetType, i32, Unknown, {
    Message = 0,
    Post = 1,
    Comment = 2,
    Reply = 3,
});
