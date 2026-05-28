//! Guild-related data models for the QQ Guild Bot API.
//!
//! This module contains guild types for the QQ Bot Open API.

mod member;
mod model;

pub use member::Member;
pub use model::Guild;

#[cfg(test)]
mod tests;
