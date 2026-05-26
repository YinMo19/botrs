use super::super::ConnectionState;
use crate::models::message::*;
use crate::reaction::Reaction;
use serde_json::Value;

pub(in crate::connection) fn parse_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = Message::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_create", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_message_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = crate::models::message::MessageDelete::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_delete", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_at_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = Message::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("at_message_create", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_public_message_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = crate::models::message::MessageDelete::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("public_message_delete", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_direct_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = Message::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("direct_message_create", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_direct_message_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = crate::models::message::MessageDelete::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("direct_message_delete", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_message_reaction_add(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reaction_id = payload.get("id").and_then(|v| v.as_str())?;
    let reaction_data = payload.get("d")?;
    let reaction = Reaction::new(
        state.api.clone(),
        Some(reaction_id.to_string()),
        reaction_data,
    )
    .ok()?;
    Some(("message_reaction_add", serde_json::to_value(reaction).ok()?))
}

pub(in crate::connection) fn parse_message_reaction_remove(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reaction_id = payload.get("id").and_then(|v| v.as_str())?;
    let reaction_data = payload.get("d")?;
    let reaction = Reaction::new(
        state.api.clone(),
        Some(reaction_id.to_string()),
        reaction_data,
    )
    .ok()?;
    Some((
        "message_reaction_remove",
        serde_json::to_value(reaction).ok()?,
    ))
}

pub(in crate::connection) fn parse_group_at_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = GroupMessage::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some((
        "group_at_message_create",
        serde_json::to_value(message).ok()?,
    ))
}

pub(in crate::connection) fn parse_c2c_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = C2CMessage::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("c2c_message_create", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_message_audit_pass(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = MessageAudit::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_audit_pass", serde_json::to_value(message).ok()?))
}

pub(in crate::connection) fn parse_message_audit_reject(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = MessageAudit::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_audit_reject", serde_json::to_value(message).ok()?))
}
