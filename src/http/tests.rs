use super::*;
use crate::error::BotError;
use crate::token_impl::Token;
use reqwest::StatusCode;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[test]
fn test_http_client_creation() {
    let client = HttpClient::new(30, false).unwrap();
    assert_eq!(client.base_url, crate::DEFAULT_API_URL);

    let sandbox_client = HttpClient::new(60, true).unwrap();
    assert_eq!(sandbox_client.base_url, crate::SANDBOX_API_URL);
}

#[tokio::test]
async fn failures_keep_status_trace_and_body_even_when_the_body_is_not_json() {
    let client = HttpClient::new(30, false).unwrap();
    for status in [StatusCode::BAD_GATEWAY, StatusCode::TOO_MANY_REQUESTS] {
        let response = axum::http::Response::builder()
            .status(status)
            .header("x-tps-trace-id", "trace")
            .header("retry-after", "7")
            .body("unavailable")
            .unwrap();
        let error = client.checked_response(response.into()).await.unwrap_err();
        let failure = error.response().unwrap();
        assert_eq!(failure.status, status);
        assert_eq!(failure.trace_id.as_deref(), Some("trace"));
        assert_eq!(failure.body, "unavailable");
        assert_eq!(failure.platform_code, None);
        if status == StatusCode::TOO_MANY_REQUESTS {
            assert_eq!(error.retry_after(), Some(7));
        }
    }
}

#[tokio::test]
async fn api_error_keeps_message_and_raw_audit_fields() {
    let client = HttpClient::new(30, false).unwrap();
    let body = r#"{"code":0,"err_code":304023,"message":"under audit","data":{"message_audit":{"audit_id":"audit-1"}}}"#;
    let response = axum::http::Response::builder()
        .status(StatusCode::ACCEPTED)
        .header("x-tps-trace-id", "trace")
        .body(body)
        .unwrap();
    let err = client.checked_response(response.into()).await.unwrap_err();
    let diagnostic = err.response().unwrap();
    assert_eq!(diagnostic.status, StatusCode::ACCEPTED);
    assert_eq!(diagnostic.platform_code, Some(304023));
    assert_eq!(diagnostic.body, body);
    assert!(matches!(err, BotError::Api { code: 304023, message, .. } if message == "under audit"));
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
        .with_base_url(format!("http://{addr}").parse().unwrap())
        .unwrap()
        .request(
            reqwest::Method::DELETE,
            &token,
            "/resource",
            None::<&()>,
            None::<&()>,
        )
        .await
        .unwrap();

    assert_eq!(response, serde_json::Value::Null);
    server.await.unwrap();
}

#[tokio::test]
async fn api_error_response_preserves_platform_code_and_trace() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = [0_u8; 4096];
        let _ = stream.read(&mut buffer).await.unwrap();

        let body = r#"{"code":0,"err_code":50015006,"message":"系统繁忙，请稍后重试"}"#;
        let response = format!(
            "HTTP/1.1 500 Internal Server Error\r\n\
             content-type: application/json\r\n\
             x-tps-trace-id: trace-from-header\r\n\
             content-length: {}\r\n\
             connection: close\r\n\r\n\
             {}",
            body.len(),
            body
        );

        stream.write_all(response.as_bytes()).await.unwrap();
    });

    let client = HttpClient::new(30, false).unwrap();
    let token = Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
    token
        .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
        .await;

    let err = client
        .with_base_url(format!("http://{addr}").parse().unwrap())
        .unwrap()
        .request(
            reqwest::Method::POST,
            &token,
            "/resource",
            None::<&()>,
            Some(&serde_json::json!({"msg_type": 4})),
        )
        .await
        .unwrap_err();

    assert!(matches!(err, BotError::Server(_)));
    assert_eq!(err.response().unwrap().platform_code, Some(50015006));
    let message = err.to_string();
    assert!(message.contains("HTTP 500 Internal Server Error"));
    assert!(message.contains("API error 50015006"));
    assert!(message.contains("系统繁忙，请稍后重试"));
    assert!(message.contains("trace_id=trace-from-header"));

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

    let mut headers = reqwest::header::HeaderMap::new();
    client.authorize(&token, &mut headers).await.unwrap();

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

    let mut headers = reqwest::header::HeaderMap::new();
    client.authorize(&token, &mut headers).await.unwrap();

    assert_eq!(
        headers
            .get("X-Union-Appid")
            .and_then(|value| value.to_str().ok()),
        Some("token-app")
    );
}
