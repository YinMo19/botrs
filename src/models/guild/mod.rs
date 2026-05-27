//! Guild-related data models for the QQ Guild Bot API.
//!
//! This module contains guild types for the QQ Bot Open API.

mod member;
mod model;
mod mute;
mod pager;
mod role;

pub use member::*;
pub use model::*;
pub use mute::*;
pub use pager::*;
pub use role::*;

#[cfg(test)]
mod tests;
