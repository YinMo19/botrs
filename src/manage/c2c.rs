use crate::api::BotApi;
use serde::{Deserialize, Serialize};

/// C2C friend event payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct C2CFriendData {
    /// User OpenID
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

impl C2CFriendData {
    /// Creates a friend event DTO from gateway data.
    pub fn new(data: &serde_json::Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// C2C (Client-to-Client) management event structure
#[derive(Debug, Clone, Serialize)]
pub struct C2CManageEvent {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
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
    pub fn new(api: BotApi, event_id: Option<String>, data: &serde_json::Value) -> Self {
        let wire: C2CManageWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            event_id,
            timestamp: wire.timestamp,
            openid: wire.openid,
            nick: wire.nick,
            avatar: wire.avatar,
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Get the event timestamp as a formatted string
    pub fn formatted_timestamp(&self) -> Option<String> {
        self.timestamp.map(|ts| {
            let datetime = chrono::DateTime::from_timestamp(ts as i64, 0).unwrap_or_default();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
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

impl std::fmt::Display for C2CManageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "C2CManageEvent {{ event_id: {:?}, timestamp: {:?}, openid: {:?}, nick: {:?}, avatar: {:?} }}",
            self.event_id, self.timestamp, self.openid, self.nick, self.avatar
        )
    }
}
