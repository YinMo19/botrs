//! Intent flags for controlling which events the bot receives.
//!
//! This module provides the `Intents` struct and related functionality for managing
//! which gateway events your bot will receive. Intents act as a permission system
//! for gateway events.

mod core;
mod display;
mod ops;

pub use core::Intents;

#[cfg(test)]
mod tests;
