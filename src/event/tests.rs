use super::*;
use crate::models::gateway::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static MESSAGE_COUNT: AtomicUsize = AtomicUsize::new(0);

fn message_handler(_: &mut WSPayload, _: &mut WSMessageData) -> crate::Result<()> {
    MESSAGE_COUNT.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

#[test]
fn register_handlers_returns_intents() {
    let intent = register_handlers([MessageEventHandler(message_handler)]);
    assert_eq!(
        intent & crate::intents::Intents::GUILD_MESSAGES,
        crate::intents::Intents::GUILD_MESSAGES
    );
}

#[test]
fn parse_and_handle_dispatches_typed_handler() {
    register_handlers([MessageEventHandler(message_handler)]);
    let body = br#"{"op":0,"t":"MESSAGE_CREATE","d":{"id":"1","content":"hello"}}"#;
    let mut payload = WSPayload {
        base: WSPayloadBase {
            op_code: WSDispatchEvent,
            seq: None,
            event_type: Some(EventMessageCreate.to_string()),
            event_id: None,
        },
        data: None,
        raw_message: Some(body.to_vec()),
        session: None,
    };

    parse_and_handle(&mut payload).unwrap();
    assert!(MESSAGE_COUNT.load(Ordering::Relaxed) > 0);
}

#[test]
fn parse_data_reads_c2c_friend_dto() {
    let body =
        br#"{"op":0,"t":"FRIEND_ADD","d":{"openid":"u1","timestamp":123,"nick":"n","avatar":"a"}}"#;
    let data: WSC2CFriendData = parse_data(body).unwrap();

    assert_eq!(data.openid, "u1");
    assert_eq!(data.timestamp, 123);
    assert_eq!(data.nick, "n");
    assert_eq!(data.avatar, "a");
}
