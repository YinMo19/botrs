use super::{InteractionData, InteractionType};
use crate::models::serde_helpers::{deserialize_string_or_number, is_default};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Interaction structure representing user interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// Interaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Application ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "deserialize_string_or_number")]
    pub application_id: Option<String>,
    /// Interaction type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<InteractionType>,
    /// Scene identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// Chat type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<u64>,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
    /// Interaction data
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub data: InteractionData,
    /// Guild ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    /// Channel ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// User OpenID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_openid: Option<String>,
    /// Group OpenID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_openid: Option<String>,
    /// Group member OpenID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_member_openid: Option<String>,
    /// Timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "deserialize_string_or_number")]
    pub timestamp: Option<String>,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

impl Interaction {
    /// Builds an interaction event from the gateway payload.
    pub(crate) fn new(event_id: Option<String>, data: Value) -> serde_json::Result<Self> {
        let mut event: Self = serde_json::from_value(data)?;
        event.event_id = event_id;
        Ok(event)
    }
}
