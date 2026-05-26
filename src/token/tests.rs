use super::*;

#[test]
fn test_token_creation() {
    let token = Token::new("123456", "secret123");
    assert_eq!(token.app_id(), "123456");
    assert_eq!(token.secret(), "secret123");
}

#[tokio::test]
async fn test_authorization_header() {
    let token = Token::new("test", "secret");

    // Since we don't have real credentials, this should fail
    let result = token.authorization_header().await;
    assert!(
        result.is_err(),
        "Expected authorization_header to fail with invalid credentials"
    );
}

#[tokio::test]
async fn test_bot_token() {
    let token = Token::new("test", "secret");
    // In real usage, both methods would fetch the same access token
    // For this test, we just verify they both start with "QQBot "
    let bot_token_result = token.bot_token().await;
    let auth_header_result = token.authorization_header().await;

    // Both should fail in the same way since we don't have real credentials
    assert!(bot_token_result.is_err());
    assert!(auth_header_result.is_err());
}

#[tokio::test]
async fn cloned_tokens_share_cached_access_token() {
    let token = Token::new("123", "secret");
    {
        let mut state = token.state.lock().await;
        state.access_token = Some("cached-token".to_string());
        state.expires_at = Some(u64::MAX);
        state.expires_in = Some(7200);
    }

    let cloned = token.clone();
    assert_eq!(
        cloned.authorization_header().await.unwrap(),
        "QQBot cached-token"
    );
    assert_eq!(cloned.cached_expires_in().await, Some(7200));
}

#[test]
fn refresh_millis_matches_expected_bounds() {
    assert_eq!(get_refresh_millis(8), 8_000);
    assert_eq!(get_refresh_millis(9), 0);

    let refresh_millis = get_refresh_millis(10);
    assert!((501..=1_000).contains(&refresh_millis));

    let refresh_millis = get_refresh_millis(7200);
    assert!((7_190_501..=7_191_000).contains(&refresh_millis));
}

#[test]
fn parse_expires_in_accepts_number_or_string() {
    assert_eq!(parse_expires_in(&serde_json::json!("7200")), Some(7200));
    assert_eq!(parse_expires_in(&serde_json::json!(7200)), Some(7200));
    assert_eq!(parse_expires_in(&serde_json::json!("bad")), None);
}

#[test]
fn test_validation() {
    let valid_token = Token::new("123", "secret");
    assert!(valid_token.validate().is_ok());

    let empty_app_id = Token::new("", "secret");
    assert!(empty_app_id.validate().is_err());

    let empty_secret = Token::new("123", "");
    assert!(empty_secret.validate().is_err());
}

#[test]
fn test_safe_display() {
    let token = Token::new("123456", "verylongsecret123");
    let display = token.safe_display();
    assert!(display.contains("123456"));
    assert!(display.contains("very"));
    assert!(display.contains("123"));
    assert!(display.contains("****"));
    assert!(!display.contains("longsecret"));

    let short_token = Token::new("123", "short");
    let short_display = short_token.safe_display();
    assert!(short_display.contains("****"));
    assert!(!short_display.contains("short"));
}

#[test]
fn test_debug_format() {
    let token = Token::new("123456", "secret123");
    let debug_str = format!("{:?}", token);
    assert!(debug_str.contains("123456"));
    assert!(debug_str.contains("[REDACTED]"));
    assert!(!debug_str.contains("secret123"));
}

#[test]
fn test_token_source_alias() {
    let credentials = QQBotCredentials {
        app_id: "123456".to_string(),
        app_secret: "secret123".to_string(),
    };
    let token = NewQQBotTokenSource(&credentials);
    assert_eq!(token.app_id(), "123456");
    assert_eq!(token.GetAppID(), "123456");
    assert_eq!(token.secret(), "secret123");
    assert_eq!(TypeQQBot, "QQBot");
    assert_eq!(TypeBearer, "Bearer");
}
