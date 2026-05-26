use super::*;
use crate::error::{
    WSCodeBackendAuthenticationFail, WSCodeBackendBotBanned, WSCodeBackendBotOffline,
    WSCodeBackendInvalidSeq, WSCodeBackendSessionNoLongerValid,
};
use crate::intents::Intents;
use crate::token::Token;
use std::time::Duration;

#[test]
fn test_gateway_creation() {
    let token = Token::new("test_app_id", "test_secret");
    let intents = Intents::default();
    let gateway = Gateway::new("wss://example.com", token, intents, None);

    assert!(!gateway.is_ready());
    assert!(gateway.session_id().is_none());
    assert_eq!(gateway.last_sequence(), 0);
}

#[test]
fn test_gateway_with_shard() {
    let token = Token::new("test_app_id", "test_secret");
    let intents = Intents::default();
    let gateway = Gateway::new("wss://example.com", token, intents, Some([0, 1]));

    assert_eq!(gateway.shard, Some([0, 1]));
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
fn test_close_code_classification() {
    assert!(Gateway::cannot_resume_close_code(
        WSCodeBackendSessionNoLongerValid
    ));
    assert!(Gateway::cannot_resume_close_code(WSCodeBackendInvalidSeq));
    assert!(Gateway::cannot_identify_close_code(WSCodeBackendBotOffline));
    assert!(Gateway::cannot_identify_close_code(WSCodeBackendBotBanned));
    assert!(!Gateway::cannot_resume_close_code(
        WSCodeBackendAuthenticationFail
    ));
}
