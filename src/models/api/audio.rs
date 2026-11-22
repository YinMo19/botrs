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
