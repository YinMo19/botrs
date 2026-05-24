//! Message setting data models for the QQ Guild Bot API.

use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Guild message frequency settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSetting {
    /// Whether creating direct messages is disabled.
    pub disable_create_dm: Option<bool>,
    /// Whether pushing messages is disabled.
    pub disable_push_msg: Option<bool>,
    /// Channel IDs covered by the setting.
    pub channel_ids: Option<Vec<Snowflake>>,
    /// Maximum number of pushed messages per channel.
    pub channel_push_max_num: Option<i32>,
}
