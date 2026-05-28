//! Announcement-related data structures for the QQ Guild Bot API.
//!
//! This module contains structures for creating and managing guild announcements,
//! including both message-type and recommended channel announcements.

use crate::models::{HasId, Snowflake};
use serde::{Deserialize, Serialize};

/// Represents a recommended channel for guild announcements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RecommendChannel {
    /// The ID of the channel to recommend
    #[serde(default)]
    pub channel_id: Snowflake,
    /// Description or introduction for the recommended channel
    #[serde(default)]
    pub introduce: String,
}

impl HasId for RecommendChannel {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.channel_id)
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

wire_enum!(AnnouncesType, u8, Unknown, {
    Member = 0,
    Welcome = 1,
});

impl From<AnnouncesType> for i32 {
    fn from(announces_type: AnnouncesType) -> Self {
        u8::from(announces_type) as i32
    }
}

/// Represents a guild announcement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Announce {
    /// The guild ID where the announcement is created
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The channel ID for message-type announcements
    #[serde(default)]
    pub channel_id: Snowflake,
    /// The message ID for message-type announcements
    #[serde(default)]
    pub message_id: Snowflake,
    /// The type of announcement
    #[serde(default)]
    pub announces_type: u32,
    /// List of recommended channels for recommended channel announcements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommend_channels: Vec<RecommendChannel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChannelAnnouncesToCreate {
    pub message_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildAnnouncesToCreate {
    pub channel_id: String,
    pub message_id: String,
    pub announces_type: u32,
    #[serde(default)]
    pub recommend_channels: Vec<RecommendChannel>,
}

impl HasId for Announce {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.guild_id)
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
        let channel = RecommendChannel {
            channel_id: "123456".to_string(),
            introduce: "Test channel".to_string(),
        };
        assert_eq!(channel.channel_id, "123456");
        assert_eq!(channel.introduce, "Test channel");
        assert_eq!(channel.id(), Some(&"123456".to_string()));
    }

    #[test]
    fn test_message_announce() {
        let announce = Announce {
            guild_id: "guild123".to_string(),
            channel_id: "channel456".to_string(),
            message_id: "message789".to_string(),
            announces_type: u8::from(AnnouncesType::Member) as u32,
            recommend_channels: Vec::new(),
        };
        assert_eq!(announce.guild_id, "guild123");
        assert_eq!(announce.channel_id, "channel456");
        assert_eq!(announce.message_id, "message789");
        assert!(announce.recommend_channels.is_empty());
    }

    #[test]
    fn test_recommend_announce() {
        let channels = vec![
            RecommendChannel {
                channel_id: "channel1".to_string(),
                introduce: "First channel".to_string(),
            },
            RecommendChannel {
                channel_id: "channel2".to_string(),
                introduce: "Second channel".to_string(),
            },
        ];
        let announce = Announce {
            guild_id: "guild123".to_string(),
            channel_id: String::new(),
            message_id: String::new(),
            announces_type: u8::from(AnnouncesType::Welcome) as u32,
            recommend_channels: channels,
        };
        assert_eq!(announce.guild_id, "guild123");
        assert_eq!(announce.announces_type, 1);
        assert_eq!(announce.message_id, "");
        assert_eq!(announce.recommend_channels.len(), 2);
    }

    #[test]
    fn announce_uses_required_zero_value_fields() {
        let announce: Announce = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(announce.guild_id, "");
        assert_eq!(announce.channel_id, "");
        assert_eq!(announce.message_id, "");
        assert_eq!(announce.announces_type, 0);
        assert!(announce.recommend_channels.is_empty());

        let value = serde_json::to_value(&announce).unwrap();
        assert_eq!(value["guild_id"], "");
        assert_eq!(value["channel_id"], "");
        assert_eq!(value["message_id"], "");
        assert_eq!(value["announces_type"], 0);
        assert!(value.get("recommend_channels").is_none());
    }

    #[test]
    fn announce_keeps_official_json_shape() {
        let announce = Announce {
            guild_id: "guild-1".to_string(),
            channel_id: "channel-1".to_string(),
            message_id: "message-1".to_string(),
            announces_type: 1,
            recommend_channels: vec![RecommendChannel {
                channel_id: "channel-2".to_string(),
                introduce: "intro".to_string(),
            }],
        };
        let value = serde_json::to_value(&announce).unwrap();

        assert_eq!(value["guild_id"], "guild-1");
        assert_eq!(value["channel_id"], "channel-1");
        assert_eq!(value["message_id"], "message-1");
        assert_eq!(value["announces_type"], 1);
        assert_eq!(value["recommend_channels"][0]["channel_id"], "channel-2");
        assert_eq!(value["recommend_channels"][0]["introduce"], "intro");
    }

    #[test]
    fn announce_create_bodies_keep_required_fields() {
        let channel = serde_json::to_value(ChannelAnnouncesToCreate::default()).unwrap();
        assert_eq!(
            channel,
            serde_json::json!({
                "message_id": ""
            })
        );

        let guild = serde_json::to_value(GuildAnnouncesToCreate::default()).unwrap();
        assert_eq!(
            guild,
            serde_json::json!({
                "channel_id": "",
                "message_id": "",
                "announces_type": 0,
                "recommend_channels": []
            })
        );
    }
}
