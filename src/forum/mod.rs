//! Forum-related functionality for QQ Bot
//!
//! This module provides structures for forum gateway events.

mod events;
mod open;

pub use events::{Post, PostInfo, Reply, ReplyInfo, Thread, ThreadInfo};
pub use open::{ForumAuditResult, OpenThread};

#[cfg(test)]
mod tests;
