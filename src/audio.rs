//! Audio-related functionality for QQ Bot
//!
//! This module provides structures and implementations for handling audio events,
//! audio controls, and live audio channel interactions.

use crate::api::BotApi;
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

#[allow(non_upper_case_globals)]
pub const AudioStatusStart: AudioStatus = AudioStatus::Start;
#[allow(non_upper_case_globals)]
pub const AudioStatusPause: AudioStatus = AudioStatus::Pause;
#[allow(non_upper_case_globals)]
pub const AudioStatusResume: AudioStatus = AudioStatus::Resume;
#[allow(non_upper_case_globals)]
pub const AudioStatusStop: AudioStatus = AudioStatus::Stop;

/// Public audio channel type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PublicAudioType {
    /// Voice channel
    Voice = 2,
    /// Live channel
    Live = 5,
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
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Event ID
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
    /// Create a new Audio instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `event_id` - Optional event ID
    /// * `data` - Audio action data
    pub fn new(api: BotApi, event_id: Option<String>, data: AudioAction) -> Self {
        Self {
            api,
            event_id,
            channel_id: non_empty(data.channel_id),
            guild_id: non_empty(data.guild_id),
            audio_url: non_empty(data.audio_url),
            text: non_empty(data.text),
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }
}

fn non_empty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}

impl std::fmt::Display for Audio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Audio {{ channel_id: {:?}, guild_id: {:?}, audio_url: {:?}, text: {:?}, event_id: {:?} }}",
            self.channel_id, self.guild_id, self.audio_url, self.text, self.event_id
        )
    }
}

/// Public audio event data structure for live channels
#[derive(Debug, Clone, Serialize)]
pub struct PublicAudio {
    /// API client reference
    #[serde(skip)]
    api: BotApi,
    /// Guild ID
    pub guild_id: Option<String>,
    /// Channel ID
    pub channel_id: Option<String>,
    /// Channel type (voice or live)
    pub channel_type: Option<PublicAudioType>,
    /// User ID
    pub user_id: Option<String>,
}

impl PublicAudio {
    /// Create a new PublicAudio instance
    ///
    /// # Arguments
    ///
    /// * `api` - The Bot API client
    /// * `data` - Audio live data from the gateway
    pub fn new(api: BotApi, data: serde_json::Value) -> Self {
        Self {
            api,
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_type: data
                .get("channel_type")
                .and_then(|v| v.as_u64())
                .and_then(|v| match v {
                    2 => Some(PublicAudioType::Voice),
                    5 => Some(PublicAudioType::Live),
                    _ => None,
                }),
            user_id: data
                .get("user_id")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }

    /// Get the API client reference
    pub fn api(&self) -> &BotApi {
        &self.api
    }
}

impl std::fmt::Display for PublicAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PublicAudio {{ guild_id: {:?}, channel_id: {:?}, channel_type: {:?}, user_id: {:?} }}",
            self.guild_id, self.channel_id, self.channel_type, self.user_id
        )
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
    fn audio_status_uses_botgo_numeric_json() {
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
    fn audio_control_uses_botgo_json_shape() {
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
        assert_eq!(PublicAudioType::Voice as u8, 2);
        assert_eq!(PublicAudioType::Live as u8, 5);
    }

    #[test]
    fn audio_event_helper_hides_empty_botgo_zero_values() {
        let http = crate::http::HttpClient::new(30, false).unwrap();
        let api = BotApi::new(http);
        let audio = Audio::new(api, None, AudioAction::default());

        assert!(audio.guild_id.is_none());
        assert!(audio.channel_id.is_none());
        assert!(audio.audio_url.is_none());
        assert!(audio.text.is_none());
    }
}
