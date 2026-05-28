//! Audio gateway event payloads and OpenAPI request models.

pub use crate::audio::{Audio, PublicAudio, PublicAudioType};

use serde::{Deserialize, Serialize};

/// Audio playback status used by the audio control API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum AudioStatus {
    /// Start playing audio.
    #[default]
    Start = 0,
    /// Pause audio playback.
    Pause = 1,
    /// Resume audio playback.
    Resume = 2,
    /// Stop audio playback.
    Stop = 3,
    /// Unknown platform value.
    Unknown(u32),
}

wire_enum!(AudioStatus, u32, Unknown, {
    Start = 0,
    Pause = 1,
    Resume = 2,
    Stop = 3,
});

/// Audio control payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AudioControl {
    /// Audio URL.
    #[serde(default)]
    pub audio_url: String,
    /// Text shown alongside the audio action.
    #[serde(default)]
    pub text: String,
    /// Playback status.
    #[serde(default)]
    pub status: AudioStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_control_keeps_start_status_on_the_wire() {
        let value = serde_json::to_value(AudioControl::default()).unwrap();

        assert_eq!(value["status"], 0);
    }
}
