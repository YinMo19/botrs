use super::super::ConnectionState;
use crate::interaction::Interaction;
use serde_json::Value;

pub(in crate::connection) fn parse_interaction_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let interaction_id = payload.get("id").and_then(|v| v.as_str())?;
    let interaction_data = payload.get("d")?;
    let interaction = Interaction::new(
        state.api.clone(),
        Some(interaction_id.to_string()),
        interaction_data,
    );
    Some((
        "interaction_create",
        serde_json::to_value(interaction).ok()?,
    ))
}
