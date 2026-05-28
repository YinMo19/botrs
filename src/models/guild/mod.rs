//! Guild-related data models for the QQ Guild Bot API.
//!
//! This module contains guild types for the QQ Bot Open API.

mod member;
mod model;
mod role;

pub use member::Member;
pub use model::Guild;
pub use role::GuildRole;

#[cfg(test)]
mod tests;
