use super::{PostInfo, ReplyInfo, ThreadInfo};
use crate::api::BotApi;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Forum publish audit result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ForumAuditResult {
    /// Audit task ID
    #[serde(default)]
    pub task_id: String,
    /// Guild ID
    #[serde(default)]
    pub guild_id: String,
    /// Channel ID
    #[serde(default)]
    pub channel_id: String,
    /// Author ID
    #[serde(default)]
    pub author_id: String,
    /// Thread ID
    #[serde(default)]
    pub thread_id: String,
    /// Post ID
    #[serde(default)]
    pub post_id: String,
    /// Reply ID
    #[serde(default)]
    pub reply_id: String,
    /// Publish type
    #[serde(default, rename = "type")]
    pub publish_type: u32,
    /// Audit result
    #[serde(default)]
    pub result: u32,
    /// Error message
    #[serde(default)]
    pub err_msg: String,
    /// Creation date and time
    #[serde(default)]
    pub date_time: String,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl ForumAuditResult {
    /// Create a forum audit result from gateway data.
    pub fn new(event_id: Option<String>, data: &Value) -> Self {
        let mut result = serde_json::from_value::<Self>(data.clone()).unwrap_or_default();
        result.event_id = event_id;
        result
    }
}

/// Open forum thread structure
#[derive(Debug, Clone, Serialize)]
pub struct OpenThread {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Guild ID
    pub guild_id: Option<String>,
    /// Author ID
    pub author_id: Option<String>,
    /// Thread information when present
    pub thread_info: Option<ThreadInfo>,
    /// Post information when present
    pub post_info: Option<PostInfo>,
    /// Reply information when present
    pub reply_info: Option<ReplyInfo>,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct OpenThreadWire {
    #[serde(default)]
    guild_id: Option<String>,
    #[serde(default)]
    channel_id: Option<String>,
    #[serde(default)]
    author_id: Option<String>,
    #[serde(default)]
    thread_info: Option<ThreadInfo>,
    #[serde(default)]
    post_info: Option<PostInfo>,
    #[serde(default)]
    reply_info: Option<ReplyInfo>,
}

impl OpenThread {
    /// Builds an open forum event from the gateway payload.
    pub fn new(api: BotApi, data: &Value) -> Self {
        let wire: OpenThreadWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            event_id: None,
            guild_id: wire.guild_id,
            channel_id: wire.channel_id,
            author_id: wire.author_id,
            thread_info: wire.thread_info,
            post_info: wire.post_info,
            reply_info: wire.reply_info,
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }
}

impl std::fmt::Display for OpenThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpenThread {{ channel_id: {:?}, guild_id: {:?}, author_id: {:?}, event_id: {:?} }}",
            self.channel_id, self.guild_id, self.author_id, self.event_id
        )
    }
}
