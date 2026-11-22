use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Thread info structure for forum events.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreadInfo {
    /// Thread title
    #[serde(default)]
    pub title: String,
    /// Thread content
    #[serde(default)]
    pub content: String,
    /// Thread ID
    #[serde(default)]
    pub thread_id: String,
    /// Creation date and time
    #[serde(default)]
    pub date_time: String,
}

/// Post info structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostInfo {
    /// Thread ID
    #[serde(default)]
    pub thread_id: String,
    /// Post ID
    #[serde(default)]
    pub post_id: String,
    /// Post content
    #[serde(default)]
    pub content: String,
    /// Creation date and time
    #[serde(default)]
    pub date_time: String,
}

/// Reply info structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplyInfo {
    /// Thread ID
    #[serde(default)]
    pub thread_id: String,
    /// Post ID
    #[serde(default)]
    pub post_id: String,
    /// Reply ID
    #[serde(default)]
    pub reply_id: String,
    /// Reply content
    #[serde(default)]
    pub content: String,
    /// Creation date and time
    #[serde(default)]
    pub date_time: String,
}

/// Forum thread structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    /// Thread information
    #[serde(default)]
    pub thread_info: ThreadInfo,
    /// Channel ID
    #[serde(default)]
    pub channel_id: String,
    /// Guild ID
    #[serde(default)]
    pub guild_id: String,
    /// Author ID
    #[serde(default)]
    pub author_id: String,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl Thread {
    /// Builds a forum thread event from the gateway payload.
    pub(crate) fn new(event_id: Option<String>, data: Value) -> serde_json::Result<Self> {
        let mut event: Self = serde_json::from_value(data)?;
        event.event_id = event_id;
        Ok(event)
    }
}

/// Forum post structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// Guild ID
    #[serde(default)]
    pub guild_id: String,
    /// Channel ID
    #[serde(default)]
    pub channel_id: String,
    /// Author ID
    #[serde(default)]
    pub author_id: String,
    /// Post information
    #[serde(default)]
    pub post_info: PostInfo,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl Post {
    /// Create a new Post instance.
    pub(crate) fn new(event_id: Option<String>, data: Value) -> serde_json::Result<Self> {
        let mut event: Self = serde_json::from_value(data)?;
        event.event_id = event_id;
        Ok(event)
    }
}

/// Forum reply structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    /// Guild ID
    #[serde(default)]
    pub guild_id: String,
    /// Channel ID
    #[serde(default)]
    pub channel_id: String,
    /// Author ID
    #[serde(default)]
    pub author_id: String,
    /// Reply information
    #[serde(default)]
    pub reply_info: ReplyInfo,
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl Reply {
    /// Create a new Reply instance.
    pub(crate) fn new(event_id: Option<String>, data: Value) -> serde_json::Result<Self> {
        let mut event: Self = serde_json::from_value(data)?;
        event.event_id = event_id;
        Ok(event)
    }
}
