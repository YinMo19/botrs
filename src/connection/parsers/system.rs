use super::super::ConnectionState;
use serde_json::Value;

pub(in crate::connection) fn parse_ready(
    _state: &ConnectionState,
    _payload: &Value,
) -> Option<(&'static str, Value)> {
    Some(("ready", Value::Null))
}

pub(in crate::connection) fn parse_resumed(
    _state: &ConnectionState,
    _payload: &Value,
) -> Option<(&'static str, Value)> {
    Some(("resumed", Value::Null))
}
