//! Audio-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling audio events,
//! audio controls, and live audio channel interactions.

use crate::models::api::AudioAction;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Audio status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AudioStatus {
    /// Start audio playback
    #[default]
    Start = 0,
    /// Pause audio playback
    Pause = 1,
    /// Resume audio playback
    Resume = 2,
    /// Stop audio playback
    Stop = 3,
}

impl Serialize for AudioStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

impl<'de> Deserialize<'de> for AudioStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u32::deserialize(deserializer)? {
            0 => Ok(Self::Start),
            1 => Ok(Self::Pause),
            2 => Ok(Self::Resume),
            3 => Ok(Self::Stop),
            value => Err(serde::de::Error::custom(format!(
                "invalid audio status {value}"
            ))),
        }
    }
}

/// Public audio channel type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PublicAudioType {
    /// Voice channel
    #[default]
    Voice,
    /// Live channel
    Live,
    /// Unknown channel type
    Unknown(u8),
}

wire_enum!(PublicAudioType, u8, Unknown, {
    Voice = 2,
    Live = 5,
});

impl Serialize for PublicAudioType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(u8::from(*self))
    }
}

impl<'de> Deserialize<'de> for PublicAudioType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(u8::deserialize(deserializer)?))
    }
}

/// Audio control structure for managing audio playback
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AudioControl {
    /// URL of the audio file
    #[serde(default)]
    pub audio_url: String,
    /// Text description of the audio
    #[serde(default)]
    pub text: String,
    /// Current audio status
    #[serde(default)]
    pub status: AudioStatus,
}

/// Audio event data structure
#[derive(Debug, Clone, Serialize)]
pub struct Audio {
    /// Event ID
    #[serde(skip)]
    pub event_id: Option<String>,
    /// Channel ID where the audio event occurred
    pub channel_id: Option<String>,
    /// Guild ID where the audio event occurred
    pub guild_id: Option<String>,
    /// URL of the audio file
    pub audio_url: Option<String>,
    /// Text description of the audio
    pub text: Option<String>,
}

impl Audio {
    /// Builds audio event data from the gateway payload.
    pub fn new(event_id: Option<String>, data: AudioAction) -> Self {
        Self {
            event_id,
            channel_id: non_empty(data.channel_id),
            guild_id: non_empty(data.guild_id),
            audio_url: non_empty(data.audio_url),
            text: non_empty(data.text),
        }
    }
}

fn non_empty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}

/// Public audio event data structure for live channels
#[derive(Debug, Clone, Serialize)]
pub struct PublicAudio {
    /// Guild ID
    pub guild_id: Option<String>,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Channel type (voice or live)
    pub channel_type: Option<PublicAudioType>,
    /// User ID
    pub user_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PublicAudioWire {
    #[serde(default)]
    guild_id: Option<String>,
    #[serde(default)]
    channel_id: Option<String>,
    #[serde(default)]
    channel_type: Option<PublicAudioType>,
    #[serde(default)]
    user_id: Option<String>,
}

impl PublicAudio {
    /// Builds public audio event data from the gateway payload.
    pub fn new(data: serde_json::Value) -> Self {
        let wire: PublicAudioWire = serde_json::from_value(data).unwrap_or_default();
        Self {
            guild_id: wire.guild_id,
            channel_id: wire.channel_id,
            channel_type: wire.channel_type,
            user_id: wire.user_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_status() {
        assert_eq!(AudioStatus::Start as u32, 0);
        assert_eq!(AudioStatus::Pause as u32, 1);
        assert_eq!(AudioStatus::Resume as u32, 2);
        assert_eq!(AudioStatus::Stop as u32, 3);
    }

    #[test]
    fn audio_status_uses_numeric_json() {
        assert_eq!(
            serde_json::to_value(AudioStatus::Start).unwrap(),
            serde_json::json!(0)
        );
        assert_eq!(
            serde_json::to_value(AudioStatus::Pause).unwrap(),
            serde_json::json!(1)
        );
        assert_eq!(
            serde_json::from_value::<AudioStatus>(serde_json::json!(3)).unwrap(),
            AudioStatus::Stop
        );
        assert!(serde_json::from_value::<AudioStatus>(serde_json::json!(4)).is_err());
    }

    #[test]
    fn audio_control_uses_json_shape() {
        let control = AudioControl {
            audio_url: "https://example.com/audio.mp3".to_string(),
            text: "now playing".to_string(),
            status: AudioStatus::Start,
        };
        let value = serde_json::to_value(&control).unwrap();

        assert_eq!(value["audio_url"], "https://example.com/audio.mp3");
        assert_eq!(value["text"], "now playing");
        assert_eq!(value["status"], 0);
    }

    #[test]
    fn test_public_audio_type() {
        assert_eq!(u8::from(PublicAudioType::Voice), 2);
        assert_eq!(u8::from(PublicAudioType::Live), 5);
    }

    #[test]
    fn audio_event_helper_hides_empty_zero_values() {
        let audio = Audio::new(None, AudioAction::default());

        assert!(audio.guild_id.is_none());
        assert!(audio.channel_id.is_none());
        assert!(audio.audio_url.is_none());
        assert!(audio.text.is_none());
    }

    #[test]
    fn audio_event_id_is_internal_only() {
        let audio = Audio::new(
            Some("event-1".to_string()),
            AudioAction {
                guild_id: "guild-1".to_string(),
                channel_id: "channel-1".to_string(),
                audio_url: "https://example.com/audio.mp3".to_string(),
                text: "now playing".to_string(),
            },
        );

        assert_eq!(audio.event_id.as_deref(), Some("event-1"));
        let value = serde_json::to_value(&audio).unwrap();
        assert!(value.get("event_id").is_none());
    }
}
