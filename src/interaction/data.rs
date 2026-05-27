use super::InteractionDataType;
use crate::models::serde_helpers::{is_default, is_zero_i32};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Resolved interaction data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Resolved {
    /// Search keyword
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub keyword: String,
    /// User ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user_id: String,
    /// Request payload
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request: String,
    /// Message ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message_id: String,
    /// Member nickname
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub member_nick: String,
    /// Button data
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub button_data: String,
    /// Button ID (for button interactions)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub button_id: String,
    /// Feature ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub feature_id: String,
    /// Message feedback option
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub feedback_opt: String,
    /// Whether feedback option is checked
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub checked: i32,
    /// Additional resolved fields not yet modeled by this crate.
    #[serde(default, flatten, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, Value>,
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
