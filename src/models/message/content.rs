use crate::models::serde_helpers::{option_is_none_or_default, serialize_option_as_default};
use serde::{Deserialize, Serialize};

/// Ark template message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ark {
    /// Template ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub template_id: Option<u32>,
    /// Keyboard data
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub kv: Option<Vec<ArkKv>>,
}

/// Ark key-value pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArkKv {
    /// Key
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub key: Option<String>,
    /// Value
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub value: Option<String>,
    /// Object data
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub obj: Option<Vec<ArkObj>>,
}

/// Ark object structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArkObj {
    /// Object key-value pairs
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub obj_kv: Option<Vec<ArkObjKv>>,
}

/// Ark object key-value pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArkObjKv {
    /// Key
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub key: Option<String>,
    /// Value
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub value: Option<String>,
}

/// Embed message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Embed {
    /// Title of the embed
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub title: Option<String>,
    /// Description of the embed
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub description: Option<String>,
    /// Message list summary/popup content
    #[serde(default)]
    pub prompt: String,
    /// Thumbnail information
    #[serde(default)]
    pub thumbnail: EmbedThumbnail,
    /// Fields in the embed
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub fields: Option<Vec<EmbedField>>,
}

/// Embed thumbnail structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EmbedThumbnail {
    /// Thumbnail URL
    #[serde(default)]
    pub url: String,
}

/// Embed field structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmbedField {
    /// Field name
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub name: Option<String>,
    /// Field value
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub value: Option<String>,
}

/// Keyboard message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Keyboard {
    /// Keyboard template ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub id: Option<String>,
    /// Keyboard content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<KeyboardContent>,
}

/// Keyboard content structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardContent {
    /// Rows of buttons
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub rows: Option<Vec<KeyboardRow>>,
    /// Keyboard style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<KeyboardStyle>,
}

/// Keyboard style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardStyle {
    /// Font size
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub font_size: Option<String>,
}

/// Keyboard row structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardRow {
    /// Buttons in this row
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub buttons: Option<Vec<KeyboardButton>>,
}

/// Keyboard button structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButton {
    /// Button ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub id: Option<String>,
    /// Button render data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_data: Option<KeyboardButtonRenderData>,
    /// Button action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<KeyboardButtonAction>,
    /// Button group ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub group_id: Option<String>,
}

/// Keyboard button render data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButtonRenderData {
    /// Button label
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub label: Option<String>,
    /// Button visited label
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub visited_label: Option<String>,
    /// Button style
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub style: Option<u32>,
}

/// Keyboard button action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButtonAction {
    /// Action type
    #[serde(rename = "type", skip_serializing_if = "option_is_none_or_default")]
    pub action_type: Option<u32>,
    /// Permission data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<KeyboardButtonPermission>,
    /// Click limit per user
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub click_limit: Option<u32>,
    /// Action data
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub data: Option<String>,
    /// Enter flag
    #[serde(default)]
    pub enter: bool,
    /// Whether to show channel selection when at-bot action is used
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub at_bot_show_channel_list: Option<bool>,
    /// Subscribe button data
    #[serde(default)]
    pub subscribe_data: KeyboardSubscribeData,
    /// Secondary confirmation modal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modal: Option<KeyboardModal>,
}

/// Keyboard button permission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardButtonPermission {
    /// Permission type
    #[serde(rename = "type", skip_serializing_if = "option_is_none_or_default")]
    pub permission_type: Option<u32>,
    /// Specify role IDs
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub specify_role_ids: Option<Vec<String>>,
    /// Specify user IDs
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub specify_user_ids: Option<Vec<String>>,
}

/// Keyboard subscribe data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyboardSubscribeData {
    /// Subscription template IDs
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub template_ids: Option<Vec<KeyboardTemplateId>>,
}

/// Keyboard template ID wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeyboardTemplateId {
    /// Official template ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub template_id: Option<u32>,
    /// Custom template ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub custom_template_id: Option<String>,
}

/// Keyboard secondary confirmation modal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeyboardModal {
    /// Confirmation content
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub content: Option<String>,
    /// Confirm button text
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub confirm_text: Option<String>,
    /// Cancel button text
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub cancel_text: Option<String>,
}

/// Keyboard payload structure for API requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardPayload {
    /// Keyboard content
    pub content: serde_json::Value,
}

impl From<KeyboardPayload> for Keyboard {
    fn from(payload: KeyboardPayload) -> Self {
        serde_json::from_value(payload.content).unwrap_or_default()
    }
}

/// Markdown message payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarkdownPayload {
    /// Template ID
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub template_id: Option<i32>,
    /// Custom template ID
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub custom_template_id: Option<String>,
    /// Template parameters
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub params: Option<Vec<MarkdownParam>>,
    /// Markdown content
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub content: Option<String>,
    /// Markdown style
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub style: Option<MarkdownStyle>,
    /// Markdown guide message
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub process_msg: Option<String>,
}

/// Markdown style.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MarkdownStyle {
    /// Body font size, for example small/middle/large.
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub main_font_size: Option<String>,
    /// Layout, for example hide_avatar_and_center.
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub layout: Option<String>,
}

/// Markdown parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkdownParam {
    /// Parameter key
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub key: Option<String>,
    /// Parameter values
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub values: Option<Vec<String>>,
}

/// Media message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Media {
    /// Uploaded file ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_uuid: Option<String>,
    /// File info
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_info: Option<String>,
    /// TTL (time to live)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
}

/// Message reference structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    /// Referenced message ID
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub message_id: Option<String>,
    /// Whether to ignore getting reference message error
    #[serde(default, serialize_with = "serialize_option_as_default")]
    pub ignore_get_message_error: Option<bool>,
}

/// Typing/input status payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InputNotify {
    /// Input status type.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub input_type: Option<u32>,
    /// Duration in seconds for the input status.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub input_second: Option<i32>,
}

/// Prompt keyboard payload shown in the interaction area.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PromptKeyboard {
    /// Keyboard content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
}

/// Message action button payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionButton {
    /// Action template ID.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub template_id: Option<i32>,
    /// Callback data returned by interaction events.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub callback_data: Option<String>,
    /// Whether to show feedback controls.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub feedback: Option<bool>,
    /// Whether to show TTS control.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub tts: Option<bool>,
    /// Whether to show regenerate control.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub re_generate: Option<bool>,
    /// Whether to show stop-generation control.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub stop_generate: Option<bool>,
}

/// Streaming message metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Stream {
    /// Streaming state.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub state: Option<i32>,
    /// Streaming message ID.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub id: Option<String>,
    /// Streaming fragment index.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub index: Option<i32>,
    /// Whether to reset an unfinished stream.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub reset: Option<bool>,
}

/// Setting guide target used by direct-message setting guide payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SettingGuide {
    /// Guild ID to jump to from the setting guide.
    #[serde(default)]
    pub guild_id: String,
}

/// Setting guide send payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SettingGuideParams {
    /// Optional text content, usually mentions for channel setting guides.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub content: Option<String>,
    /// Optional direct-message jump target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_guide: Option<SettingGuide>,
}

impl SettingGuideParams {
    /// Builds a channel setting guide that mentions the provided user IDs.
    pub fn for_users(user_ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let content = user_ids
            .into_iter()
            .map(|user_id| format!("<@{}>", user_id.as_ref()))
            .collect::<String>();
        Self {
            content: (!content.is_empty()).then_some(content),
            ..Default::default()
        }
    }

    /// Builds a direct-message setting guide that jumps to the provided guild.
    pub fn for_guild(guild_id: impl Into<String>) -> Self {
        Self {
            setting_guide: Some(SettingGuide {
                guild_id: guild_id.into(),
            }),
            ..Default::default()
        }
    }
}
