use crate::api::{BotApi, message::legacy::ChannelLikeMessageParts, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        Ark, DirectMessageParams, Embed, Keyboard, MarkdownPayload, Message, MessageToCreate,
        Reference,
    },
};
use crate::token::Token;
use base64::Engine;
use reqwest::Method;
use serde::Serialize;
use tracing::debug;

#[derive(Serialize)]
struct BotpyDirectMessageBody<'a> {
    guild_id: &'a str,
    content: Option<&'a str>,
    embed: Option<&'a Embed>,
    ark: Option<&'a Ark>,
    message_reference: Option<&'a Reference>,
    image: Option<&'a str>,
    file_image: Option<String>,
    msg_id: Option<&'a str>,
    event_id: Option<&'a str>,
    markdown: Option<&'a MarkdownPayload>,
    keyboard: Option<&'a Keyboard>,
}

impl BotApi {
    /// Sends a direct message using DirectMessageParams.
    pub async fn post_dms_with_params(
        &self,
        token: &Token,
        guild_id: &str,
        params: DirectMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending direct message to guild session {}", guild_id);
        let body = MessageToCreate::from(params);
        let path = resource::dms_messages(guild_id);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Sends a direct message using the structured message create payload.
    pub async fn post_direct_message(
        &self,
        token: &Token,
        guild_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        debug!("Sending direct message to guild {}", guild_id);
        let path = resource::dms_messages(guild_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(msg))
            .await
    }

    /// Sends a direct message using botpy's locals()-style request body.
    #[allow(clippy::too_many_arguments)]
    pub async fn post_dms_botpy(
        &self,
        token: &Token,
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
        debug!("Sending botpy-style direct message to {}", guild_id);
        let body = BotpyDirectMessageBody {
            guild_id,
            content,
            embed,
            ark,
            message_reference,
            image,
            file_image: file_image
                .map(|data| base64::engine::general_purpose::STANDARD.encode(data)),
            msg_id,
            event_id,
            markdown,
            keyboard,
        };
        let path = resource::dms_messages(guild_id);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Sends a direct message (legacy API for backward compatibility).
    #[deprecated(since = "0.1.0", note = "Use post_dms_with_params instead")]
    #[allow(clippy::too_many_arguments)]
    pub async fn post_dms(
        &self,
        token: &Token,
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
        let params: DirectMessageParams = ChannelLikeMessageParts::new(
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
        .into();

        self.post_dms_with_params(token, guild_id, params).await
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
        let body = request.split("\r\n\r\n").nth(1).unwrap_or_default();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn botpy_post_dms_body_keeps_guild_id_and_nulls() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_dms_botpy(
                api.token_required().unwrap(),
                "guild-1",
                Some("hello"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /dms/guild-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "guild_id": "guild-1",
                "content": "hello",
                "embed": null,
                "ark": null,
                "message_reference": null,
                "image": null,
                "file_image": null,
                "msg_id": null,
                "event_id": null,
                "markdown": null,
                "keyboard": null
            })
        );
        server.await.unwrap();
    }
}
