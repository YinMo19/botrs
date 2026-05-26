use super::*;
use crate::intents::{
    IntentEnterAIO, IntentForum, IntentGroupMessages, IntentGuildAtMessage, IntentGuildMembers,
    IntentGuildMessages, IntentGuilds, IntentNone,
};

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

    assert_eq!(intent & IntentGuilds, IntentGuilds);
    assert_eq!(intent & IntentGuildMembers, IntentGuildMembers);
    assert_eq!(intent & IntentGuildMessages, IntentGuildMessages);
    assert_eq!(intent & IntentGroupMessages, IntentGroupMessages);
    assert_eq!(intent & IntentEnterAIO, IntentEnterAIO);
    assert_eq!(event_to_intent(["UNKNOWN_EVENT"]), IntentNone);
}

#[test]
fn test_event_to_intent_function_name() {
    assert_eq!(
        EventToIntent([EventAtMessageCreate, EventForumAuditResult]),
        IntentGuildAtMessage | IntentForum
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
            "intents": crate::intents::IntentGroupMessages,
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
