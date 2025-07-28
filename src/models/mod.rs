//! Data models for the BotRS library.
//!
//! This module contains all the data structures used for interacting with the QQ Guild Bot API,
//! including messages, guilds, users, channels, and other entities.

pub mod api;
pub mod channel;
pub mod gateway;
pub mod guild;
pub mod message;
pub mod robot;
pub mod user;

// Re-export commonly used types
pub use api::*;
pub use channel::*;
pub use gateway::*;
// Guild types are already exported by the specific re-exports below
pub use message::*;
pub use robot::*;
pub use user::*;

// Re-export specific types for convenience
pub use guild::{Guild, Member, Role};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A snowflake ID used throughout the QQ Guild API.
pub type Snowflake = String;

/// Represents a timestamp in the API.
pub type Timestamp = DateTime<Utc>;

/// Common trait for objects that have a snowflake ID.
pub trait HasId {
    /// Returns the object's ID, or None if not set.
    fn id(&self) -> Option<&Snowflake>;

    /// Returns the object's ID as a string, or empty string if not set.
    fn id_string(&self) -> String {
        self.id().cloned().unwrap_or_default()
    }
}

/// Common trait for objects that have a name.
pub trait HasName {
    /// Returns the object's name.
    fn name(&self) -> &str;
}

/// Represents the type of a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum ChannelType {
    /// Text channel
    Text = 0,
    /// Voice channel
    Voice = 2,
    /// Category channel
    Category = 4,
    /// Announcement channel
    Announcement = 5,
    /// Thread channel
    Thread = 10,
    /// Live channel
    Live = 12,
    /// Application channel
    Application = 13,
    /// Forum channel
    Forum = 15,
    /// Unknown channel type
    Unknown(u8),
}

impl From<u8> for ChannelType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Text,
            2 => Self::Voice,
            4 => Self::Category,
            5 => Self::Announcement,
            10 => Self::Thread,
            12 => Self::Live,
            13 => Self::Application,
            15 => Self::Forum,
            other => Self::Unknown(other),
        }
    }
}

impl From<ChannelType> for u8 {
    fn from(channel_type: ChannelType) -> Self {
        match channel_type {
            ChannelType::Text => 0,
            ChannelType::Voice => 2,
            ChannelType::Category => 4,
            ChannelType::Announcement => 5,
            ChannelType::Thread => 10,
            ChannelType::Live => 12,
            ChannelType::Application => 13,
            ChannelType::Forum => 15,
            ChannelType::Unknown(value) => value,
        }
    }
}

/// Represents the type of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum MessageType {
    /// Default message type
    Default = 0,
    /// System message
    System = 1,
    /// Unknown message type
    Unknown(u8),
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::System,
            other => Self::Unknown(other),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(message_type: MessageType) -> Self {
        match message_type {
            MessageType::Default => 0,
            MessageType::System => 1,
            MessageType::Unknown(value) => value,
        }
    }
}

/// Represents a color value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color(pub u32);

impl Color {
    /// Creates a new color from RGB values.
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Creates a new color from a hex value.
    pub const fn from_hex(hex: u32) -> Self {
        Self(hex)
    }

    /// Gets the red component.
    pub const fn r(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Gets the green component.
    pub const fn g(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Gets the blue component.
    pub const fn b(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Gets the raw hex value.
    pub const fn hex(self) -> u32 {
        self.0
    }

    // Common colors
    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const GREEN: Color = Color::from_rgb(0, 255, 0);
    pub const BLUE: Color = Color::from_rgb(0, 0, 255);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
    pub const YELLOW: Color = Color::from_rgb(255, 255, 0);
    pub const CYAN: Color = Color::from_rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:06X}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_type_conversion() {
        assert_eq!(ChannelType::from(0), ChannelType::Text);
        assert_eq!(u8::from(ChannelType::Text), 0);

        assert_eq!(ChannelType::from(99), ChannelType::Unknown(99));
        assert_eq!(u8::from(ChannelType::Unknown(99)), 99);
    }

    #[test]
    fn test_color() {
        let red = Color::from_rgb(255, 0, 0);
        assert_eq!(red.r(), 255);
        assert_eq!(red.g(), 0);
        assert_eq!(red.b(), 0);
        assert_eq!(red.hex(), 0xFF0000);

        let color = Color::from_hex(0x123456);
        assert_eq!(color.r(), 0x12);
        assert_eq!(color.g(), 0x34);
        assert_eq!(color.b(), 0x56);

        assert_eq!(format!("{}", Color::RED), "#FF0000");
    }
}
