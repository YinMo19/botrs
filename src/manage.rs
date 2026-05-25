//! Management event functionality for QQ Bot
//!
//! This module provides structures and implementations for handling management events,
//! including group and C2C (client-to-client) management operations.

use crate::api::BotApi;
use serde::{Deserialize, Serialize};
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
    pub fn new(api: BotApi, event_id: Option<String>, data: &serde_json::Value) -> Self {
        let wire: GroupManageWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            event_id,
            timestamp: wire.timestamp,
            group_openid: wire.group_openid,
            op_member_openid: wire.op_member_openid,
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

#[derive(Debug, Default, Deserialize)]
struct GroupManageWire {
    #[serde(default)]
    timestamp: Option<u64>,
    #[serde(default)]
    group_openid: Option<String>,
    #[serde(default)]
    op_member_openid: Option<String>,
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

/// C2C friend event payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct C2CFriendData {
    /// User OpenID
    pub openid: String,
    /// Add/delete timestamp
    #[serde(default)]
    pub timestamp: u64,
    /// User nickname, currently filled by upstream when available
    #[serde(default)]
    pub nick: String,
    /// User avatar URL, currently filled by upstream when available
    #[serde(default)]
    pub avatar: String,
}

impl C2CFriendData {
    /// Creates a friend event DTO from gateway data.
    pub fn new(data: &serde_json::Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
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
    pub fn new(api: BotApi, event_id: Option<String>, data: &serde_json::Value) -> Self {
        let wire: C2CManageWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            event_id,
            timestamp: wire.timestamp,
            openid: wire.openid,
            nick: wire.nick,
            avatar: wire.avatar,
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

#[derive(Debug, Default, Deserialize)]
struct C2CManageWire {
    #[serde(default)]
    timestamp: Option<u64>,
    #[serde(default)]
    openid: Option<String>,
    #[serde(default)]
    nick: Option<String>,
    #[serde(default)]
    avatar: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EnterAioEvent {
    /// User OpenID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user_openid: String,
    /// Source from which the user entered AIO
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from_source: String,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

pub type EnterAIO = EnterAioEvent;

impl EnterAioEvent {
    /// Creates a new EnterAioEvent from gateway data.
    pub fn new(event_id: Option<String>, data: &serde_json::Value) -> Self {
        let mut event = serde_json::from_value::<Self>(data.clone()).unwrap_or_default();
        event.event_id = event_id;
        event
    }
}

/// Subscribe message status event data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubscribeMessageStatusData {
    /// Group OpenID, present for group subscription messages
    #[serde(default)]
    pub group_openid: String,
    /// User OpenID, present for C2C subscription messages
    #[serde(default)]
    pub openid: String,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubscribeMsgTemplateResult {
    /// Official template ID
    #[serde(default)]
    pub template_id: i32,
    /// Custom template ID
    #[serde(default)]
    pub custom_template_id: String,
    /// Authorization operation, 1 allow and 2 reject
    #[serde(default)]
    pub op: u32,
    /// Subscription ID
    #[serde(default)]
    pub subscribe_id: String,
    /// Status update timestamp
    #[serde(default)]
    pub update_ts: u64,
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

    #[test]
    fn enter_aio_uses_zero_value_omitempty_shape() {
        let event = EnterAioEvent::new(
            Some("event-1".to_string()),
            &serde_json::json!({
                "user_openid": "user-1",
                "from_source": "profile"
            }),
        );

        assert_eq!(event.user_openid, "user-1");
        assert_eq!(event.from_source, "profile");
        assert_eq!(event.event_id.as_deref(), Some("event-1"));

        let value = serde_json::to_value(&event).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "user_openid": "user-1",
                "from_source": "profile"
            })
        );

        let empty = serde_json::to_value(EnterAioEvent::default()).unwrap();
        assert_eq!(empty, serde_json::json!({}));
    }

    #[test]
    fn subscribe_message_status_uses_required_zero_value_fields() {
        let event = SubscribeMessageStatusData::new(
            Some("event-1".to_string()),
            &serde_json::json!({
                "group_openid": "group-1",
                "openid": "user-1",
                "result": [{
                    "template_id": 1,
                    "custom_template_id": "custom-1",
                    "op": 2,
                    "subscribe_id": "sub-1",
                    "update_ts": 1710000000
                }]
            }),
        );

        assert_eq!(event.group_openid, "group-1");
        assert_eq!(event.openid, "user-1");
        assert_eq!(event.event_id.as_deref(), Some("event-1"));
        assert_eq!(event.result[0].template_id, 1);
        assert_eq!(event.result[0].custom_template_id, "custom-1");
        assert_eq!(event.result[0].op, 2);
        assert_eq!(event.result[0].subscribe_id, "sub-1");
        assert_eq!(event.result[0].update_ts, 1_710_000_000);

        let value = serde_json::to_value(&event).unwrap();
        assert_eq!(value["group_openid"], "group-1");
        assert_eq!(value["openid"], "user-1");
        assert_eq!(value["result"][0]["template_id"], 1);
        assert_eq!(value["result"][0]["custom_template_id"], "custom-1");
        assert_eq!(value["result"][0]["op"], 2);
        assert_eq!(value["result"][0]["subscribe_id"], "sub-1");
        assert_eq!(value["result"][0]["update_ts"], 1_710_000_000_u64);
        assert!(value.get("event_id").is_none());

        let empty = serde_json::to_value(SubscribeMessageStatusData::default()).unwrap();
        assert_eq!(empty["group_openid"], "");
        assert_eq!(empty["openid"], "");
        assert_eq!(empty["result"], serde_json::json!([]));
    }
}
