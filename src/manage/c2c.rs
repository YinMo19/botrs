use serde::{Deserialize, Serialize};

/// C2C friend event payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct C2CFriendData {
    /// User OpenID
    #[serde(default)]
    pub openid: String,
    /// Add/delete timestamp
    #[serde(default)]
    pub timestamp: u64,
    /// User nickname, currently filled by upstream when available
    #[serde(default)]
    pub nick: String,
    /// User avatar URL, currently filled by upstream when available
    #[serde(default)]
    pub avatar: String,
}

/// C2C (Client-to-Client) management event structure
#[derive(Debug, Clone, Serialize)]
pub struct C2CManageEvent {
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
    /// Timestamp of the event
    pub timestamp: Option<u64>,
    /// User OpenID
    pub openid: Option<String>,
    /// User nickname
    pub nick: Option<String>,
    /// User avatar URL
    pub avatar: Option<String>,
}

impl C2CManageEvent {
    /// Builds a C2C management event from the gateway payload.
    pub(crate) fn new(event_id: Option<String>, data: &serde_json::Value) -> Self {
        let wire: C2CManageWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            event_id,
            timestamp: wire.timestamp,
            openid: wire.openid,
            nick: wire.nick,
            avatar: wire.avatar,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct C2CManageWire {
    #[serde(default)]
    timestamp: Option<u64>,
    #[serde(default)]
    openid: Option<String>,
    #[serde(default)]
    nick: Option<String>,
    #[serde(default)]
    avatar: Option<String>,
}
