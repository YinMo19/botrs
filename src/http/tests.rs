use super::*;
use crate::token::Token;
use reqwest::StatusCode;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[test]
fn test_http_client_creation() {
    let client = HttpClient::new(30, false).unwrap();
    assert!(!client.is_sandbox());
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

    let error = client.parse_api_error(StatusCode::NOT_FOUND, &json);
    assert_eq!(error.code, 404);
    assert_eq!(error.err_code, None);
    assert_eq!(error.message, "Not found");
    assert_eq!(error.trace_id, Some("test-trace".to_string()));
}

#[test]
fn api_error_parsing_prefers_official_err_code() {
    let client = HttpClient::new(30, false).unwrap();

    let json = serde_json::json!({
        "code": 0,
        "err_code": 11244,
        "message": "token expired",
        "trace_id": "trace-err-code"
    });

    let error = client.parse_api_error(StatusCode::UNAUTHORIZED, &json);
    assert_eq!(error.code, 11244);
    assert_eq!(error.err_code, Some(11244));
    assert_eq!(error.message, "token expired");
    assert_eq!(error.trace_id, Some("trace-err-code".to_string()));
}

#[tokio::test]
async fn no_content_response_is_successful_null() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = [0_u8; 4096];
        let _ = stream.read(&mut buffer).await.unwrap();
        stream
            .write_all(b"HTTP/1.1 204 No Content\r\ncontent-length: 0\r\nconnection: close\r\n\r\n")
            .await
            .unwrap();
    });

    let client = HttpClient::new(30, false).unwrap();
    let token = Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
    token
        .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
        .await;

    let response = client
        .request_json_url(
            &token,
            reqwest::Method::DELETE,
            &format!("http://{addr}/resource"),
            None::<&()>,
            None::<&()>,
        )
        .await
        .unwrap();

    assert_eq!(response, serde_json::Value::Null);
    server.await.unwrap();
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
