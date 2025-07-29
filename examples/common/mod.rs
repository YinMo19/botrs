//! Common utilities for BotRS examples.
//!
//! This module provides shared functionality used across multiple examples,
//! including configuration loading and common helper functions.

pub mod config;

pub use config::*;


/// Initialize logging for examples with a reasonable default configuration.
pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,info")
        .init();
}

/// Initialize logging with a custom filter.
#[allow(unused)]
pub fn init_logging_with_filter(filter: &str) {
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
