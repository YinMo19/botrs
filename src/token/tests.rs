use super::*;

#[test]
fn test_token_creation() {
    let token = Token::new("123456", "secret123");
    assert_eq!(token.app_id(), "123456");
    assert_eq!(token.secret(), "secret123");
}

#[tokio::test]
async fn bot_token_and_header_use_the_same_authorization_format() {
    let token = Token::new("test", "secret");
    token.set_cached_access_token_for_test("access-token").await;
    assert_eq!(token.bot_token().await.unwrap(), "QQBot access-token");
    assert_eq!(
        token.authorization_header().await.unwrap(),
        "QQBot access-token"
    );
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
