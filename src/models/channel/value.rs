use super::{ChannelSubType, ChannelType, PrivateType, SpeakPermission};
use crate::models::Snowflake;
use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

/// Channel value object used when creating or modifying a channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelValueObject {
    /// Channel name
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub name: Option<String>,
    /// Channel type
    #[serde(rename = "type", skip_serializing_if = "option_is_none_or_default")]
    pub channel_type: Option<ChannelType>,
    /// Sort position
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub position: Option<i64>,
    /// Parent channel ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub parent_id: Option<Snowflake>,
    /// Owner ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub owner_id: Option<Snowflake>,
    /// Channel subtype
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub sub_type: Option<ChannelSubType>,
    /// Channel visibility type
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub private_type: Option<PrivateType>,
    /// Private channel member IDs
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub private_user_ids: Option<Vec<String>>,
    /// Speak permission
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub speak_permission: Option<SpeakPermission>,
    /// Application ID for application channels
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub application_id: Option<Snowflake>,
    /// Channel permissions
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub permissions: Option<String>,
    /// Operator user ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub op_user_id: Option<Snowflake>,
}

impl ChannelValueObject {
    /// Creates a channel value object with the required create-channel fields.
    pub fn new(
        name: impl Into<String>,
        channel_type: ChannelType,
        sub_type: ChannelSubType,
    ) -> Self {
        Self {
            name: Some(name.into()),
            channel_type: Some(channel_type),
            sub_type: Some(sub_type),
            ..Default::default()
        }
    }
}
