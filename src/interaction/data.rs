use super::InteractionDataType;
use crate::models::serde_helpers::is_default;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        serde_json::from_value(data.clone()).unwrap_or_default()
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
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}
