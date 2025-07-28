//! Robot (bot) related data models for the QQ Guild Bot API.

use crate::models::{HasId, HasName, Snowflake};
use serde::{Deserialize, Serialize};

/// Represents the bot/robot information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Robot {
    /// The bot's unique ID
    pub id: Snowflake,
    /// The bot's username
    pub username: String,
    /// The bot's avatar hash
    pub avatar: Option<String>,
    /// The bot's discriminator (usually #0000 for bots)
    pub discriminator: Option<String>,
    /// Whether this is a bot account
    #[serde(default = "default_true")]
    pub bot: bool,
    /// The bot's status
    pub status: Option<RobotStatus>,
    /// The bot's activity
    pub activity: Option<Activity>,
}

/// Helper function for default bot value
fn default_true() -> bool {
    true
}

impl Robot {
    /// Creates a new robot instance.
    pub fn new(id: impl Into<Snowflake>, username: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            avatar: None,
            discriminator: None,
            bot: true,
            status: None,
            activity: None,
        }
    }

    /// Gets the robot's avatar URL if it has one.
    pub fn avatar_url(&self) -> Option<String> {
        self.avatar.as_ref().map(|_hash| {
            format!(
                "https://thirdqq.qlogo.cn/headimg_dl?dst_uin={}&spec=640",
                self.id
            )
        })
    }

    /// Gets the robot's full tag (username#discriminator).
    pub fn tag(&self) -> String {
        match &self.discriminator {
            Some(disc) => format!("{}#{}", self.username, disc),
            None => self.username.clone(),
        }
    }

    /// Gets the robot's mention string.
    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }

    /// Sets the robot's status.
    pub fn with_status(mut self, status: RobotStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the robot's activity.
    pub fn with_activity(mut self, activity: Activity) -> Self {
        self.activity = Some(activity);
        self
    }
}

impl HasId for Robot {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.id)
    }
}

impl HasName for Robot {
    fn name(&self) -> &str {
        &self.username
    }
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

impl From<u8> for RobotStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Offline,
            1 => Self::Online,
            2 => Self::Idle,
            3 => Self::Dnd,
            4 => Self::Invisible,
            other => Self::Unknown(other),
        }
    }
}

impl From<RobotStatus> for u8 {
    fn from(status: RobotStatus) -> Self {
        match status {
            RobotStatus::Offline => 0,
            RobotStatus::Online => 1,
            RobotStatus::Idle => 2,
            RobotStatus::Dnd => 3,
            RobotStatus::Invisible => 4,
            RobotStatus::Unknown(value) => value,
        }
    }
}

impl RobotStatus {
    /// Returns true if the status indicates the robot is available.
    pub fn is_available(self) -> bool {
        matches!(self, RobotStatus::Online | RobotStatus::Idle)
    }

    /// Returns true if the status indicates the robot is busy.
    pub fn is_busy(self) -> bool {
        matches!(self, RobotStatus::Dnd)
    }

    /// Returns true if the status indicates the robot is offline.
    pub fn is_offline(self) -> bool {
        matches!(self, RobotStatus::Offline | RobotStatus::Invisible)
    }
}

/// Represents the robot's activity/presence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    /// The activity name
    pub name: String,
    /// The activity type
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    /// The activity URL (for streaming)
    pub url: Option<String>,
    /// Custom status text
    pub state: Option<String>,
    /// Activity details
    pub details: Option<String>,
}

impl Activity {
    /// Creates a new activity.
    pub fn new(name: impl Into<String>, activity_type: ActivityType) -> Self {
        Self {
            name: name.into(),
            activity_type,
            url: None,
            state: None,
            details: None,
        }
    }

    /// Creates a playing activity.
    pub fn playing(name: impl Into<String>) -> Self {
        Self::new(name, ActivityType::Playing)
    }

    /// Creates a streaming activity.
    pub fn streaming(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            activity_type: ActivityType::Streaming,
            url: Some(url.into()),
            state: None,
            details: None,
        }
    }

    /// Creates a listening activity.
    pub fn listening(name: impl Into<String>) -> Self {
        Self::new(name, ActivityType::Listening)
    }

    /// Creates a watching activity.
    pub fn watching(name: impl Into<String>) -> Self {
        Self::new(name, ActivityType::Watching)
    }

    /// Creates a custom activity.
    pub fn custom(name: impl Into<String>) -> Self {
        Self::new(name, ActivityType::Custom)
    }

    /// Sets the activity state.
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Sets the activity details.
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
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

impl From<u8> for ActivityType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Playing,
            1 => Self::Streaming,
            2 => Self::Listening,
            3 => Self::Watching,
            4 => Self::Custom,
            5 => Self::Competing,
            other => Self::Unknown(other),
        }
    }
}

impl From<ActivityType> for u8 {
    fn from(activity_type: ActivityType) -> Self {
        match activity_type {
            ActivityType::Playing => 0,
            ActivityType::Streaming => 1,
            ActivityType::Listening => 2,
            ActivityType::Watching => 3,
            ActivityType::Custom => 4,
            ActivityType::Competing => 5,
            ActivityType::Unknown(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_creation() {
        let robot = Robot::new("123456789", "TestBot");
        assert_eq!(robot.id, "123456789");
        assert_eq!(robot.username, "TestBot");
        assert!(robot.bot);
        assert_eq!(robot.mention(), "<@123456789>");
    }

    #[test]
    fn test_robot_tag() {
        let mut robot = Robot::new("123456789", "TestBot");
        assert_eq!(robot.tag(), "TestBot");

        robot.discriminator = Some("0001".to_string());
        assert_eq!(robot.tag(), "TestBot#0001");
    }

    #[test]
    fn test_robot_status() {
        assert!(RobotStatus::Online.is_available());
        assert!(RobotStatus::Idle.is_available());
        assert!(RobotStatus::Dnd.is_busy());
        assert!(RobotStatus::Offline.is_offline());
        assert!(RobotStatus::Invisible.is_offline());
    }

    #[test]
    fn test_activity_creation() {
        let activity = Activity::playing("Rust Programming");
        assert_eq!(activity.name, "Rust Programming");
        assert_eq!(activity.activity_type, ActivityType::Playing);

        let streaming = Activity::streaming("Live Coding", "https://example.com");
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
