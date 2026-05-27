//! Channel-related data models for the QQ Guild Bot API.
//!
//! This module contains channel types for the QQ Bot Open API.

mod enums;
mod model;
mod permissions;
mod value;

pub use enums::*;
pub use model::*;
pub use permissions::*;
pub use value::*;

#[cfg(test)]
mod tests;
