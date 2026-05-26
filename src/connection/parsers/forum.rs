use super::super::ConnectionState;
use crate::forum::{OpenThread, Thread};
use serde_json::Value;

pub(in crate::connection) fn parse_forum_thread_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_id = payload.get("id").and_then(|v| v.as_str())?;
    let thread_data = payload.get("d")?;
    let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
    Some(("forum_thread_create", serde_json::to_value(thread).ok()?))
}

pub(in crate::connection) fn parse_forum_thread_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_id = payload.get("id").and_then(|v| v.as_str())?;
    let thread_data = payload.get("d")?;
    let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
    Some(("forum_thread_update", serde_json::to_value(thread).ok()?))
}

pub(in crate::connection) fn parse_forum_thread_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_id = payload.get("id").and_then(|v| v.as_str())?;
    let thread_data = payload.get("d")?;
    let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
    Some(("forum_thread_delete", serde_json::to_value(thread).ok()?))
}

pub(in crate::connection) fn parse_forum_post_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("forum_post_create", post_data.clone()))
}

pub(in crate::connection) fn parse_forum_post_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("forum_post_delete", post_data.clone()))
}

pub(in crate::connection) fn parse_forum_reply_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("forum_reply_create", reply_data.clone()))
}

pub(in crate::connection) fn parse_forum_reply_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("forum_reply_delete", reply_data.clone()))
}

pub(in crate::connection) fn parse_forum_publish_audit_result(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audit_data = payload.get("d")?;
    Some(("forum_publish_audit_result", audit_data.clone()))
}

pub(in crate::connection) fn parse_open_forum_thread_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_data = payload.get("d")?;
    let thread = OpenThread::new(state.api.clone(), thread_data);
    Some((
        "open_forum_thread_create",
        serde_json::to_value(thread).ok()?,
    ))
}

pub(in crate::connection) fn parse_open_forum_thread_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_data = payload.get("d")?;
    let thread = OpenThread::new(state.api.clone(), thread_data);
    Some((
        "open_forum_thread_update",
        serde_json::to_value(thread).ok()?,
    ))
}

pub(in crate::connection) fn parse_open_forum_thread_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_data = payload.get("d")?;
    let thread = OpenThread::new(state.api.clone(), thread_data);
    Some((
        "open_forum_thread_delete",
        serde_json::to_value(thread).ok()?,
    ))
}

pub(in crate::connection) fn parse_open_forum_post_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("open_forum_post_create", post_data.clone()))
}

pub(in crate::connection) fn parse_open_forum_post_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("open_forum_post_delete", post_data.clone()))
}

pub(in crate::connection) fn parse_open_forum_reply_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("open_forum_reply_create", reply_data.clone()))
}

pub(in crate::connection) fn parse_open_forum_reply_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("open_forum_reply_delete", reply_data.clone()))
}
