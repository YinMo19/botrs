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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    /// Text content
    pub text: Option<String>,
}

impl Text {
    /// Create a new Text instance
    pub fn new(data: &Value) -> Self {
        Self {
            text: data.get("text").and_then(|v| v.as_str()).map(String::from),
        }
    }
}

/// Platform image structure
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        Self {
            url: data.get("url").and_then(|v| v.as_str()).map(String::from),
            width: data.get("width").and_then(|v| v.as_u64()).map(|v| v as u32),
            height: data
                .get("height")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
            image_id: data
                .get("image_id")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// Image element structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Platform image data
    pub plat_image: PlatImage,
}

impl Image {
    /// Create a new Image instance
    pub fn new(data: &Value) -> Self {
        Self {
            plat_image: PlatImage::new(
                data.get("plat_image")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
        }
    }
}

/// Video cover structure
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        Self {
            url: data.get("url").and_then(|v| v.as_str()).map(String::from),
            width: data.get("width").and_then(|v| v.as_u64()).map(|v| v as u32),
            height: data
                .get("height")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
        }
    }
}

/// Platform video structure
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub cover: Cover,
}

impl PlatVideo {
    /// Create a new PlatVideo instance
    pub fn new(data: &Value) -> Self {
        Self {
            url: data.get("url").and_then(|v| v.as_str()).map(String::from),
            width: data.get("width").and_then(|v| v.as_u64()).map(|v| v as u32),
            height: data
                .get("height")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
            video_id: data
                .get("video_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            cover: Cover::new(
                data.get("cover")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
        }
    }
}

/// Video element structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    /// Platform video data
    pub plat_video: PlatVideo,
}

impl Video {
    /// Create a new Video instance
    pub fn new(data: &Value) -> Self {
        Self {
            plat_video: PlatVideo::new(
                data.get("plat_video")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
        }
    }
}

/// URL element structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    /// URL
    pub url: Option<String>,
    /// URL description
    pub desc: Option<String>,
}

impl Url {
    /// Create a new Url instance
    pub fn new(data: &Value) -> Self {
        Self {
            url: data.get("url").and_then(|v| v.as_str()).map(String::from),
            desc: data.get("desc").and_then(|v| v.as_str()).map(String::from),
        }
    }
}

/// Element structure for forum content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elem {
    /// Element type (1: text, 2: image, 3: video, 4: url)
    pub element_type: Option<u8>,
    /// Text content (if type is 1)
    pub text: Option<Text>,
    /// Image content (if type is 2)
    pub image: Option<Image>,
    /// Video content (if type is 3)
    pub video: Option<Video>,
    /// URL content (if type is 4)
    pub url: Option<Url>,
}

impl Elem {
    /// Create a new Elem instance
    pub fn new(data: &Value) -> Self {
        let element_type = data.get("type").and_then(|v| v.as_u64()).map(|v| v as u8);

        let mut elem = Self {
            element_type,
            text: None,
            image: None,
            video: None,
            url: None,
        };

        match element_type {
            Some(1) => {
                elem.text = Some(Text::new(
                    data.get("text")
                        .unwrap_or(&Value::Object(serde_json::Map::new())),
                ));
            }
            Some(2) => {
                elem.image = Some(Image::new(
                    data.get("image")
                        .unwrap_or(&Value::Object(serde_json::Map::new())),
                ));
            }
            Some(3) => {
                elem.video = Some(Video::new(
                    data.get("video")
                        .unwrap_or(&Value::Object(serde_json::Map::new())),
                ));
            }
            Some(4) => {
                elem.url = Some(Url::new(
                    data.get("url")
                        .unwrap_or(&Value::Object(serde_json::Map::new())),
                ));
            }
            _ => {}
        }

        elem
    }
}

/// Paragraph structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    /// Elements in the paragraph
    pub elems: Vec<Elem>,
    /// Paragraph properties
    pub props: Option<Value>,
}

impl Paragraph {
    /// Create a new Paragraph instance
    pub fn new(data: &Value) -> Self {
        let elems = data
            .get("elems")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().map(Elem::new).collect())
            .unwrap_or_default();

        Self {
            elems,
            props: data.get("props").cloned(),
        }
    }
}

/// Title structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    /// Paragraphs in the title
    pub paragraphs: Vec<Paragraph>,
}

impl Title {
    /// Create a new Title instance
    pub fn new(data: &Value) -> Self {
        let paragraphs = data
            .get("paragraphs")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().map(Paragraph::new).collect())
            .unwrap_or_default();

        Self { paragraphs }
    }
}

/// Content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    /// Paragraphs in the content
    pub paragraphs: Vec<Paragraph>,
}

impl Content {
    /// Create a new Content instance
    pub fn new(data: &Value) -> Self {
        let paragraphs = data
            .get("paragraphs")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().map(Paragraph::new).collect())
            .unwrap_or_default();

        Self { paragraphs }
    }
}

/// Thread info structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadInfo {
    /// Thread title
    pub title: Title,
    /// Thread content
    pub content: Content,
    /// Thread ID
    pub thread_id: Option<String>,
    /// Creation date and time
    pub date_time: Option<String>,
}

impl ThreadInfo {
    /// Create a new ThreadInfo instance
    pub fn new(data: &Value) -> Self {
        let title_data = data
            .get("title")
            .and_then(|v| v.as_str())
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        let content_data = data
            .get("content")
            .and_then(|v| v.as_str())
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        Self {
            title: Title::new(&title_data),
            content: Content::new(&content_data),
            thread_id: data
                .get("thread_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            date_time: data
                .get("date_time")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
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

impl Thread {
    /// Create a new Thread instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `event_id` - Optional event ID
    /// * `data` - Thread data from the gateway
    pub fn new(api: BotApi, event_id: Option<String>, data: &Value) -> Self {
        Self {
            api,
            event_id,
            author_id: data
                .get("author_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            thread_info: ThreadInfo::new(
                data.get("thread_info")
                    .unwrap_or(&Value::Object(serde_json::Map::new())),
            ),
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
    /// Event ID
    pub event_id: Option<String>,
}

impl OpenThread {
    /// Create a new OpenThread instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `data` - Open forum event data from the gateway
    pub fn new(api: BotApi, data: &Value) -> Self {
        Self {
            api,
            event_id: None,
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            author_id: data
                .get("author_id")
                .and_then(|v| v.as_str())
                .map(String::from),
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
}
