use serde::{Deserialize, Serialize};

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
    /// Unknown reaction target type.
    Unknown(i32),
}

impl Default for ReactionTargetType {
    fn default() -> Self {
        Self::Message
    }
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

wire_enum!(ReactionTargetType, i32, Unknown, {
    Message = 0,
    Post = 1,
    Comment = 2,
    Reply = 3,
});
