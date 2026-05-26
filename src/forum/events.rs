use crate::api::BotApi;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Thread info structure for forum events.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreadInfo {
    /// Thread title
    pub title: Option<String>,
    /// Thread content
    pub content: Option<String>,
    /// Thread ID
    pub thread_id: Option<String>,
    /// Creation date and time
    pub date_time: Option<String>,
}

impl ThreadInfo {
    /// Create a new ThreadInfo instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Post info structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostInfo {
    /// Thread ID
    pub thread_id: Option<String>,
    /// Post ID
    pub post_id: Option<String>,
    /// Post content
    pub content: Option<String>,
    /// Creation date and time
    pub date_time: Option<String>,
}

impl PostInfo {
    /// Create a new PostInfo instance.
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Reply info structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplyInfo {
    /// Thread ID
    pub thread_id: Option<String>,
    /// Post ID
    pub post_id: Option<String>,
    /// Reply ID
    pub reply_id: Option<String>,
    /// Reply content
    pub content: Option<String>,
    /// Creation date and time
    pub date_time: Option<String>,
}

impl ReplyInfo {
    /// Create a new ReplyInfo instance.
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Forum thread structure
#[derive(Debug, Clone, Serialize)]
pub struct Thread {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Thread information
    pub thread_info: ThreadInfo,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Guild ID
    pub guild_id: Option<String>,
    /// Author ID
    pub author_id: Option<String>,
    /// Event ID
    pub event_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ThreadWire {
    #[serde(default)]
    thread_info: ThreadInfo,
    #[serde(default)]
    channel_id: Option<String>,
    #[serde(default)]
    guild_id: Option<String>,
    #[serde(default)]
    author_id: Option<String>,
}

impl Thread {
    /// Builds a forum thread event from the gateway payload.
    pub fn new(api: BotApi, event_id: Option<String>, data: &Value) -> Self {
        let wire: ThreadWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            thread_info: wire.thread_info,
            channel_id: wire.channel_id,
            guild_id: wire.guild_id,
            author_id: wire.author_id,
            event_id,
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }
}

impl std::fmt::Display for Thread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Thread {{ channel_id: {:?}, guild_id: {:?}, author_id: {:?}, event_id: {:?} }}",
            self.channel_id, self.guild_id, self.author_id, self.event_id
        )
    }
}

/// Forum post structure.
#[derive(Debug, Clone, Serialize)]
pub struct Post {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Guild ID
    pub guild_id: Option<String>,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Author ID
    pub author_id: Option<String>,
    /// Post information
    pub post_info: PostInfo,
    /// Event ID
    pub event_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PostWire {
    #[serde(default)]
    guild_id: Option<String>,
    #[serde(default)]
    channel_id: Option<String>,
    #[serde(default)]
    author_id: Option<String>,
    #[serde(default)]
    post_info: PostInfo,
}

impl Post {
    /// Create a new Post instance.
    pub fn new(api: BotApi, event_id: Option<String>, data: &Value) -> Self {
        let wire: PostWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            guild_id: wire.guild_id,
            channel_id: wire.channel_id,
            author_id: wire.author_id,
            post_info: wire.post_info,
            event_id,
        }
    }

    /// Get the API client reference.
    pub fn api(&self) -> &BotApi {
        &self.api
    }
}

/// Forum reply structure.
#[derive(Debug, Clone, Serialize)]
pub struct Reply {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Guild ID
    pub guild_id: Option<String>,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Author ID
    pub author_id: Option<String>,
    /// Reply information
    pub reply_info: ReplyInfo,
    /// Event ID
    pub event_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ReplyWire {
    #[serde(default)]
    guild_id: Option<String>,
    #[serde(default)]
    channel_id: Option<String>,
    #[serde(default)]
    author_id: Option<String>,
    #[serde(default)]
    reply_info: ReplyInfo,
}

impl Reply {
    /// Create a new Reply instance.
    pub fn new(api: BotApi, event_id: Option<String>, data: &Value) -> Self {
        let wire: ReplyWire = serde_json::from_value(data.clone()).unwrap_or_default();
        Self {
            api,
            guild_id: wire.guild_id,
            channel_id: wire.channel_id,
            author_id: wire.author_id,
            reply_info: wire.reply_info,
            event_id,
        }
    }

    /// Get the API client reference.
    pub fn api(&self) -> &BotApi {
        &self.api
    }
}
