use super::{InteractionData, InteractionType};
use crate::models::serde_helpers::{deserialize_string_or_number, is_default};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Interaction structure representing user interactions
#[derive(Debug, Clone, Serialize)]
pub struct Interaction {
    /// Interaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Application ID
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub timestamp: Option<String>,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

impl Interaction {
    /// Builds an interaction event from the gateway payload.
    pub(crate) fn new(event_id: Option<String>, data: &Value) -> Self {
        let wire: InteractionWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            event_id,
            id: wire.id,
            application_id: wire.application_id,
            interaction_type: wire.interaction_type,
            scene: wire.scene,
            chat_type: wire.chat_type,
            data: wire.data,
            guild_id: wire.guild_id,
            channel_id: wire.channel_id,
            user_openid: wire.user_openid,
            group_openid: wire.group_openid,
            group_member_openid: wire.group_member_openid,
            timestamp: wire.timestamp,
            version: wire.version,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct InteractionWire {
    #[serde(default)]
    id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_number")]
    application_id: Option<String>,
    #[serde(rename = "type", default)]
    interaction_type: Option<InteractionType>,
    #[serde(default)]
    scene: Option<String>,
    #[serde(default)]
    chat_type: Option<u64>,
    #[serde(default)]
    data: InteractionData,
    #[serde(default)]
    guild_id: Option<String>,
    #[serde(default)]
    channel_id: Option<String>,
    #[serde(default)]
    user_openid: Option<String>,
    #[serde(default)]
    group_openid: Option<String>,
    #[serde(default)]
    group_member_openid: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_number")]
    timestamp: Option<String>,
    #[serde(default)]
    version: Option<u64>,
}
