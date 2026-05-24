//! Message setting data models for the QQ Guild Bot API.

use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Guild message frequency settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageSetting {
    /// Whether creating direct messages is disabled.
    #[serde(default)]
    pub disable_create_dm: bool,
    /// Whether pushing messages is disabled.
    #[serde(default)]
    pub disable_push_msg: bool,
    /// Channel IDs covered by the setting.
    #[serde(default)]
    pub channel_ids: Vec<Snowflake>,
    /// Maximum number of pushed messages per channel.
    #[serde(default)]
    pub channel_push_max_num: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn botgo_message_setting_uses_required_zero_value_fields() {
        let setting: MessageSetting = serde_json::from_value(serde_json::json!({})).unwrap();

        assert!(!setting.disable_create_dm);
        assert!(!setting.disable_push_msg);
        assert!(setting.channel_ids.is_empty());
        assert_eq!(setting.channel_push_max_num, 0);
    }

    #[test]
    fn botgo_message_setting_keeps_official_json_shape() {
        let setting = MessageSetting {
            disable_create_dm: true,
            disable_push_msg: false,
            channel_ids: vec!["channel-1".to_string(), "channel-2".to_string()],
            channel_push_max_num: 10,
        };
        let value = serde_json::to_value(&setting).unwrap();

        assert_eq!(value["disable_create_dm"], true);
        assert_eq!(value["disable_push_msg"], false);
        assert_eq!(value["channel_ids"][0], "channel-1");
        assert_eq!(value["channel_push_max_num"], 10);
    }
}
