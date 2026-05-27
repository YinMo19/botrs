use serde::{Deserialize, Serialize};

/// Subscribe message status event data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubscribeMessageStatusData {
    /// Group OpenID, present for group subscription messages
    #[serde(default)]
    pub group_openid: String,
    /// User OpenID, present for C2C subscription messages
    #[serde(default)]
    pub openid: String,
    /// Template authorization results
    #[serde(default)]
    pub result: Vec<SubscribeMsgTemplateResult>,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl SubscribeMessageStatusData {
    /// Creates a subscribe status event from gateway data.
    pub(crate) fn new(event_id: Option<String>, data: &serde_json::Value) -> Self {
        let mut event = serde_json::from_value::<Self>(data.clone()).unwrap_or_default();
        event.event_id = event_id;
        event
    }
}

/// Subscribe template authorization result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubscribeMsgTemplateResult {
    /// Official template ID
    #[serde(default)]
    pub template_id: i32,
    /// Custom template ID
    #[serde(default)]
    pub custom_template_id: String,
    /// Authorization operation, 1 allow and 2 reject
    #[serde(default)]
    pub op: u32,
    /// Subscription ID
    #[serde(default)]
    pub subscribe_id: String,
    /// Status update timestamp
    #[serde(default)]
    pub update_ts: u64,
}
