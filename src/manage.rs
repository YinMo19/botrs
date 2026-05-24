//! Management event functionality for QQ Bot
//!
//! This module provides structures and implementations for handling management events,
//! including group and C2C (client-to-client) management operations.

use crate::api::BotApi;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Group management event structure
#[derive(Debug, Clone, Serialize)]
pub struct GroupManageEvent {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Event ID
    pub event_id: Option<String>,
    /// Timestamp of the event
    pub timestamp: Option<u64>,
    /// Group OpenID
    pub group_openid: Option<String>,
    /// Operator member OpenID
    pub op_member_openid: Option<String>,
}

impl GroupManageEvent {
    /// Create a new GroupManageEvent instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `event_id` - Optional event ID
    /// * `data` - Management event data from the gateway
    pub fn new(
        api: BotApi,
        event_id: Option<String>,
        data: &HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            api,
            event_id,
            timestamp: data.get("timestamp").and_then(|v| v.as_u64()),
            group_openid: data
                .get("group_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            op_member_openid: data
                .get("op_member_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Get the event timestamp as a formatted string
    pub fn formatted_timestamp(&self) -> Option<String> {
        self.timestamp.map(|ts| {
            let datetime = chrono::DateTime::from_timestamp(ts as i64, 0).unwrap_or_default();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }
}

impl std::fmt::Display for GroupManageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GroupManageEvent {{ event_id: {:?}, timestamp: {:?}, group_openid: {:?}, op_member_openid: {:?} }}",
            self.event_id, self.timestamp, self.group_openid, self.op_member_openid
        )
    }
}

/// C2C (Client-to-Client) management event structure
#[derive(Debug, Clone, Serialize)]
pub struct C2CManageEvent {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Event ID
    pub event_id: Option<String>,
    /// Timestamp of the event
    pub timestamp: Option<u64>,
    /// User OpenID
    pub openid: Option<String>,
    /// User nickname
    pub nick: Option<String>,
    /// User avatar URL
    pub avatar: Option<String>,
}

impl C2CManageEvent {
    /// Create a new C2CManageEvent instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `event_id` - Optional event ID
    /// * `data` - Management event data from the gateway
    pub fn new(
        api: BotApi,
        event_id: Option<String>,
        data: &HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            api,
            event_id,
            timestamp: data.get("timestamp").and_then(|v| v.as_u64()),
            openid: data
                .get("openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            nick: data.get("nick").and_then(|v| v.as_str()).map(String::from),
            avatar: data
                .get("avatar")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Get the event timestamp as a formatted string
    pub fn formatted_timestamp(&self) -> Option<String> {
        self.timestamp.map(|ts| {
            let datetime = chrono::DateTime::from_timestamp(ts as i64, 0).unwrap_or_default();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }
}

impl std::fmt::Display for C2CManageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "C2CManageEvent {{ event_id: {:?}, timestamp: {:?}, openid: {:?}, nick: {:?}, avatar: {:?} }}",
            self.event_id, self.timestamp, self.openid, self.nick, self.avatar
        )
    }
}

/// Event emitted when a user enters AIO.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnterAioEvent {
    /// User OpenID
    pub user_openid: Option<String>,
    /// Source from which the user entered AIO
    pub from_source: Option<String>,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl EnterAioEvent {
    /// Creates a new EnterAioEvent from gateway data.
    pub fn new(event_id: Option<String>, data: &serde_json::Value) -> Self {
        Self {
            user_openid: data
                .get("user_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            from_source: data
                .get("from_source")
                .and_then(|v| v.as_str())
                .map(String::from),
            event_id,
        }
    }
}

/// Subscribe message status event data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeMessageStatusData {
    /// Group OpenID, present for group subscription messages
    pub group_openid: Option<String>,
    /// User OpenID, present for C2C subscription messages
    pub openid: Option<String>,
    /// Template authorization results
    #[serde(default)]
    pub result: Vec<SubscribeMsgTemplateResult>,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl SubscribeMessageStatusData {
    /// Creates a subscribe status event from gateway data.
    pub fn new(event_id: Option<String>, data: &serde_json::Value) -> Self {
        let mut event = serde_json::from_value::<Self>(data.clone()).unwrap_or_default();
        event.event_id = event_id;
        event
    }
}

/// Subscribe template authorization result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeMsgTemplateResult {
    /// Official template ID
    pub template_id: Option<i32>,
    /// Custom template ID
    pub custom_template_id: Option<String>,
    /// Authorization operation, 1 allow and 2 reject
    pub op: Option<u32>,
    /// Subscription ID
    pub subscribe_id: Option<String>,
    /// Status update timestamp
    pub update_ts: Option<u64>,
}

/// Management event type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManageEventType {
    /// Group add robot event
    GroupAddRobot,
    /// Group delete robot event
    GroupDelRobot,
    /// Group message reject event
    GroupMsgReject,
    /// Group message receive event
    GroupMsgReceive,
    /// Friend add event
    FriendAdd,
    /// Friend delete event
    FriendDel,
    /// C2C message reject event
    C2CMsgReject,
    /// C2C message receive event
    C2CMsgReceive,
}

impl ManageEventType {
    /// Convert event type to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GroupAddRobot => "group_add_robot",
            Self::GroupDelRobot => "group_del_robot",
            Self::GroupMsgReject => "group_msg_reject",
            Self::GroupMsgReceive => "group_msg_receive",
            Self::FriendAdd => "friend_add",
            Self::FriendDel => "friend_del",
            Self::C2CMsgReject => "c2c_msg_reject",
            Self::C2CMsgReceive => "c2c_msg_receive",
        }
    }

    /// Check if this is a group-related event
    pub fn is_group_event(&self) -> bool {
        matches!(
            self,
            Self::GroupAddRobot
                | Self::GroupDelRobot
                | Self::GroupMsgReject
                | Self::GroupMsgReceive
        )
    }

    /// Check if this is a C2C-related event
    pub fn is_c2c_event(&self) -> bool {
        matches!(
            self,
            Self::FriendAdd | Self::FriendDel | Self::C2CMsgReject | Self::C2CMsgReceive
        )
    }
}

impl FromStr for ManageEventType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "group_add_robot" => Ok(Self::GroupAddRobot),
            "group_del_robot" => Ok(Self::GroupDelRobot),
            "group_msg_reject" => Ok(Self::GroupMsgReject),
            "group_msg_receive" => Ok(Self::GroupMsgReceive),
            "friend_add" => Ok(Self::FriendAdd),
            "friend_del" => Ok(Self::FriendDel),
            "c2c_msg_reject" => Ok(Self::C2CMsgReject),
            "c2c_msg_receive" => Ok(Self::C2CMsgReceive),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manage_event_type_from_str() {
        assert_eq!(
            "group_add_robot".parse::<ManageEventType>(),
            Ok(ManageEventType::GroupAddRobot)
        );
        assert_eq!(
            "friend_add".parse::<ManageEventType>(),
            Ok(ManageEventType::FriendAdd)
        );
        assert_eq!("invalid".parse::<ManageEventType>(), Err(()));
    }

    #[test]
    fn test_manage_event_type_as_str() {
        assert_eq!(ManageEventType::GroupAddRobot.as_str(), "group_add_robot");
        assert_eq!(ManageEventType::FriendAdd.as_str(), "friend_add");
    }

    #[test]
    fn test_is_group_event() {
        assert!(ManageEventType::GroupAddRobot.is_group_event());
        assert!(!ManageEventType::FriendAdd.is_group_event());
    }

    #[test]
    fn test_is_c2c_event() {
        assert!(ManageEventType::FriendAdd.is_c2c_event());
        assert!(!ManageEventType::GroupAddRobot.is_c2c_event());
    }
}
