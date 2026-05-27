use super::*;

#[test]
fn test_pagination() {
    let pagination = Pagination::new(2, 10, 25);
    assert_eq!(pagination.total_pages, 3);
    assert!(pagination.has_prev);
    assert!(pagination.has_next);

    let last_page = Pagination::new(3, 10, 25);
    assert!(!last_page.has_next);
    assert!(last_page.has_prev);
}

#[test]
fn test_rate_limit() {
    let rate_limit = RateLimit {
        bucket: Some("global".to_string()),
        limit: 100,
        remaining: 0,
        reset: chrono::Utc::now().timestamp() as u64 + 60,
        retry_after: Some(60),
    };

    assert_eq!(rate_limit.remaining, 0);
    assert!(rate_limit.reset_in() > 0);
}

#[test]
fn test_api_error() {
    let error = ApiError::new(429, "Rate limited");
    assert_eq!(error.err_code, None);
    assert_eq!(error.code, 429);

    let auth_error = ApiError::new(401, "Unauthorized");
    assert_eq!(auth_error.code, 401);
}

#[test]
fn api_error_accepts_current_err_code_field() {
    let error: ApiError = serde_json::from_value(serde_json::json!({
        "err_code": 11244,
        "message": "token expired",
        "trace_id": "trace-err-code"
    }))
    .unwrap();

    assert_eq!(error.code, 11244);
    assert_eq!(error.err_code, Some(11244));
    assert_eq!(error.message, "token expired");
    assert_eq!(error.trace_id.as_deref(), Some("trace-err-code"));
}

#[test]
fn websocket_ap_keeps_official_json_shape() {
    let ap: GatewayResponse = serde_json::from_value(serde_json::json!({
        "url": "wss://api.sgroup.qq.com/websocket",
        "shards": 2,
        "session_start_limit": {
            "total": 10,
            "remaining": 9,
            "reset_after": 1000,
            "max_concurrency": 1
        }
    }))
    .unwrap();

    assert_eq!(ap.url, "wss://api.sgroup.qq.com/websocket");
    assert_eq!(ap.shards, 2);
    assert_eq!(ap.session_start_limit.total, 10);
    assert_eq!(ap.session_start_limit.remaining, 9);
    assert_eq!(ap.session_start_limit.reset_after, 1000);
    assert_eq!(ap.session_start_limit.max_concurrency, 1);

    let value = serde_json::to_value(&ap).unwrap();
    assert_eq!(value["session_start_limit"]["reset_after"], 1000);
}

#[test]
fn bot_info_keeps_current_user_extra_fields() {
    let bot: BotInfo = serde_json::from_value(serde_json::json!({
        "id": "bot-1",
        "username": "bot",
        "avatar": "https://example.com/avatar.png",
        "union_openid": "UNION_OPENID_XXXXXX",
        "union_user_account": "UNION_ACCOUNT_XXXXXX",
        "share_url": "https://example.com/share",
        "welcome_msg": "hello"
    }))
    .unwrap();

    assert_eq!(bot.id, "bot-1");
    assert_eq!(bot.username, "bot");
    assert_eq!(bot.avatar, "https://example.com/avatar.png");
    assert!(!bot.bot);
    assert_eq!(bot.union_openid, "UNION_OPENID_XXXXXX");
    assert_eq!(bot.union_user_account, "UNION_ACCOUNT_XXXXXX");
    assert_eq!(bot.share_url, "https://example.com/share");
    assert_eq!(bot.welcome_msg, "hello");
}

#[test]
fn audio_action_uses_required_zero_value_fields() {
    let action: AudioAction = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(action.guild_id, "");
    assert_eq!(action.channel_id, "");
    assert_eq!(action.audio_url, "");
    assert_eq!(action.text, "");
}

#[test]
fn audio_action_keeps_official_json_shape() {
    let action = AudioAction {
        guild_id: "guild-1".to_string(),
        channel_id: "channel-1".to_string(),
        audio_url: "https://example.com/audio.mp3".to_string(),
        text: "now playing".to_string(),
    };
    let value = serde_json::to_value(&action).unwrap();

    assert_eq!(value["guild_id"], "guild-1");
    assert_eq!(value["channel_id"], "channel-1");
    assert_eq!(value["audio_url"], "https://example.com/audio.mp3");
    assert_eq!(value["text"], "now playing");
}

#[test]
fn audio_action_from_value_tolerates_missing_fields() {
    let action = AudioAction::from_value(&serde_json::json!({
        "guild_id": "guild-1",
        "channel_id": 123,
    }));

    assert_eq!(action.guild_id, "guild-1");
    assert_eq!(action.channel_id, "");
    assert_eq!(action.audio_url, "");
    assert_eq!(action.text, "");
}

#[test]
fn pins_message_uses_required_zero_value_fields() {
    let pins: PinsMessage = serde_json::from_value(serde_json::json!({})).unwrap();

    assert!(pins.guild_id.is_empty());
    assert!(pins.channel_id.is_empty());
    assert!(pins.message_ids.is_empty());
}

#[test]
fn pins_message_keeps_official_json_shape() {
    let pins = PinsMessage {
        guild_id: "guild-1".to_string(),
        channel_id: "channel-1".to_string(),
        message_ids: vec!["message-1".to_string(), "message-2".to_string()],
    };
    let value = serde_json::to_value(&pins).unwrap();

    assert_eq!(value["guild_id"], "guild-1");
    assert_eq!(value["channel_id"], "channel-1");
    assert_eq!(value["message_ids"][0], "message-1");
}

#[test]
fn message_response_collects_extra_fields() {
    let response: MessageResponse = serde_json::from_value(serde_json::json!({
        "id": "message-1",
        "timestamp": "2026-01-01T00:00:00+08:00",
        "msg_seq": 1,
        "audit_id": "audit-1"
    }))
    .unwrap();

    assert_eq!(response.id.as_deref(), Some("message-1"));
    assert_eq!(
        response.timestamp.as_deref(),
        Some("2026-01-01T00:00:00+08:00")
    );
    assert_eq!(response.extra["msg_seq"], serde_json::json!(1));
    assert_eq!(response.extra["audit_id"], serde_json::json!("audit-1"));

    let value = serde_json::to_value(&response).unwrap();
    assert_eq!(value["id"], "message-1");
    assert_eq!(value["timestamp"], "2026-01-01T00:00:00+08:00");
    assert_eq!(value["msg_seq"], 1);
    assert_eq!(value["audit_id"], "audit-1");
}
