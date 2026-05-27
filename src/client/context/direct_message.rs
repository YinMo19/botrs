use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn create_dms(
        &self,
        source_guild_id: &str,
        recipient_id: &str,
    ) -> Result<DirectMessageSession> {
        self.api
            .create_dms(&self.token, source_guild_id, recipient_id)
            .await
    }

    /// Creates a direct message session.

    pub async fn create_direct_message(
        &self,
        dm: &DirectMessageToCreate,
    ) -> Result<DirectMessageSession> {
        self.api.create_direct_message(&self.token, dm).await
    }

    /// Sends a direct message and returns the full message.

    pub async fn post_direct_message(
        &self,
        guild_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.api
            .post_direct_message(&self.token, guild_id, msg)
            .await
    }

    /// Sends a direct message using the legacy positional argument API.
    #[allow(deprecated)]
    #[allow(clippy::too_many_arguments)]
    pub async fn post_dms(
        &self,
        guild_id: &str,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        image: Option<&str>,
        file_image: Option<&[u8]>,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&Keyboard>,
    ) -> Result<MessageResponse> {
        self.api
            .post_dms(
                &self.token,
                guild_id,
                content,
                embed,
                ark,
                message_reference,
                image,
                file_image,
                msg_id,
                event_id,
                markdown,
                keyboard,
            )
            .await
    }

    /// Posts a DM setting guide message.

    pub async fn post_dm_setting_guide(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<MessageResponse> {
        self.api
            .post_dm_setting_guide(&self.token, guild_id, jump_guild_id)
            .await
    }

    /// Posts a DM setting guide message and returns the full message.

    pub async fn post_dm_setting_guide_message(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<Message> {
        self.api
            .post_dm_setting_guide_message(&self.token, guild_id, jump_guild_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn test_context(base_url: String) -> Context {
        let token = Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let mut http = HttpClient::new(30, false).unwrap();
        http.base_url = base_url;
        Context::new(Arc::new(BotApi::with_token(http, token.clone())), token)
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

            let body = r#"{"guild_id":"dm-guild-1","channel_id":"dm-channel-1","create_time":"0"}"#;
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
    async fn context_create_dms_uses_source_guild_then_recipient() {
        let (base_url, request, server) = spawn_capture_server().await;
        let ctx = test_context(base_url).await;
        let session = ctx.create_dms("guild-1", "user-1").await.unwrap();

        assert_eq!(session.guild_id, "dm-guild-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /users/@me/dms HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "source_guild_id": "guild-1",
                "recipient_id": "user-1"
            })
        );
        server.await.unwrap();
    }
}
