use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        Keyboard, MarkdownPayload, Message, MessagePagerType, MessageParams, MessageToCreate,
        MessagesPager,
    },
};
use crate::token::Token;
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Gets a specific message.
    pub async fn get_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Message> {
        debug!("Getting message {} in channel {}", message_id, channel_id);
        let path = resource::channel_message(channel_id, message_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::parse_message_response(response)
    }

    /// Gets channel messages using paginated requests.
    pub async fn get_messages(
        &self,
        token: &Token,
        channel_id: &str,
        pager: &MessagesPager,
    ) -> Result<Vec<Message>> {
        debug!("Getting messages in channel {}", channel_id);
        let params = pager.query_params();
        let path = resource::channel_messages(channel_id);
        self.request_json(
            token,
            Method::GET,
            &path,
            if params.is_empty() {
                None
            } else {
                Some(&params)
            },
            None::<&()>,
        )
        .await
    }

    /// Gets channel messages using simple pagination parameters.
    pub async fn get_messages_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        pager_type: Option<MessagePagerType>,
        message_id: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Message>> {
        let pager = MessagesPager::new(pager_type, message_id, limit);
        self.get_messages(token, channel_id, &pager).await
    }

    /// Sends a channel message using the structured message create payload.
    pub async fn post_message_to_create(
        &self,
        token: &Token,
        channel_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        debug!("Sending message to channel {}", channel_id);
        let path = resource::channel_messages(channel_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(msg))
            .await
    }

    /// Pascal-case alias for sending a channel message.
    pub async fn post_message_api(
        &self,
        token: &Token,
        channel_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_message_to_create(token, channel_id, msg).await
    }

    /// Edits a channel message using the structured message create payload.
    pub async fn patch_message_to_create(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        debug!("Editing message {} in channel {}", message_id, channel_id);
        let path = resource::channel_message(channel_id, message_id);
        self.request_json(token, Method::PATCH, &path, None::<&()>, Some(msg))
            .await
    }

    /// Pascal-case alias for editing a channel message.
    pub async fn patch_message_api(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.patch_message_to_create(token, channel_id, message_id, msg)
            .await
    }

    /// Sends a message to a channel using MessageParams.
    pub async fn post_message_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending message to channel {}", channel_id);
        let body = MessageToCreate::from(params);
        let path = resource::channel_messages(channel_id);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Edits a channel message using MessageParams.
    pub async fn patch_message_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        debug!("Editing message {} in channel {}", message_id, channel_id);
        let body = MessageToCreate::from(params);
        let path = resource::channel_message(channel_id, message_id);
        self.request_message_response_body(token, Method::PATCH, &path, &body)
            .await
    }

    /// Alias for editing a channel message.
    pub async fn patch_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        self.patch_message_with_params(token, channel_id, message_id, params)
            .await
    }

    /// Sends an inline keyboard message body.
    pub async fn post_keyboard_message(
        &self,
        token: &Token,
        channel_id: &str,
        keyboard: Option<&Keyboard>,
        markdown: Option<&MarkdownPayload>,
    ) -> Result<MessageResponse> {
        debug!("Sending keyboard message to channel {}", channel_id);
        let body = MessageToCreate {
            keyboard: keyboard.cloned(),
            markdown: markdown.cloned(),
            ..Default::default()
        };
        let path = resource::channel_messages(channel_id);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Edits a guild message with inline markdown or keyboard content.
    pub async fn patch_guild_message(
        &self,
        token: &Token,
        channel_id: &str,
        patch_msg_id: &str,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&Keyboard>,
    ) -> Result<MessageResponse> {
        debug!(
            "Editing guild message {} in channel {}",
            patch_msg_id, channel_id
        );
        let body = MessageToCreate {
            msg_id: msg_id.map(ToOwned::to_owned),
            event_id: event_id.map(ToOwned::to_owned),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
            ..Default::default()
        };
        let path = resource::channel_message(channel_id, patch_msg_id);
        self.request_message_response_body(token, Method::PATCH, &path, &body)
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
        let mut http = crate::http::HttpClient::new(30, false).unwrap();
        http.base_url = base_url;
        BotApi::with_token(http, token)
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

            let body = r#"{"id":"message-1","timestamp":"2026-01-01T00:00:00+08:00"}"#;
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
    async fn post_message_with_params_sends_botgo_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_message_with_params(
                api.token().unwrap(),
                "channel-1",
                MessageParams::new_text("hello"),
            )
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
    async fn post_message_with_params_file_image_uses_json_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_message_with_params(
                api.token().unwrap(),
                "channel-1",
                MessageParams::new_text("hello")
                    .with_file_image(b"image-bytes")
                    .with_reply("message-1"),
            )
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /channels/channel-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello",
                "msg_id": "message-1",
                "file_image": "aW1hZ2UtYnl0ZXM="
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn post_keyboard_message_omits_empty_fields() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_keyboard_message(api.token().unwrap(), "channel-1", None, None)
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /channels/channel-1/messages HTTP/1.1"));
        assert_eq!(request_body(&request), serde_json::json!({}));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn patch_guild_message_omits_empty_fields() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .patch_guild_message(
                api.token().unwrap(),
                "channel-1",
                "message-1",
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("PATCH /channels/channel-1/messages/message-1 HTTP/1.1"));
        assert_eq!(request_body(&request), serde_json::json!({}));
        server.await.unwrap();
    }
}
