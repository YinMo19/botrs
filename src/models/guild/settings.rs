use serde::{Deserialize, Serialize};

/// Message history deletion range used when removing a guild member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "i32", into = "i32")]
#[repr(i32)]
pub enum DeleteHistoryMsgDays {
    /// Delete no message history.
    #[default]
    None = 0,
    /// Delete three days of message history.
    ThreeDays = 3,
    /// Delete seven days of message history.
    SevenDays = 7,
    /// Delete fifteen days of message history.
    FifteenDays = 15,
    /// Delete thirty days of message history.
    ThirtyDays = 30,
    /// Delete all message history.
    All = -1,
    /// Unknown platform value.
    Unknown(i32) = -9999,
}

wire_enum!(DeleteHistoryMsgDays, i32, Unknown, {
    None = 0,
    ThreeDays = 3,
    SevenDays = 7,
    FifteenDays = 15,
    ThirtyDays = 30,
    All = -1,
});

/// Options for deleting a guild member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MemberDeleteOptions {
    #[serde(default)]
    pub add_blacklist: bool,
    #[serde(default)]
    pub delete_history_msg_days: DeleteHistoryMsgDays,
}

/// Guild message push setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageSetting {
    #[serde(default)]
    pub disable_create_dm: bool,
    #[serde(default)]
    pub disable_push_msg: bool,
    #[serde(default)]
    pub channel_ids: Vec<String>,
    #[serde(default)]
    pub channel_push_max_num: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_delete_options_keep_default_fields_on_the_wire() {
        let value = serde_json::to_value(MemberDeleteOptions::default()).unwrap();

        assert_eq!(value["add_blacklist"], false);
        assert_eq!(value["delete_history_msg_days"], 0);
    }
}
