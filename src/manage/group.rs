use crate::api::BotApi;
use serde::{Deserialize, Serialize};

/// Group management event structure
#[derive(Debug, Clone, Serialize)]
pub struct GroupManageEvent {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
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
    pub fn new(api: BotApi, event_id: Option<String>, data: &serde_json::Value) -> Self {
        let wire: GroupManageWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            event_id,
            timestamp: wire.timestamp,
            group_openid: wire.group_openid,
            op_member_openid: wire.op_member_openid,
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
struct GroupManageWire {
    #[serde(default)]
    timestamp: Option<u64>,
    #[serde(default)]
    group_openid: Option<String>,
    #[serde(default)]
    op_member_openid: Option<String>,
}

impl std::fmt::Display for GroupManageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GroupManageEvent {{ event_id: {:?}, timestamp: {:?}, group_openid: {:?}, op_member_openid: {:?} }}",
            self.event_id, self.timestamp, self.group_openid, self.op_member_openid
        )
    }
}
