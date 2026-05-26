//! Forum-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling forum threads,
//! posts, replies, and open forum events.

mod content;
mod events;
mod open;

pub use content::{
    Content, Cover, Elem, Format, Image, Paragraph, PlatImage, PlatVideo, Text, ThreadToCreate,
    Title, Url, Video,
};
pub use events::{ForumRsp, Post, PostInfo, PostThreadRsp, Reply, ReplyInfo, Thread, ThreadInfo};
pub use open::{ForumAuditResult, OpenThread};

#[cfg(test)]
mod tests;
