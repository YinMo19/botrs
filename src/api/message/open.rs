use crate::api_impl::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{C2CMessageParams, GroupMessageParams, MessageToCreate},
};
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Sends a group message. For proactive delivery use params without a
    /// `msg_id` or `event_id`, e.g. `GroupMessageParams::new_text("hello")`.
    /// QQ enforces the group's proactive permission and platform quotas on send.
    pub async fn send_group_message(
        &self,
        group_openid: &str,
        params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending group message to {}", group_openid);
        let body = MessageToCreate::from(params);
        let path = resource::group_messages(group_openid);
        self.request_message_response_body(Method::POST, &path, &body)
            .await
    }

    /// Sends a C2C (client-to-client) message using C2CMessageParams.
    pub async fn send_c2c_message(
        &self,
        openid: &str,
        params: C2CMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending C2C message to {}", openid);
        let body = MessageToCreate::from(params);
        let path = resource::c2c_messages(openid);
        self.request_message_response_body(Method::POST, &path, &body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn test_api(base_url: String) -> BotApi {
        let token = crate::Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let http = crate::http::HttpClient::new(30, false)
            .unwrap()
            .with_base_url(base_url.parse().unwrap())
            .unwrap();
        BotApi::new(http, token)
    }

    async fn spawn_capture_server() -> (
        String,
        oneshot::Receiver<String>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request_bytes = Vec::new();
            let mut buffer = [0_u8; 4096];
            loop {
                let n = stream.read(&mut buffer).await.unwrap();
                request_bytes.extend_from_slice(&buffer[..n]);

                let request = String::from_utf8_lossy(&request_bytes);
                let Some(header_end) = request.find("\r\n\r\n") else {
                    continue;
                };
                let content_length = request
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .unwrap_or(0);
                let body_start = header_end + 4;
                if request_bytes.len().saturating_sub(body_start) >= content_length {
                    break;
                }
            }

            let request = String::from_utf8_lossy(&request_bytes).to_string();
            let _ = tx.send(request);

            let body = r#"{"id":"message-1","timestamp":1700000000}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    fn request_body(request: &str) -> serde_json::Value {
        let body = request.split("\r\n\r\n").nth(1).unwrap_or_default();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn scoped_url_changes_only_the_selected_request() {
        let (base, normal_request, normal_server) = spawn_capture_server().await;
        let (other, custom_request, custom_server) = spawn_capture_server().await;
        let api = test_api(base).await;
        let url = format!("{other}/custom?trace=1").parse().unwrap();
        api.with_url(url)
            .send_group_message("group", GroupMessageParams::new_text("hello"))
            .await
            .unwrap();
        api.send_group_message("group", GroupMessageParams::new_text("hello"))
            .await
            .unwrap();
        assert!(
            custom_request
                .await
                .unwrap()
                .starts_with("POST /custom?trace=1 ")
        );
        assert!(
            normal_request
                .await
                .unwrap()
                .starts_with("POST /v2/groups/group/messages ")
        );
        custom_server.await.unwrap();
        normal_server.await.unwrap();
    }

    #[tokio::test]
    async fn transport_returns_headers_and_unparsed_body() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = axum::Router::new().route(
            "/raw",
            axum::routing::post(
                |headers: axum::http::HeaderMap, bytes: axum::body::Bytes| async move {
                    assert_eq!(headers["authorization"], "QQBot ACCESS_TOKEN_XXXXXX");
                    assert_eq!(&bytes[..], &[0xff, 0x00, 0x01]);
                    ([("x-tps-trace-id", "trace-raw")], "plain text")
                },
            ),
        );
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let api = test_api(format!("http://{addr}")).await;
        let response = api
            .transport(
                Method::POST,
                &format!("http://{addr}/raw").parse().unwrap(),
                |r| r.body(vec![0xff, 0x00, 0x01]),
            )
            .await
            .unwrap();
        assert_eq!(response.headers()["x-tps-trace-id"], "trace-raw");
        assert_eq!(response.text().await.unwrap(), "plain text");
        server.abort();
    }

    #[tokio::test]
    async fn group_state_is_an_authenticated_get_with_independent_permissions() {
        use crate::models::group::{GroupMemberRole, GroupReceiveMode};
        use axum::{Json, Router, extract::Path, http::HeaderMap, routing::get};
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let api = test_api(format!("http://{}", listener.local_addr().unwrap())).await;
        let app = Router::new().route(
            "/v2/groups/{group}/bot_state",
            get(|Path(group): Path<String>, headers: HeaderMap| async move {
                assert_eq!(headers["authorization"], "QQBot ACCESS_TOKEN_XXXXXX");
                Json(serde_json::json!({
                    "member_openid": "bot", "joined_at": "2025-06-15T14:30:00+08:00",
                    "allow_proactive_msg": group == "only_mention",
                    "recv_msg_setting": group, "member_role": "admin"
                }))
            }),
        );
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        for (wire, mode) in [
            ("only_mention", GroupReceiveMode::OnlyMention),
            ("mention_and_context", GroupReceiveMode::MentionAndContext),
            ("all", GroupReceiveMode::All),
            ("future", GroupReceiveMode::Other("future".into())),
        ] {
            let state = api.get_group_bot_state(wire).await.unwrap();
            assert_eq!(state.recv_msg_setting, mode);
            assert_eq!(state.allow_proactive_msg, wire == "only_mention");
            assert_eq!(state.member_role, GroupMemberRole::Admin);
            assert_eq!(serde_json::to_value(mode).unwrap(), wire);
        }
        server.abort();
    }

    #[tokio::test]
    async fn group_message_params_send_message_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .send_group_message("group-openid-1", GroupMessageParams::new_text("hello"))
            .await
            .unwrap();

        assert_eq!(response.timestamp.as_deref(), Some("1700000000"));
        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /v2/groups/group-openid-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello",
                "msg_type": 0
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn c2c_message_params_send_message_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .send_c2c_message("openid-1", C2CMessageParams::new_text("hello"))
            .await
            .unwrap();

        assert_eq!(response.timestamp.as_deref(), Some("1700000000"));
        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /v2/users/openid-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello",
                "msg_type": 0
            })
        );
        server.await.unwrap();
    }
}
