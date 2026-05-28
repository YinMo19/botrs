use super::*;
use crate::intents::Intents;

#[test]
fn gateway_event_omits_absent_wire_fields() {
    let event = GatewayEvent {
        id: None,
        event_type: None,
        data: Some(serde_json::json!({
            "token": "QQBot token",
            "intents": Intents::PUBLIC_MESSAGES,
            "shard": [0, 1],
        })),
        sequence: None,
        opcode: opcodes::IDENTIFY,
    };

    let value = serde_json::to_value(&event).unwrap();

    assert_eq!(value["op"], opcodes::IDENTIFY);
    assert!(value.get("d").is_some());
    assert!(value.get("id").is_none());
    assert!(value.get("t").is_none());
    assert!(value.get("s").is_none());
}

#[test]
fn identify_properties_default_to_zero_value() {
    let identify = Identify {
        token: "QQBot ACCESS_TOKEN_XXXXXX".to_string(),
        intents: Intents::PUBLIC_MESSAGES,
        shard: Some([0, 1]),
        properties: IdentifyProperties::default(),
    };

    let value = serde_json::to_value(&identify).unwrap();

    assert_eq!(
        value,
        serde_json::json!({
            "token": "QQBot ACCESS_TOKEN_XXXXXX",
            "intents": Intents::PUBLIC_MESSAGES,
            "shard": [0, 1],
            "properties": {}
        })
    );
}
