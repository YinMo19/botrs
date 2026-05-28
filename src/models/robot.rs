//! Robot (bot) related data models for the QQ Guild Bot API.

use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Represents the bot/robot information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Robot {
    /// The bot's unique ID
    pub id: Snowflake,
    /// The bot's username
    pub username: String,
    /// The bot's avatar hash
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// The bot's discriminator (usually #0000 for bots)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    /// Whether this is a bot account
    #[serde(default = "default_true")]
    pub bot: bool,
    /// The bot's status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RobotStatus>,
    /// The bot's activity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
}

/// Helper function for default bot value
fn default_true() -> bool {
    true
}

/// Represents the robot's online status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum RobotStatus {
    /// Offline
    Offline,
    /// Online and active
    Online,
    /// Away/idle
    Idle,
    /// Do not disturb
    Dnd,
    /// Invisible/offline
    Invisible,
    /// Unknown status
    Unknown(u8),
}

wire_enum!(RobotStatus, u8, Unknown, {
    Offline = 0,
    Online = 1,
    Idle = 2,
    Dnd = 3,
    Invisible = 4,
});

/// Represents the robot's activity/presence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    /// The activity name
    pub name: String,
    /// The activity type
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    /// The activity URL (for streaming)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Custom status text
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Activity details
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// The type of activity the robot is performing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum ActivityType {
    /// Playing a game
    Playing = 0,
    /// Streaming
    Streaming = 1,
    /// Listening to something
    Listening = 2,
    /// Watching something
    Watching = 3,
    /// Custom status
    Custom = 4,
    /// Competing in something
    Competing = 5,
    /// Unknown activity type
    Unknown(u8),
}

wire_enum!(ActivityType, u8, Unknown, {
    Playing = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
});

#[cfg(test)]
mod tests {
    use super::*;

    fn test_robot(id: &str, username: &str) -> Robot {
        Robot {
            id: id.to_string(),
            username: username.to_string(),
            avatar: None,
            discriminator: None,
            bot: true,
            status: None,
            activity: None,
        }
    }

    #[test]
    fn test_robot_creation() {
        let robot = test_robot("123456789", "TestBot");
        assert_eq!(robot.id, "123456789");
        assert_eq!(robot.username, "TestBot");
        assert!(robot.bot);
    }

    #[test]
    fn robot_omits_absent_extension_fields_like_ready_user() {
        let robot = test_robot("123456789", "TestBot");
        assert_eq!(
            serde_json::to_value(&robot).unwrap(),
            serde_json::json!({
                "id": "123456789",
                "username": "TestBot",
                "bot": true
            })
        );

        let mut robot = test_robot("123456789", "TestBot");
        robot.status = Some(RobotStatus::Online);
        robot.activity = Some(Activity {
            name: "Rust".to_string(),
            activity_type: ActivityType::Playing,
            url: None,
            state: None,
            details: None,
        });
        assert_eq!(
            serde_json::to_value(&robot).unwrap(),
            serde_json::json!({
                "id": "123456789",
                "username": "TestBot",
                "bot": true,
                "status": 1,
                "activity": {
                    "name": "Rust",
                    "type": 0
                }
            })
        );
    }

    #[test]
    fn test_robot_tag() {
        let mut robot = test_robot("123456789", "TestBot");
        assert!(robot.discriminator.is_none());

        robot.discriminator = Some("0001".to_string());
        assert_eq!(robot.discriminator.as_deref(), Some("0001"));
    }

    #[test]
    fn test_robot_status() {
        assert_eq!(RobotStatus::from(0), RobotStatus::Offline);
        assert_eq!(RobotStatus::from(1), RobotStatus::Online);
        assert_eq!(RobotStatus::from(2), RobotStatus::Idle);
        assert_eq!(RobotStatus::from(3), RobotStatus::Dnd);
        assert_eq!(RobotStatus::from(4), RobotStatus::Invisible);
    }

    #[test]
    fn test_activity_creation() {
        let activity = Activity {
            name: "Rust Programming".to_string(),
            activity_type: ActivityType::Playing,
            url: None,
            state: None,
            details: None,
        };
        assert_eq!(activity.name, "Rust Programming");
        assert_eq!(activity.activity_type, ActivityType::Playing);

        let streaming = Activity {
            name: "Live Coding".to_string(),
            activity_type: ActivityType::Streaming,
            url: Some("https://example.com".to_string()),
            state: None,
            details: None,
        };
        assert_eq!(streaming.activity_type, ActivityType::Streaming);
        assert_eq!(streaming.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_activity_type_conversion() {
        assert_eq!(ActivityType::from(0), ActivityType::Playing);
        assert_eq!(u8::from(ActivityType::Playing), 0);

        assert_eq!(ActivityType::from(99), ActivityType::Unknown(99));
        assert_eq!(u8::from(ActivityType::Unknown(99)), 99);
    }
}
