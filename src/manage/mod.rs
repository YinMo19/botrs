//! Management event functionality for QQ Bot
//!
//! This module provides structures and implementations for handling management events,
//! including group and C2C (client-to-client) management operations.

mod aio;
mod c2c;
mod group;
mod subscribe;

pub use aio::EnterAioEvent;
pub use c2c::{C2CFriendData, C2CManageEvent};
pub use group::GroupManageEvent;
pub use subscribe::{SubscribeMessageStatusData, SubscribeMsgTemplateResult};

#[cfg(test)]
mod tests;
