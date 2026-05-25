//! Forum-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling forum threads,
//! posts, replies, and open forum events.

use crate::api::BotApi;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Forum content format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Format {
    /// Plain text format
    PlainText = 1,
    /// HTML format
    Html = 2,
    /// Markdown format
    Markdown = 3,
    /// JSON format
    Json = 4,
}

/// Text element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
    /// Text content
    pub text: Option<String>,
}

impl Text {
    /// Create a new Text instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Platform image structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlatImage {
    /// Image URL
    pub url: Option<String>,
    /// Image width
    pub width: Option<u32>,
    /// Image height
    pub height: Option<u32>,
    /// Image ID
    pub image_id: Option<String>,
}

impl PlatImage {
    /// Create a new PlatImage instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Image element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Image {
    /// Platform image data
    #[serde(default)]
    pub plat_image: PlatImage,
}

impl Image {
    /// Create a new Image instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Video cover structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cover {
    /// Cover URL
    pub url: Option<String>,
    /// Cover width
    pub width: Option<u32>,
    /// Cover height
    pub height: Option<u32>,
}

impl Cover {
    /// Create a new Cover instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Platform video structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlatVideo {
    /// Video URL
    pub url: Option<String>,
    /// Video width
    pub width: Option<u32>,
    /// Video height
    pub height: Option<u32>,
    /// Video ID
    pub video_id: Option<String>,
    /// Video cover
    #[serde(default)]
    pub cover: Cover,
}

impl PlatVideo {
    /// Create a new PlatVideo instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Video element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Video {
    /// Platform video data
    #[serde(default)]
    pub plat_video: PlatVideo,
}

impl Video {
    /// Create a new Video instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// URL element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Url {
    /// URL
    pub url: Option<String>,
    /// URL description
    pub desc: Option<String>,
}

impl Url {
    /// Create a new Url instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Element structure for forum content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Elem {
    /// Element type (1: text, 2: image, 3: video, 4: url)
    #[serde(rename = "type", default)]
    pub element_type: Option<u8>,
    /// Text content (if type is 1)
    #[serde(default)]
    pub text: Option<Text>,
    /// Image content (if type is 2)
    #[serde(default)]
    pub image: Option<Image>,
    /// Video content (if type is 3)
    #[serde(default)]
    pub video: Option<Video>,
    /// URL content (if type is 4)
    #[serde(default)]
    pub url: Option<Url>,
}

impl Elem {
    /// Create a new Elem instance
    pub fn new(data: &Value) -> Self {
        // Forum payloads only populate the variant matching the element type;
        // discard payloads that disagree with the discriminator to keep wire
        // shape stable.
        let mut elem: Self = serde_json::from_value(data.clone()).unwrap_or_default();
        match elem.element_type {
            Some(1) => {
                elem.image = None;
                elem.video = None;
                elem.url = None;
            }
            Some(2) => {
                elem.text = None;
                elem.video = None;
                elem.url = None;
            }
            Some(3) => {
                elem.text = None;
                elem.image = None;
                elem.url = None;
            }
            Some(4) => {
                elem.text = None;
                elem.image = None;
                elem.video = None;
            }
            _ => {
                elem.text = None;
                elem.image = None;
                elem.video = None;
                elem.url = None;
            }
        }
        elem
    }
}

/// Paragraph structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paragraph {
    /// Elements in the paragraph
    #[serde(default)]
    pub elems: Vec<Elem>,
    /// Paragraph properties
    pub props: Option<Value>,
}

impl Paragraph {
    /// Create a new Paragraph instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Title structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Title {
    /// Paragraphs in the title
    #[serde(default)]
    pub paragraphs: Vec<Paragraph>,
}

impl Title {
    /// Create a new Title instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

/// Content structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Content {
    /// Paragraphs in the content
    #[serde(default)]
    pub paragraphs: Vec<Paragraph>,
}

impl Content {
    /// Create a new Content instance
    pub fn new(data: &Value) -> Self {
        serde_json::from_value(data.clone()).unwrap_or_default()
    }
}

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
    /// Create a new Thread instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `event_id` - Optional event ID
    /// * `data` - Thread data from the gateway
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
    /// Create a new OpenThread instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `data` - Open forum event data from the gateway
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(Format::PlainText as u8, 1);
        assert_eq!(Format::Html as u8, 2);
        assert_eq!(Format::Markdown as u8, 3);
        assert_eq!(Format::Json as u8, 4);
    }

    #[test]
    fn test_text_creation() {
        let data = serde_json::json!({
            "text": "Hello, world!"
        });
        let text = Text::new(&data);
        assert_eq!(text.text, Some("Hello, world!".to_string()));
    }

    #[test]
    fn thread_info_keeps_title_and_content_as_strings() {
        let data = serde_json::json!({
            "thread_id": "thread-1",
            "title": "{\"paragraphs\":[]}",
            "content": "{\"paragraphs\":[{\"elems\":[]}]}",
            "date_time": "2024-01-02T03:04:05+08:00"
        });

        let thread_info = ThreadInfo::new(&data);
        assert_eq!(thread_info.thread_id.as_deref(), Some("thread-1"));
        assert_eq!(thread_info.title.as_deref(), Some("{\"paragraphs\":[]}"));
        assert_eq!(
            thread_info.content.as_deref(),
            Some("{\"paragraphs\":[{\"elems\":[]}]}")
        );
        assert_eq!(
            thread_info.date_time.as_deref(),
            Some("2024-01-02T03:04:05+08:00")
        );

        let value = serde_json::to_value(&thread_info).unwrap();
        assert_eq!(value["title"], serde_json::json!("{\"paragraphs\":[]}"));
        assert_eq!(
            value["content"],
            serde_json::json!("{\"paragraphs\":[{\"elems\":[]}]}")
        );
    }

    #[test]
    fn forum_audit_result_serializes_zero_value_strings() {
        // The QQ Bot Open API audit payload defines every field as a bare
        // string/integer; zero values must serialize as `""`/`0` rather than
        // being omitted or rendered as `null`.
        let data = serde_json::json!({
            "task_id": "task-1",
            "guild_id": "guild-1",
            "channel_id": "channel-1",
            "author_id": "author-1",
            "thread_id": "thread-1",
            "post_id": "",
            "reply_id": "",
            "type": 1,
            "result": 2,
            "err_msg": "",
            "date_time": "2024-01-02T03:04:05+08:00"
        });
        let parsed = ForumAuditResult::new(Some("event-1".into()), &data);

        assert_eq!(parsed.task_id, "task-1");
        assert_eq!(parsed.publish_type, 1);
        assert_eq!(parsed.result, 2);
        assert_eq!(parsed.event_id.as_deref(), Some("event-1"));

        let value = serde_json::to_value(ForumAuditResult::default()).unwrap();
        assert_eq!(value["task_id"], "");
        assert_eq!(value["guild_id"], "");
        assert_eq!(value["type"], 0);
        assert_eq!(value["result"], 0);
        assert_eq!(value["date_time"], "");
        // event_id is internal-only and never appears on the wire.
        assert!(value.get("event_id").is_none());
    }
}
