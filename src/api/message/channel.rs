use super::legacy::ChannelLikeMessageParts;
use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        Ark, Embed, Keyboard, KeyboardPayload, MarkdownPayload, Message, MessagePagerType,
        MessageParams, MessageToCreate, MessagesPager, Reference,
    },
};
use crate::token::Token;
use base64::Engine;
use reqwest::Method;
use serde::Serialize;
use serde_json::{Value, json};
use tracing::debug;

#[derive(Serialize)]
struct BotpyChannelMessageBody<'a> {
    channel_id: &'a str,
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

#[derive(Serialize)]
struct BotpyKeyboardMessageBody<'a> {
    keyboard: Option<&'a KeyboardPayload>,
    markdown: Option<&'a MarkdownPayload>,
}

#[derive(Serialize)]
struct BotpyPatchGuildMessageBody<'a> {
    channel_id: &'a str,
    patch_msg_id: &'a str,
    msg_id: Option<&'a str>,
    event_id: Option<&'a str>,
    markdown: Option<&'a MarkdownPayload>,
    keyboard: Value,
}

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

    /// Sends a channel message using botpy's locals()-style request body.
    #[allow(clippy::too_many_arguments)]
    pub async fn post_message_botpy(
        &self,
        token: &Token,
        channel_id: &str,
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
        debug!("Sending botpy-style message to channel {}", channel_id);
        let body = BotpyChannelMessageBody {
            channel_id,
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

    /// Sends a botpy-style inline keyboard message body.
    pub async fn post_keyboard_message(
        &self,
        token: &Token,
        channel_id: &str,
        keyboard: Option<&KeyboardPayload>,
        markdown: Option<&MarkdownPayload>,
    ) -> Result<MessageResponse> {
        debug!("Sending keyboard message to channel {}", channel_id);
        let body = BotpyKeyboardMessageBody { keyboard, markdown };
        let path = resource::channel_messages(channel_id);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Edits a guild message using botpy's inline markdown/keyboard body shape.
    pub async fn patch_guild_message(
        &self,
        token: &Token,
        channel_id: &str,
        patch_msg_id: &str,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        debug!(
            "Editing guild message {} in channel {}",
            patch_msg_id, channel_id
        );
        let keyboard = keyboard
            .map(serde_json::to_value)
            .transpose()?
            .unwrap_or_else(|| json!({"content": {}}));
        let body = BotpyPatchGuildMessageBody {
            channel_id,
            patch_msg_id,
            msg_id,
            event_id,
            markdown,
            keyboard,
        };
        let path = resource::channel_message(channel_id, patch_msg_id);
        self.request_message_response_body(token, Method::PATCH, &path, &body)
            .await
    }

    /// Sends a message to a channel (legacy API for backward compatibility).
    #[deprecated(since = "0.1.0", note = "Use post_message_with_params instead")]
    #[allow(clippy::too_many_arguments)]
    pub async fn post_message(
        &self,
        token: &Token,
        channel_id: &str,
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
        let params: MessageParams = ChannelLikeMessageParts::new(
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

        self.post_message_with_params(token, channel_id, params)
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
    async fn post_message_botpy_matches_locals_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_message_botpy(
                api.token_required().unwrap(),
                "channel-1",
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
        assert!(request.starts_with("POST /channels/channel-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "channel_id": "channel-1",
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

    #[tokio::test]
    async fn post_keyboard_message_matches_botpy_null_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_keyboard_message(api.token_required().unwrap(), "channel-1", None, None)
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /channels/channel-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "keyboard": null,
                "markdown": null
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn patch_guild_message_matches_botpy_locals_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .patch_guild_message(
                api.token_required().unwrap(),
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
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "channel_id": "channel-1",
                "patch_msg_id": "message-1",
                "msg_id": null,
                "event_id": null,
                "markdown": null,
                "keyboard": {
                    "content": {}
                }
            })
        );
        server.await.unwrap();
    }
}
