//! Interaction-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling user interactions,
//! including button clicks, command interactions, and other interactive elements.

use crate::api::BotApi;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Interaction type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Interaction data type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
pub const ACTION_TYPE_SEND_ARK: crate::models::message::ActionType = 0;
#[allow(non_upper_case_globals)]
pub const ActionTypeSendARK: crate::models::message::ActionType = ACTION_TYPE_SEND_ARK;

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

/// Resolved interaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolved {
    /// Search keyword
    pub keyword: Option<String>,
    /// Button ID (for button interactions)
    pub button_id: Option<String>,
    /// Button data
    pub button_data: Option<String>,
    /// Message ID
    pub message_id: Option<String>,
    /// User ID
    pub user_id: Option<String>,
    /// Request payload
    pub request: Option<String>,
    /// Member nickname
    pub member_nick: Option<String>,
    /// Feature ID
    pub feature_id: Option<String>,
    /// Message feedback option
    pub feedback_opt: Option<String>,
    /// Whether feedback option is checked
    pub checked: Option<i32>,
}

impl Resolved {
    /// Create a new Resolved instance from JSON data
    pub fn new(data: &Value) -> Self {
        Self {
            keyword: data
                .get("keyword")
                .and_then(|v| v.as_str())
                .map(String::from),
            button_id: data
                .get("button_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            button_data: data
                .get("button_data")
                .and_then(|v| v.as_str())
                .map(String::from),
            message_id: data
                .get("message_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            user_id: data
                .get("user_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            request: data
                .get("request")
                .and_then(|v| v.as_str())
                .map(String::from),
            member_nick: data
                .get("member_nick")
                .and_then(|v| v.as_str())
                .map(String::from),
            feature_id: data
                .get("feature_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            feedback_opt: data
                .get("feedback_opt")
                .and_then(|v| v.as_str())
                .map(String::from),
            checked: data
                .get("checked")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
        }
    }
}

/// Interaction data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionData {
    /// Interaction name
    pub name: Option<String>,
    /// Data type
    pub data_type: Option<InteractionDataType>,
    /// Resolved data
    pub resolved: Resolved,
}

impl InteractionData {
    /// Create a new InteractionData instance from JSON data
    pub fn new(data: &Value) -> Self {
        Self {
            name: data.get("name").and_then(|v| v.as_str()).map(String::from),
            data_type: data
                .get("type")
                .and_then(|v| v.as_u64())
                .map(|v| InteractionDataType::from(v as u8)),
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
    pub keyword: Option<String>,
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
    pub layout_type: Option<u32>,
    /// Action type
    pub action_type: Option<u32>,
    /// Layout title
    pub title: Option<String>,
    /// Search records
    #[serde(default)]
    pub records: Vec<SearchRecord>,
}

/// Search result record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchRecord {
    /// Cover URL
    pub cover: Option<String>,
    /// Title
    pub title: Option<String>,
    /// Tips
    pub tips: Option<String>,
    /// Target URL
    pub url: Option<String>,
}

/// Interaction structure representing user interactions
#[derive(Debug, Clone, Serialize)]
pub struct Interaction {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Interaction ID
    pub id: Option<String>,
    /// Application ID
    pub application_id: Option<String>,
    /// Interaction type
    pub interaction_type: Option<InteractionType>,
    /// Scene identifier
    pub scene: Option<String>,
    /// Chat type
    pub chat_type: Option<u64>,
    /// Event ID
    pub event_id: Option<String>,
    /// Interaction data
    pub data: InteractionData,
    /// Guild ID
    pub guild_id: Option<String>,
    /// Channel ID
    pub channel_id: Option<String>,
    /// User OpenID
    pub user_openid: Option<String>,
    /// Group OpenID
    pub group_openid: Option<String>,
    /// Group member OpenID
    pub group_member_openid: Option<String>,
    /// Timestamp
    pub timestamp: Option<String>,
    /// Version
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
        self.data.resolved.button_id.as_deref()
    }

    /// Get the button data if this is a button interaction
    pub fn button_data(&self) -> Option<&str> {
        self.data.resolved.button_data.as_deref()
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
}
