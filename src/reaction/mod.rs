//! Reaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling message reactions,
//! emoji reactions, and reaction-related events.

mod emoji;
mod event;
mod pager;
mod reaction;
mod target;
mod target_type;
mod users;

pub use emoji::Emoji;
pub use event::MessageReaction;
pub use pager::MessageReactionPager;
pub use reaction::Reaction;
pub use target::ReactionTarget;
pub use target_type::{
    REACTION_TARGET_TYPE_COMMENT, REACTION_TARGET_TYPE_FEED, REACTION_TARGET_TYPE_MSG,
    REACTION_TARGET_TYPE_REPLY, ReactionTargetType, ReactionTargetTypeComment,
    ReactionTargetTypeFeed, ReactionTargetTypeMsg, ReactionTargetTypeReply,
};
pub use users::ReactionUsers;

#[cfg(test)]
mod tests;
