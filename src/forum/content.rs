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
