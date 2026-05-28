//! Data models for the BotRS library.
//!
//! This module contains all the data structures used for interacting with the QQ Guild Bot API,
//! including messages, guilds, users, channels, and other entities.

pub mod announce;
pub mod api;
pub mod channel;
pub mod emoji;
pub mod gateway;
pub mod guild;
pub mod message;
pub mod permission;
pub mod robot;
pub mod schedule;
pub(crate) mod serde_helpers;
pub mod user;

// Re-export commonly used types
pub use announce::*;
pub use api::*;
pub use channel::*;
pub use emoji::*;
pub use gateway::*;
// Guild types are already exported by the specific re-exports below
pub use message::*;
pub use permission::*;
pub use robot::*;
pub use schedule::*;
pub use user::{Member as UserMember, Role as UserRole, User};

// Re-export specific types for convenience
pub use guild::{Guild, GuildRole, Member};

/// A snowflake ID used throughout the QQ Guild API.
pub type Snowflake = String;

/// Timestamp string used by the API.
pub type Timestamp = String;

#[cfg(test)]
mod tests {
    #[test]
    fn snowflake_and_timestamp_are_strings() {
        let snowflake: super::Snowflake = "123".to_string();
        let timestamp: super::Timestamp = "2024-01-01T00:00:00Z".to_string();
        assert_eq!(snowflake, "123");
        assert_eq!(timestamp, "2024-01-01T00:00:00Z");
    }
}
