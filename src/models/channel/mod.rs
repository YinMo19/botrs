//! Channel-related data models for the QQ Guild Bot API.
//!
//! This module contains channel types for the QQ Bot Open API.

mod enums;
mod model;

pub use enums::*;
pub use model::*;

#[cfg(test)]
mod tests;
