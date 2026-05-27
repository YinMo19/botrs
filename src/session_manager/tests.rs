use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;

use super::*;
use crate::error::{
    CodeConnCloseCantIdentify, CodeConnCloseCantResume, CodeNeedReConnect, sdk_error,
};
use crate::intents::Intents;
use crate::models::api::{GatewayResponse, SessionStartLimit};
use crate::token::Token;

fn ap_info(shards: u32, remaining: u32, max_concurrency: u32) -> GatewayResponse {
    GatewayResponse {
        url: "wss://example.com".to_string(),
        shards,
        session_start_limit: SessionStartLimit {
            total: 10,
            remaining,
            reset_after: 1000,
            max_concurrency,
        },
    }
}

#[test]
fn calc_interval_matches_expected() {
    assert_eq!(calc_interval(0), Duration::from_secs(2));
    assert_eq!(calc_interval(1), Duration::from_secs(2));
    assert_eq!(calc_interval(2), Duration::from_secs(1));
    assert_eq!(calc_interval(3), Duration::from_secs(1));
    assert_eq!(calc_interval(100), Duration::from_secs(1));
}

#[test]
fn check_session_limit_matches_expected() {
    assert!(check_session_limit(&ap_info(2, 2, 1)).is_ok());

    let err = check_session_limit(&ap_info(3, 2, 1)).unwrap_err();
    assert!(can_not_identify(&err));
}

#[test]
fn resume_and_identify_error_sets_match_expected() {
    let resume = sdk_error(CodeConnCloseCantResume, "invalid session");
    let identify = sdk_error(CodeConnCloseCantIdentify, "bot banned");
    let reconnect = sdk_error(CodeNeedReConnect, "need reconnect");

    assert!(can_not_resume(&resume));
    assert!(!can_not_resume(&identify));
    assert!(can_not_identify(&identify));
    assert!(!can_not_identify(&resume));
    assert!(!can_not_identify(&reconnect));
}

#[test]
fn sessions_are_generated_per_shard() {
    let token = Token::new("app_id", "secret");
    let sessions = ChanManager::sessions(&ap_info(3, 3, 1), token, Intents::default());

    let shards = sessions
        .into_iter()
        .map(|session| session.shard())
        .collect::<Vec<_>>();
    assert_eq!(shards, vec![[0, 3], [1, 3], [2, 3]]);
}

#[test]
fn app_id_session_matches_webhook_shape() {
    let session = Session::from_app_id("app-id-1");

    assert_eq!(session.app_id.as_deref(), Some("app-id-1"));
    assert!(session.id.is_empty());
    assert_eq!(session.last_seq, 0);
    assert_eq!(session.shard(), [0, 0]);
}

#[tokio::test]
async fn non_resumable_error_clears_session_before_requeue() {
    let (session_tx, mut session_rx) = mpsc::unbounded_channel();
    let (event_tx, _event_rx) = mpsc::unbounded_channel();
    let connect_fn: Arc<SessionConnectFn> = Arc::new(|session, _event_sender| {
        Box::pin(async move {
            let mut next = session;
            next.id = "stale-session".to_string();
            next.last_seq = 42;
            (
                next,
                Err(sdk_error(CodeConnCloseCantResume, "invalid session").into()),
            )
        })
    });

    ChanManager::new_connect(
        session_tx,
        connect_fn,
        event_tx,
        Session::new(
            "wss://example.com",
            Token::new("app_id", "secret"),
            Intents::default(),
            0,
            1,
        ),
    )
    .await;

    let session = session_rx.recv().await.unwrap();
    assert!(session.id.is_empty());
    assert_eq!(session.last_seq, 0);
}
