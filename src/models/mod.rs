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
pub mod message_setting;
pub mod permission;
pub mod robot;
pub mod schedule;
pub mod user;
pub mod webhook;

// Re-export commonly used types
pub use announce::*;
pub use api::*;
pub use channel::*;
pub use emoji::*;
pub use gateway::*;
// Guild types are already exported by the specific re-exports below
pub use message::*;
pub use message_setting::*;
pub use permission::*;
pub use robot::*;
pub use schedule::*;
pub use user::*;
pub use webhook::*;

// Re-export specific types for convenience
pub use guild::{Guild, Member, Role};

use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, str::FromStr, time::Duration as StdDuration};

/// A snowflake ID used throughout the QQ Guild API.
pub type Snowflake = String;

/// Botgo-compatible timestamp string used by the API.
pub type Timestamp = String;

/// Botgo-compatible duration wrapper parsed from duration strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Duration(pub StdDuration);

impl Duration {
    pub const fn as_std(self) -> StdDuration {
        self.0
    }
}

impl From<StdDuration> for Duration {
    fn from(duration: StdDuration) -> Self {
        Self(duration)
    }
}

impl From<Duration> for StdDuration {
    fn from(duration: Duration) -> Self {
        duration.0
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        parse_duration(&value)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

fn parse_duration(value: &str) -> Result<StdDuration, String> {
    let value = value.trim_matches(['"', '\'']);
    let (number, unit) = value
        .find(|ch: char| !ch.is_ascii_digit())
        .map_or((value, ""), |index| value.split_at(index));
    let number = u64::from_str(number).map_err(|err| format!("invalid duration {value}: {err}"))?;
    match unit {
        "ns" => Ok(StdDuration::from_nanos(number)),
        "us" | "µs" => Ok(StdDuration::from_micros(number)),
        "ms" => Ok(StdDuration::from_millis(number)),
        "s" | "" => Ok(StdDuration::from_secs(number)),
        "m" => Ok(StdDuration::from_secs(number * 60)),
        "h" => Ok(StdDuration::from_secs(number * 60 * 60)),
        _ => Err(format!("unsupported duration unit {unit:?}")),
    }
}

/// Botgo-compatible pager trait.
pub trait Pager {
    fn query_params(&self) -> HashMap<String, String>;

    #[allow(non_snake_case)]
    fn QueryParams(&self) -> HashMap<String, String> {
        self.query_params()
    }
}

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

    #[test]
    fn test_botgo_duration_deserialization() {
        let duration: Duration = serde_json::from_str("\"1500ms\"").unwrap();
        assert_eq!(duration.as_std(), std::time::Duration::from_millis(1500));

        let duration: Duration = serde_json::from_str("\"2h\"").unwrap();
        assert_eq!(duration.as_std(), std::time::Duration::from_secs(7200));
    }
}
