use crate::api_impl::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{DirectMessageParams, Message, MessageToCreate, SettingGuideParams},
};
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Sends a direct message using DirectMessageParams.
    pub async fn send_direct_message(
        &self,
        guild_id: &str,
        params: DirectMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending direct message to guild session {}", guild_id);
        let body = MessageToCreate::from(params);
        let path = resource::dms_messages(guild_id);
        self.request_message_response_body(Method::POST, &path, &body)
            .await
    }

    /// Recalls a direct message by its DM guild/session ID.
    pub async fn recall_direct_message(
        &self,
        guild_id: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!(
            "Recalling direct message {} in DM session {}",
            message_id, guild_id
        );
        let params = hidetip.and_then(Self::hide_tip_query);
        let path = resource::dms_message(guild_id, message_id);
        self.http
            .delete(self.token(), &path, params.as_ref())
            .await?;
        Ok(())
    }

    /// Sends a direct-message setting guide.
    pub async fn send_direct_setting_guide(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<Message> {
        debug!("Sending direct setting guide in DM session {}", guild_id);
        let params = SettingGuideParams::for_guild(jump_guild_id);
        let path = resource::dms_setting_guide(guild_id);
        self.request_json(Method::POST, &path, None::<&()>, Some(&params))
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
    async fn send_direct_message_sends_message_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .send_direct_message("guild-1", DirectMessageParams::new_text("hello"))
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /dms/guild-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello"
            })
        );
        server.await.unwrap();
    }
}
