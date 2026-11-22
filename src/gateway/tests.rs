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

#[tokio::test]
async fn fatal_close_after_identify_propagates_to_the_session_manager() {
    use crate::models::api::{GatewayResponse, SessionStartLimit};
    use crate::session_manager::{ChanManager, SessionManager, can_not_identify};
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::{Message, protocol::CloseFrame};

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        socket
            .send(Message::Text(
                r#"{"op":10,"d":{"heartbeat_interval":60000}}"#.into(),
            ))
            .await
            .unwrap();
        let identify = socket.next().await.unwrap().unwrap().into_text().unwrap();
        let identify: serde_json::Value = serde_json::from_str(&identify).unwrap();
        assert_eq!(identify["op"], 2);
        assert_eq!(identify["d"]["shard"], serde_json::json!([0, 1]));
        socket
            .close(Some(CloseFrame {
                code: WS_CODE_BACKEND_BOT_OFFLINE.into(),
                reason: "offline".into(),
            }))
            .await
            .unwrap();
    });
    let token = Token::new("app", "secret");
    token
        .set_cached_access_token_for_test("test-access-token")
        .await;
    let info = GatewayResponse {
        url,
        shards: 1,
        session_start_limit: SessionStartLimit {
            total: 1,
            remaining: 1,
            reset_after: 1000,
            max_concurrency: 1,
        },
    };
    let (tx, mut rx) = tokio::sync::mpsc::channel(16);
    let mut manager = ChanManager::new();
    let run = manager.start(&info, token, Intents::new(), tx);
    let error = tokio::time::timeout(Duration::from_secs(3), run)
        .await
        .unwrap()
        .unwrap_err();
    assert!(can_not_identify(&error));
    assert!(rx.recv().await.is_none());
    server.await.unwrap();
}
