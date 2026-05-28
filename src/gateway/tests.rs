use super::*;
use crate::error::{
    WS_CODE_BACKEND_AUTHENTICATION_FAIL, WS_CODE_BACKEND_BOT_BANNED, WS_CODE_BACKEND_BOT_OFFLINE,
    WS_CODE_BACKEND_INVALID_SEQ, WS_CODE_BACKEND_SESSION_NO_LONGER_VALID,
};
use crate::intents::Intents;
use crate::token_impl::Token;
use std::time::Duration;

#[test]
fn test_gateway_creation() {
    let token = Token::new("test_app_id", "test_secret");
    let intents = Intents::default();
    let gateway = Gateway::new("wss://example.com", token, intents, None);

    assert!(!gateway.is_ready());
    assert!(gateway.session_id().is_none());
    assert_eq!(gateway.last_sequence(), 0);
    assert_eq!(gateway.shard, Some([0, 1]));
}

#[test]
fn test_gateway_with_shard() {
    let token = Token::new("test_app_id", "test_secret");
    let intents = Intents::default();
    let gateway = Gateway::new("wss://example.com", token, intents, Some([0, 1]));

    assert_eq!(gateway.shard, Some([0, 1]));
}

#[test]
fn identify_intents_fall_back_to_guilds_when_empty() {
    let token = Token::new("test_app_id", "test_secret");
    let gateway = Gateway::new("wss://example.com", token, Intents::new(), Some([0, 1]));

    assert_eq!(gateway.identify_intents(), Intents::GUILDS);
}

#[test]
fn identify_intents_keep_configured_bits() {
    let token = Token::new("test_app_id", "test_secret");
    let intents = Intents::new().with_public_messages();
    let gateway = Gateway::new("wss://example.com", token, intents, Some([0, 1]));

    assert_eq!(gateway.identify_intents(), Intents::PUBLIC_MESSAGES);
}

#[test]
fn test_session_start_interval() {
    assert_eq!(Gateway::session_start_interval(0), Duration::from_secs(2));
    assert_eq!(Gateway::session_start_interval(1), Duration::from_secs(2));
    assert_eq!(Gateway::session_start_interval(2), Duration::from_secs(1));
    assert_eq!(Gateway::session_start_interval(3), Duration::from_secs(1));
    assert_eq!(Gateway::session_start_interval(5), Duration::from_secs(1));
    assert_eq!(Gateway::session_start_interval(100), Duration::from_secs(1));
}

#[test]
fn heartbeat_interval_uses_hello_millis_with_minimum() {
    assert_eq!(
        Gateway::heartbeat_interval_duration(41_250),
        Duration::from_millis(41_250)
    );
    assert_eq!(
        Gateway::heartbeat_interval_duration(0),
        Duration::from_millis(1)
    );
}

#[test]
fn test_close_code_classification() {
    assert!(Gateway::cannot_resume_close_code(
        WS_CODE_BACKEND_SESSION_NO_LONGER_VALID
    ));
    assert!(Gateway::cannot_resume_close_code(
        WS_CODE_BACKEND_INVALID_SEQ
    ));
    assert!(Gateway::cannot_identify_close_code(
        WS_CODE_BACKEND_BOT_OFFLINE
    ));
    assert!(Gateway::cannot_identify_close_code(
        WS_CODE_BACKEND_BOT_BANNED
    ));
    assert!(!Gateway::cannot_resume_close_code(
        WS_CODE_BACKEND_AUTHENTICATION_FAIL
    ));
}
