use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Management event type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManageEventType {
    /// Group add robot event
    #[serde(rename = "group_add_robot")]
    GroupAddRobot,
    /// Group delete robot event
    #[serde(rename = "group_del_robot")]
    GroupDelRobot,
    /// Group message reject event
    #[serde(rename = "group_msg_reject")]
    GroupMsgReject,
    /// Group message receive event
    #[serde(rename = "group_msg_receive")]
    GroupMsgReceive,
    /// Friend add event
    #[serde(rename = "friend_add")]
    FriendAdd,
    /// Friend delete event
    #[serde(rename = "friend_del")]
    FriendDel,
    /// C2C message reject event
    #[serde(rename = "c2c_msg_reject")]
    C2CMsgReject,
    /// C2C message receive event
    #[serde(rename = "c2c_msg_receive")]
    C2CMsgReceive,
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
