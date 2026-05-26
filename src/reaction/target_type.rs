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
