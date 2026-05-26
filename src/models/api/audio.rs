use serde::{Deserialize, Serialize};

/// Audio action data structure for audio events
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AudioAction {
    /// Guild ID where the audio event occurred
    #[serde(default)]
    pub guild_id: String,
    /// Channel ID where the audio event occurred
    #[serde(default)]
    pub channel_id: String,
    /// URL of the audio file
    #[serde(default)]
    pub audio_url: String,
    /// Text description of the audio
    #[serde(default)]
    pub text: String,
}

impl AudioAction {
    pub(crate) fn from_value(value: &serde_json::Value) -> Self {
        Self {
            guild_id: string_field(value, "guild_id"),
            channel_id: string_field(value, "channel_id"),
            audio_url: string_field(value, "audio_url"),
            text: string_field(value, "text"),
        }
    }
}

fn string_field(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or_default()
        .to_string()
}
