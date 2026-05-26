use super::*;
use crate::token::Token;
use reqwest::StatusCode;
use std::time::Duration;

#[test]
fn test_http_client_creation() {
    let client = HttpClient::new(30, false).unwrap();
    assert!(!client.is_sandbox());
    assert_eq!(client.timeout(), Duration::from_secs(30));
    assert_eq!(client.base_url(), crate::DEFAULT_API_URL);

    let sandbox_client = HttpClient::new(60, true).unwrap();
    assert!(sandbox_client.is_sandbox());
    assert_eq!(sandbox_client.base_url(), crate::SANDBOX_API_URL);
}

#[test]
fn test_api_error_parsing() {
    let client = HttpClient::new(30, false).unwrap();

    let json = serde_json::json!({
        "code": 404,
        "message": "Not found",
        "trace_id": "test-trace"
    });

    let error = client
        .parse_api_error(StatusCode::NOT_FOUND, &json)
        .unwrap();
    assert_eq!(error.code, 404);
    assert_eq!(error.message, "Not found");
    assert_eq!(error.trace_id, Some("test-trace".to_string()));
}

#[test]
fn test_rate_limit_parsing() {
    let client = HttpClient::new(30, false).unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-ratelimit-limit", "100".parse().unwrap());
    headers.insert("x-ratelimit-remaining", "50".parse().unwrap());
    headers.insert("x-ratelimit-reset", "1234567890".parse().unwrap());
    headers.insert("x-ratelimit-bucket", "global".parse().unwrap());

    let rate_limit = client.parse_rate_limit(&headers).unwrap();
    assert_eq!(rate_limit.limit, 100);
    assert_eq!(rate_limit.remaining, 50);
    assert_eq!(rate_limit.reset, 1234567890);
    assert_eq!(rate_limit.bucket, Some("global".to_string()));
}

#[test]
fn test_trace_id_storage() {
    let client = HttpClient::new(30, false).unwrap();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-Tps-trace-ID", "trace-123".parse().unwrap());

    client.store_trace_id(&headers);

    assert_eq!(client.trace_id(), "trace-123");
}

#[tokio::test]
async fn authorized_headers_include_union_app_id() {
    let client = HttpClient::new(30, false)
        .unwrap()
        .with_union_app_id("openapi-app");
    let token = Token::new("token-app", "secret");
    token.set_cached_access_token_for_test("cached-token").await;

    let headers = client
        .authorized_headers(&token, reqwest::header::HeaderMap::new())
        .await
        .unwrap();

    assert_eq!(
        headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok()),
        Some("QQBot cached-token")
    );
    assert_eq!(
        headers
            .get("X-Union-Appid")
            .and_then(|value| value.to_str().ok()),
        Some("openapi-app")
    );
}

#[tokio::test]
async fn authorized_headers_fall_back_to_token_app_id() {
    let client = HttpClient::new(30, false).unwrap();
    let token = Token::new("token-app", "secret");
    token.set_cached_access_token_for_test("cached-token").await;

    let headers = client
        .authorized_headers(&token, reqwest::header::HeaderMap::new())
        .await
        .unwrap();

    assert_eq!(
        headers
            .get("X-Union-Appid")
            .and_then(|value| value.to_str().ok()),
        Some("token-app")
    );
}
