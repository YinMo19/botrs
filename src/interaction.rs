//! Interaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling user interactions,
//! including button clicks, command interactions, and other interactive elements.

use crate::api::BotApi;
use crate::models::serde_helpers::is_default;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// Interaction type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractionType {
    /// Ping interaction
    Ping = 1,
    /// Application command interaction
    ApplicationCommand = 2,
    /// HTTP proxy interaction
    HttpProxy = 10,
    /// Inline keyboard interaction
    InlineKeyboard = 11,
}

#[allow(non_upper_case_globals)]
pub const InteractionTypePing: InteractionType = InteractionType::Ping;
#[allow(non_upper_case_globals)]
pub const InteractionTypeCommand: InteractionType = InteractionType::ApplicationCommand;

impl From<u8> for InteractionType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Ping,
            2 => Self::ApplicationCommand,
            10 => Self::HttpProxy,
            11 => Self::InlineKeyboard,
            _ => Self::Ping, // Default fallback
        }
    }
}

impl Serialize for InteractionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for InteractionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(u8::deserialize(deserializer)?))
    }
}

/// Interaction data type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractionDataType {
    /// Chat input search
    ChatInputSearch = 9,
    /// HTTP proxy
    HttpProxy = 10,
    /// Inline keyboard button click
    InlineKeyboardButtonClick = 11,
    /// C2C callback command click
    CallbackCommandClick = 12,
    /// Message feedback click
    MessageFeedbackClick = 13,
    /// Clear session click
    ClearSessionClick = 14,
}

#[allow(non_upper_case_globals)]
pub const InteractionDataTypeChatSearch: InteractionDataType = InteractionDataType::ChatInputSearch;
#[allow(non_upper_case_globals)]
pub const InteractionDataTypeInlineKeyboardClick: InteractionDataType =
    InteractionDataType::InlineKeyboardButtonClick;
#[allow(non_upper_case_globals)]
pub const InteractionDataTypeCallbackCommandClick: InteractionDataType =
    InteractionDataType::CallbackCommandClick;
#[allow(non_upper_case_globals)]
pub const InteractionDataTypeMessageFeedbackClick: InteractionDataType =
    InteractionDataType::MessageFeedbackClick;
#[allow(non_upper_case_globals)]
pub const InteractionDataTypeClearSessionClick: InteractionDataType =
    InteractionDataType::ClearSessionClick;

pub type LayoutType = u32;
pub const LAYOUT_TYPE_IMAGE_TEXT: LayoutType = 0;
#[allow(non_upper_case_globals)]
pub const LayoutTypeImageText: LayoutType = LAYOUT_TYPE_IMAGE_TEXT;
pub type ActionType = u32;
pub const ACTION_TYPE_SEND_ARK: ActionType = 0;
#[allow(non_upper_case_globals)]
pub const ActionTypeSendARK: ActionType = ACTION_TYPE_SEND_ARK;

impl From<u8> for InteractionDataType {
    fn from(value: u8) -> Self {
        match value {
            9 => Self::ChatInputSearch,
            10 => Self::HttpProxy,
            11 => Self::InlineKeyboardButtonClick,
            12 => Self::CallbackCommandClick,
            13 => Self::MessageFeedbackClick,
            14 => Self::ClearSessionClick,
            _ => Self::ChatInputSearch, // Default fallback
        }
    }
}

impl Serialize for InteractionDataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for InteractionDataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(u8::deserialize(deserializer)?))
    }
}

fn string_field(data: &Value, key: &str) -> String {
    data.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

/// Resolved interaction data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Resolved {
    /// Search keyword
    #[serde(default)]
    pub keyword: String,
    /// User ID
    #[serde(default)]
    pub user_id: String,
    /// Request payload
    #[serde(default)]
    pub request: String,
    /// Message ID
    #[serde(default)]
    pub message_id: String,
    /// Member nickname
    #[serde(default)]
    pub member_nick: String,
    /// Button data
    #[serde(default)]
    pub button_data: String,
    /// Button ID (for button interactions)
    #[serde(default)]
    pub button_id: String,
    /// Feature ID
    #[serde(default)]
    pub feature_id: String,
    /// Message feedback option
    #[serde(default)]
    pub feedback_opt: String,
    /// Whether feedback option is checked
    #[serde(default)]
    pub checked: i32,
}

impl Resolved {
    /// Create a new Resolved instance from JSON data
    pub fn new(data: &Value) -> Self {
        Self {
            keyword: string_field(data, "keyword"),
            user_id: string_field(data, "user_id"),
            request: string_field(data, "request"),
            message_id: string_field(data, "message_id"),
            member_nick: string_field(data, "member_nick"),
            button_data: string_field(data, "button_data"),
            button_id: string_field(data, "button_id"),
            feature_id: string_field(data, "feature_id"),
            feedback_opt: string_field(data, "feedback_opt"),
            checked: data
                .get("checked")
                .and_then(Value::as_i64)
                .map_or(0, |value| value as i32),
        }
    }
}

/// Interaction data structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InteractionData {
    /// Interaction name
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Data type
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<InteractionDataType>,
    /// Resolved data
    #[serde(default, skip_serializing_if = "is_default")]
    pub resolved: Resolved,
}

impl InteractionData {
    /// Create a new InteractionData instance from JSON data
    pub fn new(data: &Value) -> Self {
        Self {
            name: string_field(data, "name"),
            data_type: data
                .get("type")
                .and_then(Value::as_u64)
                .map(|value| InteractionDataType::from(value as u8)),
            resolved: Resolved::new(
                data.get("resolved")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
        }
    }
}

/// Search input resolved data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchInputResolved {
    /// Search keyword
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub keyword: String,
}

/// Search interaction response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchRsp {
    /// Search layouts
    #[serde(default)]
    pub layouts: Vec<SearchLayout>,
}

/// Search result layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchLayout {
    /// Layout type
    #[serde(rename = "LayoutType")]
    pub layout_type: LayoutType,
    /// Action type
    #[serde(rename = "ActionType")]
    pub action_type: ActionType,
    /// Layout title
    #[serde(rename = "Title")]
    pub title: String,
    /// Search records
    #[serde(rename = "Records", default)]
    pub records: Vec<SearchRecord>,
}

/// Search result record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchRecord {
    /// Cover URL
    #[serde(default)]
    pub cover: String,
    /// Title
    #[serde(default)]
    pub title: String,
    /// Tips
    #[serde(default)]
    pub tips: String,
    /// Target URL
    #[serde(rename = "url", alias = "URL", default)]
    pub url: String,
}

/// Interaction structure representing user interactions
#[derive(Debug, Clone, Serialize)]
pub struct Interaction {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Create a new Interaction instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `event_id` - Optional event ID
    /// * `data` - Interaction payload data from the gateway
    pub fn new(api: BotApi, event_id: Option<String>, data: &Value) -> Self {
        Self {
            api,
            event_id,
            id: data.get("id").and_then(|v| v.as_str()).map(String::from),
            application_id: data.get("application_id").and_then(|v| {
                v.as_str()
                    .map(String::from)
                    .or_else(|| v.as_u64().map(|value| value.to_string()))
            }),
            interaction_type: data
                .get("type")
                .and_then(|v| v.as_u64())
                .map(|v| InteractionType::from(v as u8)),
            scene: data.get("scene").and_then(|v| v.as_str()).map(String::from),
            chat_type: data.get("chat_type").and_then(|v| v.as_u64()),
            data: InteractionData::new(
                data.get("data")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            user_openid: data
                .get("user_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            group_openid: data
                .get("group_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            group_member_openid: data
                .get("group_member_openid")
                .and_then(|v| v.as_str())
                .map(String::from),
            timestamp: data.get("timestamp").and_then(|v| {
                v.as_str()
                    .map(String::from)
                    .or_else(|| v.as_u64().map(|value| value.to_string()))
            }),
            version: data.get("version").and_then(|v| v.as_u64()),
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Check if this is a button interaction
    pub fn is_button_interaction(&self) -> bool {
        matches!(
            self.data.data_type,
            Some(InteractionDataType::InlineKeyboardButtonClick)
        )
    }

    /// Check if this is a command interaction
    pub fn is_command_interaction(&self) -> bool {
        matches!(
            self.interaction_type,
            Some(InteractionType::ApplicationCommand)
        )
    }

    /// Get the button ID if this is a button interaction
    pub fn button_id(&self) -> Option<&str> {
        (!self.data.resolved.button_id.is_empty()).then_some(self.data.resolved.button_id.as_str())
    }

    /// Get the button data if this is a button interaction
    pub fn button_data(&self) -> Option<&str> {
        (!self.data.resolved.button_data.is_empty())
            .then_some(self.data.resolved.button_data.as_str())
    }
}

impl std::fmt::Display for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Interaction {{ id: {:?}, type: {:?}, scene: {:?}, chat_type: {:?}, event_id: {:?} }}",
            self.id, self.interaction_type, self.scene, self.chat_type, self.event_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_type() {
        assert_eq!(InteractionType::Ping as u8, 1);
        assert_eq!(InteractionType::ApplicationCommand as u8, 2);
        assert_eq!(InteractionType::HttpProxy as u8, 10);
        assert_eq!(InteractionType::InlineKeyboard as u8, 11);
    }

    #[test]
    fn test_interaction_data_type() {
        assert_eq!(InteractionDataType::ChatInputSearch as u8, 9);
        assert_eq!(InteractionDataType::HttpProxy as u8, 10);
        assert_eq!(InteractionDataType::InlineKeyboardButtonClick as u8, 11);
        assert_eq!(InteractionDataType::CallbackCommandClick as u8, 12);
        assert_eq!(InteractionDataType::MessageFeedbackClick as u8, 13);
        assert_eq!(InteractionDataType::ClearSessionClick as u8, 14);
    }

    #[test]
    fn test_interaction_type_from() {
        assert_eq!(InteractionType::from(1), InteractionType::Ping);
        assert_eq!(
            InteractionType::from(2),
            InteractionType::ApplicationCommand
        );
        assert_eq!(InteractionType::from(10), InteractionType::HttpProxy);
        assert_eq!(InteractionType::from(11), InteractionType::InlineKeyboard);
    }

    #[test]
    fn test_interaction_data_type_from() {
        assert_eq!(
            InteractionDataType::from(9),
            InteractionDataType::ChatInputSearch
        );
        assert_eq!(
            InteractionDataType::from(10),
            InteractionDataType::HttpProxy
        );
        assert_eq!(
            InteractionDataType::from(11),
            InteractionDataType::InlineKeyboardButtonClick
        );
        assert_eq!(
            InteractionDataType::from(12),
            InteractionDataType::CallbackCommandClick
        );
        assert_eq!(
            InteractionDataType::from(13),
            InteractionDataType::MessageFeedbackClick
        );
        assert_eq!(
            InteractionDataType::from(14),
            InteractionDataType::ClearSessionClick
        );
    }

    #[test]
    fn interaction_types_serialize_as_numeric_wire_values() {
        assert_eq!(
            serde_json::to_value(InteractionType::ApplicationCommand).unwrap(),
            serde_json::json!(2)
        );
        assert_eq!(
            serde_json::from_value::<InteractionType>(serde_json::json!(11)).unwrap(),
            InteractionType::InlineKeyboard
        );
        assert_eq!(
            serde_json::to_value(InteractionDataType::ChatInputSearch).unwrap(),
            serde_json::json!(9)
        );
        assert_eq!(
            serde_json::from_value::<InteractionDataType>(serde_json::json!(14)).unwrap(),
            InteractionDataType::ClearSessionClick
        );
    }

    #[test]
    fn interaction_payload_uses_expected_type_fields() {
        let interaction = Interaction::new(
            BotApi::new(crate::http::HttpClient::new(30, false).unwrap()),
            Some("event-1".to_string()),
            &serde_json::json!({
                "id": "interaction-1",
                "application_id": "app-1",
                "type": 2,
                "data": {
                    "name": "search",
                    "type": 9,
                    "resolved": {
                        "keyword": "botrs"
                    }
                },
                "version": 1
            }),
        );

        let value = serde_json::to_value(&interaction).unwrap();
        assert_eq!(value["type"], serde_json::json!(2));
        assert_eq!(value["data"]["type"], serde_json::json!(9));
        assert!(value.get("interaction_type").is_none());
        assert!(value["data"].get("data_type").is_none());
    }

    #[test]
    fn resolved_uses_required_zero_value_fields() {
        let resolved: Resolved = serde_json::from_value(serde_json::json!({
            "button_id": "btn-1",
            "checked": 1
        }))
        .unwrap();

        assert_eq!(resolved.keyword, "");
        assert_eq!(resolved.user_id, "");
        assert_eq!(resolved.request, "");
        assert_eq!(resolved.message_id, "");
        assert_eq!(resolved.member_nick, "");
        assert_eq!(resolved.button_data, "");
        assert_eq!(resolved.button_id, "btn-1");
        assert_eq!(resolved.feature_id, "");
        assert_eq!(resolved.feedback_opt, "");
        assert_eq!(resolved.checked, 1);

        let value = serde_json::to_value(Resolved::default()).unwrap();
        assert_eq!(value["keyword"], "");
        assert_eq!(value["button_id"], "");
        assert_eq!(value["checked"], 0);
    }

    #[test]
    fn search_dtos_keep_official_json_shape() {
        let resolved = SearchInputResolved {
            keyword: "botrs".to_string(),
        };
        let resolved_value = serde_json::to_value(&resolved).unwrap();
        assert_eq!(resolved_value["keyword"], "botrs");

        let empty_resolved = serde_json::to_value(SearchInputResolved::default()).unwrap();
        assert!(empty_resolved.get("keyword").is_none());

        let response = SearchRsp {
            layouts: vec![SearchLayout {
                layout_type: LayoutTypeImageText,
                action_type: ActionTypeSendARK,
                title: "docs".to_string(),
                records: vec![SearchRecord {
                    cover: "https://example.com/cover.png".to_string(),
                    title: "BotRS".to_string(),
                    tips: "Rust SDK".to_string(),
                    url: "https://example.com".to_string(),
                }],
            }],
        };
        let value = serde_json::to_value(&response).unwrap();

        assert_eq!(value["layouts"][0]["LayoutType"], 0);
        assert_eq!(value["layouts"][0]["ActionType"], 0);
        assert_eq!(value["layouts"][0]["Title"], "docs");
        assert_eq!(
            value["layouts"][0]["Records"][0]["cover"],
            "https://example.com/cover.png"
        );
        assert_eq!(value["layouts"][0]["Records"][0]["title"], "BotRS");
        assert_eq!(value["layouts"][0]["Records"][0]["tips"], "Rust SDK");
        assert_eq!(
            value["layouts"][0]["Records"][0]["url"],
            "https://example.com"
        );
        assert!(value["layouts"][0].get("layout_type").is_none());
        assert!(value["layouts"][0].get("action_type").is_none());
    }
}
