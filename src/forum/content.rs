use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Forum content format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
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
    /// Unknown format value
    Unknown(u8),
}

wire_enum!(Format, u8, Unknown, {
    PlainText = 1,
    Html = 2,
    Markdown = 3,
    Json = 4,
});

/// Body used by the forum thread creation API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadToCreate {
    /// Thread title.
    pub title: String,
    /// Thread content.
    pub content: String,
    /// Content format.
    pub format: Format,
}

impl ThreadToCreate {
    /// Creates a forum thread request body.
    pub fn new(title: impl Into<String>, content: impl Into<String>, format: Format) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            format,
        }
    }
}

/// Text element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
    /// Text content
    pub text: Option<String>,
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

/// Image element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Image {
    /// Platform image data
    #[serde(default)]
    pub plat_image: PlatImage,
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

/// Video element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Video {
    /// Platform video data
    #[serde(default)]
    pub plat_video: PlatVideo,
}

/// URL element structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Url {
    /// URL
    pub url: Option<String>,
    /// URL description
    pub desc: Option<String>,
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

/// Paragraph structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paragraph {
    /// Elements in the paragraph
    #[serde(default)]
    pub elems: Vec<Elem>,
    /// Paragraph properties
    pub props: Option<Value>,
}

/// Title structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Title {
    /// Paragraphs in the title
    #[serde(default)]
    pub paragraphs: Vec<Paragraph>,
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
