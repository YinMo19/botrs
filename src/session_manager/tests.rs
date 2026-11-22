use tokio::sync::mpsc;

use super::channel::SessionFuture;
use super::*;
use crate::error::{CODE_CONN_CLOSE_CANT_IDENTIFY, CODE_CONN_CLOSE_CANT_RESUME, sdk_error};
use crate::intents::Intents;
use crate::models::api::{GatewayResponse, SessionStartLimit};
use crate::token_impl::Token;

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
fn check_session_limit_matches_expected() {
    assert!(check_session_limit(&ap_info(2, 2, 1)).is_ok());

    let err = check_session_limit(&ap_info(3, 2, 1)).unwrap_err();
    assert!(can_not_identify(&err));
}

#[test]
fn resume_and_identify_error_sets_match_expected() {
    let resume = sdk_error(CODE_CONN_CLOSE_CANT_RESUME, "invalid session").into();
    let identify = sdk_error(CODE_CONN_CLOSE_CANT_IDENTIFY, "bot banned").into();
    let other = sdk_error(9999, "unknown sdk error").into();

    assert!(can_not_resume(&resume));
    assert!(!can_not_resume(&identify));
    assert!(can_not_identify(&identify));
    assert!(!can_not_identify(&resume));
    assert!(!can_not_identify(&other));
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

#[tokio::test]
async fn non_resumable_error_clears_session_before_requeue() {
    let (event_tx, _event_rx) = mpsc::channel(16);
    let connect_fn = std::sync::Arc::new(|session: Session, _event_sender| -> SessionFuture {
        Box::pin(async move {
            let mut next = session;
            next.id = "stale-session".to_string();
            next.last_seq = 42;
            (
                next,
                Err(sdk_error(CODE_CONN_CLOSE_CANT_RESUME, "invalid session").into()),
            )
        })
    });

    let session = ChanManager::new_connect(
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
    .await
    .unwrap();
    assert!(session.id.is_empty());
    assert_eq!(session.last_seq, 0);
}

#[tokio::test]
async fn terminal_identify_error_stops_manager_and_closes_event_channel() {
    let mut manager = ChanManager::with_connect_fn(|session, _| {
        Box::pin(async {
            (
                session,
                Err(sdk_error(CODE_CONN_CLOSE_CANT_IDENTIFY, "banned").into()),
            )
        })
    });
    let (tx, mut rx) = mpsc::channel(16);
    let info = ap_info(2, 2, 1);
    let run = manager.start(&info, Token::new("app", "secret"), Intents::new(), tx);
    let err = tokio::time::timeout(std::time::Duration::from_secs(2), run)
        .await
        .unwrap()
        .unwrap_err();
    assert!(can_not_identify(&err));
    assert!(rx.recv().await.is_none());
}

#[tokio::test]
async fn cancelling_manager_drops_active_shard_and_event_sender() {
    let info = ap_info(1, 1, 1);
    let (tx, mut rx) = mpsc::channel(16);
    let mut manager = ChanManager::with_connect_fn(|_, tx| {
        Box::pin(async move {
            let started =
                serde_json::from_value(serde_json::json!({"op":0,"t":"RESUMED"})).unwrap();
            tx.send(started).await.unwrap();
            let result = std::future::pending().await;
            drop(tx);
            result
        })
    });
    let mut run = Box::pin(manager.start(&info, Token::new("app", "secret"), Intents::new(), tx));
    tokio::select! {
        result = &mut run => panic!("unexpected manager exit: {result:?}"),
        event = rx.recv() => assert!(event.is_some()),
    }
    drop(run);
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
            .await
            .unwrap()
            .is_none()
    );
}
