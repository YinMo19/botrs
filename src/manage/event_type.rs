use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
