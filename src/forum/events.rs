use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response returned when listing forum threads.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForumRsp {
    /// Threads in the channel.
    #[serde(default)]
    pub threads: Vec<ThreadInfo>,
    /// Whether the listing is complete.
    #[serde(default)]
    pub is_finish: i32,
}

/// Response returned after creating a forum thread.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostThreadRsp {
    /// Async task ID for the create operation.
    #[serde(default)]
    pub task_id: String,
    /// Creation time returned by the API.
    #[serde(default)]
    pub create_time: String,
}

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
#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Default, Deserialize)]
struct ThreadWire {
    #[serde(default)]
    thread_info: ThreadInfo,
    #[serde(default)]
    channel_id: String,
    #[serde(default)]
    guild_id: String,
    #[serde(default)]
    author_id: String,
}

impl Thread {
    /// Builds a forum thread event from the gateway payload.
    pub(crate) fn new(event_id: Option<String>, data: &Value) -> Self {
        let wire: ThreadWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            thread_info: wire.thread_info,
            channel_id: wire.channel_id,
            guild_id: wire.guild_id,
            author_id: wire.author_id,
            event_id,
        }
    }
}

/// Forum post structure.
#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Default, Deserialize)]
struct PostWire {
    #[serde(default)]
    guild_id: String,
    #[serde(default)]
    channel_id: String,
    #[serde(default)]
    author_id: String,
    #[serde(default)]
    post_info: PostInfo,
}

impl Post {
    /// Create a new Post instance.
    pub(crate) fn new(event_id: Option<String>, data: &Value) -> Self {
        let wire: PostWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            guild_id: wire.guild_id,
            channel_id: wire.channel_id,
            author_id: wire.author_id,
            post_info: wire.post_info,
            event_id,
        }
    }
}

/// Forum reply structure.
#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Default, Deserialize)]
struct ReplyWire {
    #[serde(default)]
    guild_id: String,
    #[serde(default)]
    channel_id: String,
    #[serde(default)]
    author_id: String,
    #[serde(default)]
    reply_info: ReplyInfo,
}

impl Reply {
    /// Create a new Reply instance.
    pub(crate) fn new(event_id: Option<String>, data: &Value) -> Self {
        let wire: ReplyWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            guild_id: wire.guild_id,
            channel_id: wire.channel_id,
            author_id: wire.author_id,
            reply_info: wire.reply_info,
            event_id,
        }
    }
}
