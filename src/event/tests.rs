use super::*;
use crate::manage::C2CFriendData;
use crate::models::gateway::*;

#[test]
fn parse_and_handle_accepts_known_event_payload() {
    let body = br#"{"op":0,"t":"MESSAGE_CREATE","d":{"id":"1","content":"hello"}}"#;
    let mut payload = WSPayload {
        base: WSPayloadBase {
            op_code: WS_DISPATCH_EVENT,
            seq: None,
            event_type: Some(EVENT_MESSAGE_CREATE.to_string()),
            event_id: None,
        },
        data: None,
        raw_message: Some(body.to_vec()),
        session: None,
    };

    parse_and_handle(&mut payload).unwrap();
}

#[test]
fn parse_and_handle_rejects_invalid_known_event_payload() {
    let body = br#"{"op":0,"t":"MESSAGE_CREATE","d":"not a message object"}"#;
    let mut payload = WSPayload {
        base: WSPayloadBase {
            op_code: WS_DISPATCH_EVENT,
            seq: None,
            event_type: Some(EVENT_MESSAGE_CREATE.to_string()),
            event_id: None,
        },
        data: None,
        raw_message: Some(body.to_vec()),
        session: None,
    };

    assert!(parse_and_handle(&mut payload).is_err());
}

#[test]
fn parse_data_reads_c2c_friend_dto() {
    let body =
        br#"{"op":0,"t":"FRIEND_ADD","d":{"openid":"u1","timestamp":123,"nick":"n","avatar":"a"}}"#;
    let data: C2CFriendData = parse_data(body).unwrap();

    assert_eq!(data.openid, "u1");
    assert_eq!(data.timestamp, 123);
    assert_eq!(data.nick, "n");
    assert_eq!(data.avatar, "a");
}
