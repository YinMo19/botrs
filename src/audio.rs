//! Audio-related functionality for QQ Bot
//!
//! This module provides structures for gateway audio events.

use crate::models::api::AudioAction;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
