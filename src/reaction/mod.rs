//! Reaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling message reactions,
//! emoji reactions, and reaction-related events.

mod event;
mod model;
mod target;
mod target_type;
mod users;

pub use event::MessageReaction;
pub use model::Reaction;
pub use target::ReactionTarget;
pub use target_type::ReactionTargetType;
pub use users::ReactionUsers;

#[cfg(test)]
mod tests;
