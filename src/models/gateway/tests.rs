use super::*;
use crate::intents::Intents;

#[test]
fn test_event_to_intent_matches_expected_mapping() {
    let intent = event_to_intent([
        EventGuildCreate,
        EventChannelDelete,
        EventGuildMemberAdd,
        EventMessageCreate,
        EventGroupAtMessageCreate,
        EventC2CFriendDel,
        EventEnterAIO,
        "UNKNOWN_EVENT",
    ]);

    assert_eq!(intent & Intents::GUILDS, Intents::GUILDS);
    assert_eq!(intent & Intents::GUILD_MEMBERS, Intents::GUILD_MEMBERS);
    assert_eq!(intent & Intents::GUILD_MESSAGES, Intents::GUILD_MESSAGES);
    assert_eq!(intent & Intents::PUBLIC_MESSAGES, Intents::PUBLIC_MESSAGES);
    assert_eq!(intent & Intents::ENTER_AIO, Intents::ENTER_AIO);
    assert_eq!(event_to_intent(["UNKNOWN_EVENT"]), 0);
}

#[test]
fn test_event_to_intent_maps_public_and_forum_events() {
    assert_eq!(
        event_to_intent([EventAtMessageCreate, EventForumAuditResult]),
        Intents::PUBLIC_GUILD_MESSAGES | Intents::FORUMS
    );
}

#[test]
fn websocket_payload_keeps_session_out_of_json() {
    let mut payload = WSPayload::from(GatewayEvent {
        id: Some("event-id".to_string()),
        event_type: Some(EventMessageCreate.to_string()),
        data: Some(serde_json::json!({"content": "hello"})),
        sequence: Some(7),
        opcode: WSDispatchEvent,
    });
    payload.session = Some(crate::session_manager::Session::new(
        "wss://example.com",
        crate::Token::new("app", "secret"),
        crate::Intents::default(),
        0,
        1,
    ));

    let value = serde_json::to_value(&payload).unwrap();

    assert!(value.get("session").is_none());
    assert_eq!(value["op"], WSDispatchEvent);
    assert_eq!(value["s"], 7);
    assert_eq!(value["t"], EventMessageCreate);
    assert_eq!(value["id"], "event-id");
}

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
        opcode: WSIdentity,
    };

    let value = serde_json::to_value(&event).unwrap();

    assert_eq!(value["op"], WSIdentity);
    assert!(value.get("d").is_some());
    assert!(value.get("id").is_none());
    assert!(value.get("t").is_none());
    assert!(value.get("s").is_none());
}

#[test]
fn identify_properties_default_to_botgo_zero_value() {
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
