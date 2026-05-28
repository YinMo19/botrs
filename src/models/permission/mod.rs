//! Permission-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for managing API permissions and permission demands
//! in QQ Guild bots.

mod api;
mod channel;
mod demand;
mod identify;

pub use api::{APIPermission, APIPermissions};
pub use channel::{ChannelPermissions, ChannelRolePermissions, UpdateChannelPermissions};
pub use demand::APIPermissionDemand;
pub use identify::APIPermissionDemandIdentify;

#[cfg(test)]
mod tests;
