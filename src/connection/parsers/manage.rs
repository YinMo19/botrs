use super::super::ConnectionState;
use crate::manage::{C2CManageEvent, GroupManageEvent};
use serde_json::Value;

pub(in crate::connection) fn parse_group_add_robot(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_add_robot", serde_json::to_value(event).ok()?))
}

pub(in crate::connection) fn parse_group_del_robot(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_del_robot", serde_json::to_value(event).ok()?))
}

pub(in crate::connection) fn parse_group_msg_reject(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_msg_reject", serde_json::to_value(event).ok()?))
}

pub(in crate::connection) fn parse_group_msg_receive(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_msg_receive", serde_json::to_value(event).ok()?))
}

pub(in crate::connection) fn parse_friend_add(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("friend_add", serde_json::to_value(event).ok()?))
}

pub(in crate::connection) fn parse_friend_del(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("friend_del", serde_json::to_value(event).ok()?))
}

pub(in crate::connection) fn parse_c2c_msg_reject(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("c2c_msg_reject", serde_json::to_value(event).ok()?))
}

pub(in crate::connection) fn parse_c2c_msg_receive(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("c2c_msg_receive", serde_json::to_value(event).ok()?))
}
