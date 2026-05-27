use serde::{Deserialize, Serialize};

/// Event emitted when a user enters AIO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EnterAioEvent {
    /// User OpenID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user_openid: String,
    /// Source from which the user entered AIO
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from_source: String,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl EnterAioEvent {
    /// Creates a new EnterAioEvent from gateway data.
    pub(crate) fn new(event_id: Option<String>, data: &serde_json::Value) -> Self {
        let mut event = serde_json::from_value::<Self>(data.clone()).unwrap_or_default();
        event.event_id = event_id;
        event
    }
}
