use super::super::ConnectionState;
use crate::models::{channel::Channel, guild::Guild, user::Member};
use serde_json::Value;

pub(in crate::connection) fn parse_guild_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let guild_id = payload.get("id").and_then(|v| v.as_str())?;
    let guild_data = payload.get("d")?;
    let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
    Some(("guild_create", serde_json::to_value(guild).ok()?))
}

pub(in crate::connection) fn parse_guild_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let guild_id = payload.get("id").and_then(|v| v.as_str())?;
    let guild_data = payload.get("d")?;
    let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
    Some(("guild_update", serde_json::to_value(guild).ok()?))
}

pub(in crate::connection) fn parse_guild_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let guild_id = payload.get("id").and_then(|v| v.as_str())?;
    let guild_data = payload.get("d")?;
    let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
    Some(("guild_delete", serde_json::to_value(guild).ok()?))
}

pub(in crate::connection) fn parse_channel_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let channel_id = payload.get("id").and_then(|v| v.as_str())?;
    let channel_data = payload.get("d")?;
    let channel = Channel::from_data(
        state.api.clone(),
        channel_id.to_string(),
        channel_data.clone(),
    );
    Some(("channel_create", serde_json::to_value(channel).ok()?))
}

pub(in crate::connection) fn parse_channel_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let channel_id = payload.get("id").and_then(|v| v.as_str())?;
    let channel_data = payload.get("d")?;
    let channel = Channel::from_data(
        state.api.clone(),
        channel_id.to_string(),
        channel_data.clone(),
    );
    Some(("channel_update", serde_json::to_value(channel).ok()?))
}

pub(in crate::connection) fn parse_channel_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let channel_id = payload.get("id").and_then(|v| v.as_str())?;
    let channel_data = payload.get("d")?;
    let channel = Channel::from_data(
        state.api.clone(),
        channel_id.to_string(),
        channel_data.clone(),
    );
    Some(("channel_delete", serde_json::to_value(channel).ok()?))
}

pub(in crate::connection) fn parse_guild_member_add(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let _member_id = payload.get("id").and_then(|v| v.as_str())?;
    let member_data = payload.get("d")?;
    let member = Member::from_data(member_data.clone());
    Some(("guild_member_add", serde_json::to_value(member).ok()?))
}

pub(in crate::connection) fn parse_guild_member_update(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let _member_id = payload.get("id").and_then(|v| v.as_str())?;
    let member_data = payload.get("d")?;
    let member = Member::from_data(member_data.clone());
    Some(("guild_member_update", serde_json::to_value(member).ok()?))
}

pub(in crate::connection) fn parse_guild_member_remove(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let _member_id = payload.get("id").and_then(|v| v.as_str())?;
    let member_data = payload.get("d")?;
    let member = Member::from_data(member_data.clone());
    Some(("guild_member_remove", serde_json::to_value(member).ok()?))
}
