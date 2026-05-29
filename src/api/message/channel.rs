use crate::api_impl::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{Message, MessageParams, MessageToCreate, MessagesPager, SettingGuideParams},
};
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Fetches a single channel message by ID.
    pub async fn get_message(&self, channel_id: &str, message_id: &str) -> Result<Message> {
        debug!("Getting message {} in channel {}", message_id, channel_id);
        let path = resource::channel_message(channel_id, message_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        decode_message_response(response)
    }

    /// Lists channel messages using the provided pager.
    pub async fn list_messages(
        &self,
        channel_id: &str,
        pager: &MessagesPager,
    ) -> Result<Vec<Message>> {
        debug!("Listing messages in channel {}", channel_id);
        let path = resource::channel_messages(channel_id);
        let query = pager.to_query_params();
        let response = if query.is_empty() {
            self.http.get(self.token(), &path, None::<&()>).await?
        } else {
            self.http.get(self.token(), &path, Some(&query)).await?
        };
        Self::decode_json(response)
    }

    /// Sends a message to a channel using MessageParams.
    pub async fn send_message(
        &self,
        channel_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending message to channel {}", channel_id);
        let body = MessageToCreate::from(params);
        let path = resource::channel_messages(channel_id);
        self.request_message_response_body(Method::POST, &path, &body)
            .await
    }

    /// Updates a channel message.
    pub async fn update_message(
        &self,
        channel_id: &str,
        message_id: &str,
        params: MessageParams,
    ) -> Result<Message> {
        debug!("Updating message {} in channel {}", message_id, channel_id);
        let body = MessageToCreate::from(params);
        let path = resource::channel_message(channel_id, message_id);
        self.request_json(Method::PATCH, &path, None::<&()>, Some(&body))
            .await
    }

    /// Sends a channel setting guide message.
    pub async fn send_setting_guide(
        &self,
        channel_id: &str,
        params: SettingGuideParams,
    ) -> Result<Message> {
        debug!("Sending setting guide to channel {}", channel_id);
        let path = resource::channel_setting_guide(channel_id);
        self.request_json(Method::POST, &path, None::<&()>, Some(&params))
            .await
    }
}

fn decode_message_response(response: serde_json::Value) -> Result<Message> {
    if let Some(message) = response.get("message") {
        return BotApi::decode_json(message.clone());
    }
    BotApi::decode_json(response)
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
        let mut http = crate::http::HttpClient::new(30, false).unwrap();
        http.base_url = base_url;
        BotApi::new(http, token)
    }

    async fn spawn_capture_server() -> (
        String,
        oneshot::Receiver<String>,
        tokio::task::JoinHandle<()>,
    ) {
        spawn_capture_server_with_body(
            r#"{"id":"message-1","content":"hello","channel_id":"channel-1","guild_id":"guild-1","author":{"id":"user-1","username":"user","bot":false},"seq_in_channel":"1","timestamp":"2026-01-01T00:00:00+08:00"}"#,
        )
        .await
    }

    async fn spawn_capture_server_with_body(
        body: &'static str,
    ) -> (
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
                let is_chunked = request.lines().any(|line| {
                    let Some((name, value)) = line.split_once(':') else {
                        return false;
                    };
                    name.eq_ignore_ascii_case("transfer-encoding")
                        && value
                            .split(',')
                            .any(|encoding| encoding.trim().eq_ignore_ascii_case("chunked"))
                });
                let content_length = request.lines().find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())
                        .flatten()
                });
                let body_start = header_end + 4;
                if let Some(content_length) = content_length {
                    if request_bytes.len().saturating_sub(body_start) >= content_length {
                        break;
                    }
                } else if is_chunked {
                    if request[body_start..].contains("\r\n0\r\n\r\n") {
                        break;
                    }
                } else {
                    break;
                }
            }

            let request = String::from_utf8_lossy(&request_bytes).to_string();
            let _ = tx.send(request);

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
        let body = request
            .split_once("\r\n\r\n")
            .map(|(_, body)| body)
            .unwrap_or_default();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn send_message_sends_message_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .send_message("channel-1", MessageParams::new_text("hello"))
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /channels/channel-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello"
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn list_messages_uses_pager_query() {
        let (base_url, request, server) =
            spawn_capture_server_with_body(
                r#"[{"id":"message-1","content":"hello","channel_id":"channel-1","guild_id":"guild-1","author":{"id":"user-1","username":"user","bot":false},"seq_in_channel":"1","timestamp":"2024-01-01T00:00:00+08:00"}]"#,
            )
            .await;
        let api = test_api(base_url).await;
        let messages = api
            .list_messages("channel-1", &MessagesPager::before("message-0", 3))
            .await
            .unwrap();

        assert_eq!(messages[0].id, "message-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /channels/channel-1/messages?"));
        assert!(request.contains("before=message-0"));
        assert!(request.contains("limit=3"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn get_message_accepts_wrapped_message_response() {
        let (base_url, request, server) =
            spawn_capture_server_with_body(
                r#"{"message":{"id":"message-1","content":"hello","channel_id":"channel-1","guild_id":"guild-1","author":{"id":"user-1","username":"user","bot":false},"seq_in_channel":"1","timestamp":"2024-01-01T00:00:00+08:00"}}"#,
            )
            .await;
        let api = test_api(base_url).await;
        let message = api.get_message("channel-1", "message-1").await.unwrap();

        assert_eq!(message.id, "message-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /channels/channel-1/messages/message-1 HTTP/1.1"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn update_message_sends_patch_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.update_message("channel-1", "message-1", MessageParams::new_text("updated"))
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PATCH /channels/channel-1/messages/message-1 HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "updated"
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn send_setting_guide_mentions_users() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.send_setting_guide(
            "channel-1",
            SettingGuideParams::for_users(["user-1", "user-2"]),
        )
        .await
        .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("POST /channels/channel-1/settingguide HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "<@user-1><@user-2>"
            })
        );
        server.await.unwrap();
    }
}
