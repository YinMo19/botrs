use serde::{Deserialize, Serialize};

/// Group management event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupManageEvent {
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
    /// Timestamp of the event
    pub timestamp: Option<u64>,
    /// Group OpenID
    pub group_openid: Option<String>,
    /// Operator member OpenID
    pub op_member_openid: Option<String>,
}

impl GroupManageEvent {
    /// Builds a group management event from the gateway payload.
    pub(crate) fn new(
        event_id: Option<String>,
        data: serde_json::Value,
    ) -> serde_json::Result<Self> {
        let mut event: Self = serde_json::from_value(data)?;
        event.event_id = event_id;
        Ok(event)
    }
}
