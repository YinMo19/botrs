//! Announcement-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for creating and managing guild announcements,
//! including both message-type and recommended channel announcements.

use crate::models::{HasId, Snowflake};
use serde::{Deserialize, Serialize};

/// Represents a recommended channel for guild announcements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendChannel {
    /// The ID of the channel to recommend
    pub channel_id: Option<Snowflake>,
    /// Description or introduction for the recommended channel
    pub introduce: Option<String>,
}

impl RecommendChannel {
    /// Creates a new RecommendChannel instance.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The ID of the channel to recommend
    /// * `introduce` - Optional description for the channel
    pub fn new(channel_id: impl Into<String>, introduce: Option<String>) -> Self {
        Self {
            channel_id: Some(channel_id.into()),
            introduce,
        }
    }
}

impl HasId for RecommendChannel {
    fn id(&self) -> Option<&Snowflake> {
        self.channel_id.as_ref()
    }
}

/// Types of announcements that can be created.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum AnnouncesType {
    /// Member announcement
    Member = 0,
    /// Welcome announcement
    Welcome = 1,
    /// Unknown announcement type
    Unknown(u8),
}

impl From<u8> for AnnouncesType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Member,
            1 => Self::Welcome,
            other => Self::Unknown(other),
        }
    }
}

impl From<AnnouncesType> for u8 {
    fn from(announces_type: AnnouncesType) -> Self {
        match announces_type {
            AnnouncesType::Member => 0,
            AnnouncesType::Welcome => 1,
            AnnouncesType::Unknown(value) => value,
        }
    }
}

impl From<AnnouncesType> for i32 {
    fn from(announces_type: AnnouncesType) -> Self {
        u8::from(announces_type) as i32
    }
}

/// Represents a guild announcement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Announce {
    /// The guild ID where the announcement is created
    pub guild_id: Option<Snowflake>,
    /// The channel ID for message-type announcements
    pub channel_id: Option<Snowflake>,
    /// The message ID for message-type announcements
    pub message_id: Option<Snowflake>,
    /// The type of announcement
    pub announces_type: Option<AnnouncesType>,
    /// List of recommended channels for recommended channel announcements
    pub recommend_channels: Option<Vec<RecommendChannel>>,
}

impl Announce {
    /// Creates a new message-type announcement.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID where the announcement is created
    /// * `channel_id` - The channel ID containing the message
    /// * `message_id` - The message ID to announce
    pub fn new_message(
        guild_id: impl Into<String>,
        channel_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            guild_id: Some(guild_id.into()),
            channel_id: Some(channel_id.into()),
            message_id: Some(message_id.into()),
            announces_type: None,
            recommend_channels: None,
        }
    }

    /// Creates a new recommended channel announcement.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID where the announcement is created
    /// * `announces_type` - The type of announcement
    /// * `recommend_channels` - List of channels to recommend
    pub fn new_recommend(
        guild_id: impl Into<String>,
        announces_type: AnnouncesType,
        recommend_channels: Vec<RecommendChannel>,
    ) -> Self {
        Self {
            guild_id: Some(guild_id.into()),
            channel_id: None,
            message_id: None,
            announces_type: Some(announces_type),
            recommend_channels: Some(recommend_channels),
        }
    }

    /// Returns true if this is a message-type announcement.
    pub fn is_message_type(&self) -> bool {
        self.message_id.is_some()
    }

    /// Returns true if this is a recommended channel announcement.
    pub fn is_recommend_type(&self) -> bool {
        self.recommend_channels.is_some()
    }

    /// Gets the number of recommended channels.
    pub fn recommend_channel_count(&self) -> usize {
        self.recommend_channels
            .as_ref()
            .map(|channels| channels.len())
            .unwrap_or(0)
    }
}

impl HasId for Announce {
    fn id(&self) -> Option<&Snowflake> {
        self.guild_id.as_ref()
    }
}

impl std::fmt::Display for Announce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_message_type() {
            write!(
                f,
                "MessageAnnounce {{ guild_id: {:?}, channel_id: {:?}, message_id: {:?} }}",
                self.guild_id, self.channel_id, self.message_id
            )
        } else {
            write!(
                f,
                "RecommendAnnounce {{ guild_id: {:?}, type: {:?}, channels: {} }}",
                self.guild_id,
                self.announces_type,
                self.recommend_channel_count()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announces_type_conversion() {
        assert_eq!(AnnouncesType::from(0), AnnouncesType::Member);
        assert_eq!(AnnouncesType::from(1), AnnouncesType::Welcome);
        assert_eq!(u8::from(AnnouncesType::Member), 0);
        assert_eq!(u8::from(AnnouncesType::Welcome), 1);
        assert_eq!(i32::from(AnnouncesType::Member), 0);
        assert_eq!(i32::from(AnnouncesType::Welcome), 1);

        assert_eq!(AnnouncesType::from(99), AnnouncesType::Unknown(99));
        assert_eq!(u8::from(AnnouncesType::Unknown(99)), 99);
    }

    #[test]
    fn test_recommend_channel() {
        let channel = RecommendChannel::new("123456", Some("Test channel".to_string()));
        assert_eq!(channel.channel_id, Some("123456".to_string()));
        assert_eq!(channel.introduce, Some("Test channel".to_string()));
        assert_eq!(channel.id(), Some(&"123456".to_string()));
    }

    #[test]
    fn test_message_announce() {
        let announce = Announce::new_message("guild123", "channel456", "message789");
        assert_eq!(announce.guild_id, Some("guild123".to_string()));
        assert_eq!(announce.channel_id, Some("channel456".to_string()));
        assert_eq!(announce.message_id, Some("message789".to_string()));
        assert!(announce.is_message_type());
        assert!(!announce.is_recommend_type());
    }

    #[test]
    fn test_recommend_announce() {
        let channels = vec![
            RecommendChannel::new("channel1", Some("First channel".to_string())),
            RecommendChannel::new("channel2", Some("Second channel".to_string())),
        ];
        let announce = Announce::new_recommend("guild123", AnnouncesType::Welcome, channels);
        assert_eq!(announce.guild_id, Some("guild123".to_string()));
        assert_eq!(announce.announces_type, Some(AnnouncesType::Welcome));
        assert!(!announce.is_message_type());
        assert!(announce.is_recommend_type());
        assert_eq!(announce.recommend_channel_count(), 2);
    }

    #[test]
    fn test_announce_display() {
        let message_announce = Announce::new_message("guild1", "channel1", "message1");
        let display = format!("{}", message_announce);
        assert!(display.contains("MessageAnnounce"));
        assert!(display.contains("guild1"));

        let recommend_announce = Announce::new_recommend(
            "guild2",
            AnnouncesType::Member,
            vec![RecommendChannel::new("channel1", None)],
        );
        let display = format!("{}", recommend_announce);
        assert!(display.contains("RecommendAnnounce"));
        assert!(display.contains("guild2"));
        assert!(display.contains("channels: 1"));
    }
}
