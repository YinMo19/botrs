//! Interaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling user interactions,
//! including button clicks, command interactions, and other interactive elements.

mod data;
mod model;
mod search;
mod types;

pub use data::{InteractionData, Resolved};
pub use model::Interaction;
pub use search::{SearchInputResolved, SearchLayout, SearchRecord, SearchRsp};
pub use types::{InteractionDataType, InteractionType};

#[cfg(test)]
mod tests;
