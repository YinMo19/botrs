use crate::models::serde_helpers::option_is_none_or_default;
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

/// Ark message wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageArk {
    /// Ark payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
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

pub type ArkKV = ArkKv;

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

pub type ArkObjKV = ArkObjKv;

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

pub type MessageEmbedThumbnail = EmbedThumbnail;

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

pub type ActionType = u32;
pub type PermissionType = u32;

pub const ACTION_TYPE_URL: ActionType = 0;
pub const ACTION_TYPE_CALLBACK: ActionType = 1;
pub const ACTION_TYPE_AT_BOT: ActionType = 2;
pub const ACTION_TYPE_MQQ_API: ActionType = 3;
pub const ACTION_TYPE_SUBSCRIBE: ActionType = 4;
#[allow(non_upper_case_globals)]
pub const ActionTypeURL: ActionType = ACTION_TYPE_URL;
#[allow(non_upper_case_globals)]
pub const ActionTypeCallback: ActionType = ACTION_TYPE_CALLBACK;
#[allow(non_upper_case_globals)]
pub const ActionTypeAtBot: ActionType = ACTION_TYPE_AT_BOT;
#[allow(non_upper_case_globals)]
pub const ActionTypeMQQAPI: ActionType = ACTION_TYPE_MQQ_API;
#[allow(non_upper_case_globals)]
pub const ActionTypeSubscribe: ActionType = ACTION_TYPE_SUBSCRIBE;

pub const PERMISSION_TYPE_SPECIFY_USER_IDS: PermissionType = 0;
pub const PERMISSION_TYPE_MANAGER: PermissionType = 1;
pub const PERMISSION_TYPE_ALL: PermissionType = 2;
pub const PERMISSION_TYPE_SPECIFY_ROLE_IDS: PermissionType = 3;
#[allow(non_upper_case_globals)]
pub const PermissionTypeSpecifyUserIDs: PermissionType = PERMISSION_TYPE_SPECIFY_USER_IDS;
#[allow(non_upper_case_globals)]
pub const PermissionTypManager: PermissionType = PERMISSION_TYPE_MANAGER;
#[allow(non_upper_case_globals)]
pub const PermissionTypAll: PermissionType = PERMISSION_TYPE_ALL;
#[allow(non_upper_case_globals)]
pub const PermissionTypSpecifyRoleIDs: PermissionType = PERMISSION_TYPE_SPECIFY_ROLE_IDS;

pub type MessageKeyboard = Keyboard;
pub type CustomKeyboard = KeyboardContent;
pub type Row = KeyboardRow;
pub type Button = KeyboardButton;
pub type RenderData = KeyboardButtonRenderData;
pub type Action = KeyboardButtonAction;
pub type Permission = KeyboardButtonPermission;
pub type SubscribeData = KeyboardSubscribeData;
pub type TemplateID = KeyboardTemplateId;
pub type Modal = KeyboardModal;

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

pub type Markdown = MarkdownPayload;
pub type MarkdownParams = MarkdownParam;

/// Markdown message payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarkdownPayload {
    /// Template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i32>,
    /// Custom template ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_template_id: Option<String>,
    /// Template parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<MarkdownParam>>,
    /// Markdown content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Markdown style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<MarkdownStyle>,
    /// Markdown guide message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_msg: Option<String>,
}

/// Markdown style.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MarkdownStyle {
    /// Body font size, for example small/middle/large.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_font_size: Option<String>,
    /// Layout, for example hide_avatar_and_center.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
}

/// Markdown parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkdownParam {
    /// Parameter key
    pub key: Option<String>,
    /// Parameter values
    pub values: Option<Vec<String>>,
}

/// Media message structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Media {
    /// File info
    pub file_info: Option<String>,
    /// TTL (time to live)
    pub ttl: Option<u32>,
}

/// Message reference structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    /// Referenced message ID
    pub message_id: Option<String>,
    /// Whether to ignore getting reference message error
    pub ignore_get_message_error: Option<bool>,
}

