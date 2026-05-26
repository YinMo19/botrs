use super::super::ConnectionState;
use crate::audio::{Audio, PublicAudio};
use crate::models::api::AudioAction;
use serde_json::Value;

pub(in crate::connection) fn parse_audio_start(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("audio_start", serde_json::to_value(audio).ok()?))
}

pub(in crate::connection) fn parse_audio_finish(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("audio_finish", serde_json::to_value(audio).ok()?))
}

pub(in crate::connection) fn parse_on_mic(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("on_mic", serde_json::to_value(audio).ok()?))
}

pub(in crate::connection) fn parse_off_mic(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("off_mic", serde_json::to_value(audio).ok()?))
}

pub(in crate::connection) fn parse_audio_or_live_channel_member_enter(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_data = payload.get("d")?;
    let public_audio = PublicAudio::new(state.api.clone(), audio_data.clone());
    Some((
        "audio_or_live_channel_member_enter",
        serde_json::to_value(public_audio).ok()?,
    ))
}

pub(in crate::connection) fn parse_audio_or_live_channel_member_exit(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_data = payload.get("d")?;
    let public_audio = PublicAudio::new(state.api.clone(), audio_data.clone());
    Some((
        "audio_or_live_channel_member_exit",
        serde_json::to_value(public_audio).ok()?,
    ))
}
