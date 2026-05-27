use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{
        ApiMessage, Ark, C2CMessageParams, Embed, GroupMessageParams, KeyboardPayload,
        MarkdownPayload, Media, Message, MessageToCreate, Reference, RichMediaMessage, SendType,
    },
};
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use tracing::debug;

#[derive(Clone, Copy)]
enum OpenMessageTarget<'a> {
    Group(&'a str),
    C2c(&'a str),
}

impl<'a> OpenMessageTarget<'a> {
    const fn name(self) -> &'static str {
        match self {
            Self::Group(_) => "group",
            Self::C2c(_) => "C2C",
        }
    }

    const fn id(self) -> &'a str {
        match self {
            Self::Group(id) | Self::C2c(id) => id,
        }
    }

    fn send_path(self, send_type: SendType) -> String {
        match self {
            Self::Group(id) => resource::group_send(id, send_type),
            Self::C2c(id) => resource::c2c_send(id, send_type),
        }
    }
}

impl BotApi {
    /// Sends a group message using GroupMessageParams.
    pub async fn post_group_message_with_params(
        &self,
        token: &Token,
        group_openid: &str,
        params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending group message to {}", group_openid);
        let body = MessageToCreate::from(params);
        let path = resource::group_messages(group_openid);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Sends a group message using the structured API message envelope.
    pub async fn post_group_api_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &ApiMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(
            token,
            OpenMessageTarget::Group(group_openid),
            msg.send_type(),
            msg,
        )
        .await
    }

    /// Sends a group message create payload and returns the full message.
    pub async fn post_group_message_to_create(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_open_api_payload(
            token,
            OpenMessageTarget::Group(group_openid),
            msg.send_type(),
            msg,
        )
        .await
    }

    /// Uploads or directly sends group rich media.
    pub async fn post_group_rich_media_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg: &RichMediaMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(
            token,
            OpenMessageTarget::Group(group_openid),
            msg.send_type(),
            msg,
        )
        .await
    }

    /// Sends a group message (legacy API for backward compatibility).
    #[deprecated(since = "0.1.0", note = "Use post_group_message_with_params instead")]
    #[allow(clippy::too_many_arguments)]
    pub async fn post_group_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg_type: Option<u32>,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        media: Option<&Media>,
        msg_id: Option<&str>,
        msg_seq: Option<u32>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        let params = GroupMessageParams {
            msg_type: msg_type.unwrap_or(0),
            content: content.map(ToOwned::to_owned),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            media: media.cloned(),
            msg_id: msg_id.map(ToOwned::to_owned),
            msg_seq,
            event_id: event_id.map(ToOwned::to_owned),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
            ..Default::default()
        };
        self.post_group_message_with_params(token, group_openid, params)
            .await
    }

    /// Sends a C2C (client-to-client) message using C2CMessageParams.
    pub async fn post_c2c_message_with_params(
        &self,
        token: &Token,
        openid: &str,
        params: C2CMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending C2C message to {}", openid);
        let body = MessageToCreate::from(params);
        let path = resource::c2c_messages(openid);
        self.request_message_response_body(token, Method::POST, &path, &body)
            .await
    }

    /// Sends a C2C message using the structured API message envelope.
    pub async fn post_c2c_api_message(
        &self,
        token: &Token,
        openid: &str,
        msg: &ApiMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(token, OpenMessageTarget::C2c(openid), msg.send_type(), msg)
            .await
    }

    /// Sends a C2C message create payload and returns the full message.
    pub async fn post_c2c_message_to_create(
        &self,
        token: &Token,
        openid: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.post_open_api_payload(token, OpenMessageTarget::C2c(openid), msg.send_type(), msg)
            .await
    }

    /// Uploads or directly sends C2C rich media.
    pub async fn post_c2c_rich_media_message(
        &self,
        token: &Token,
        openid: &str,
        msg: &RichMediaMessage,
    ) -> Result<Message> {
        self.post_open_api_payload(token, OpenMessageTarget::C2c(openid), msg.send_type(), msg)
            .await
    }

    /// Sends a C2C (client-to-client) message (legacy API for backward compatibility).
    #[deprecated(since = "0.1.0", note = "Use post_c2c_message_with_params instead")]
    #[allow(clippy::too_many_arguments)]
    pub async fn post_c2c_message(
        &self,
        token: &Token,
        openid: &str,
        msg_type: Option<u32>,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        media: Option<&Media>,
        msg_id: Option<&str>,
        msg_seq: Option<u32>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        let params = C2CMessageParams {
            msg_type: msg_type.unwrap_or(0),
            content: content.map(ToOwned::to_owned),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            media: media.cloned(),
            msg_id: msg_id.map(ToOwned::to_owned),
            msg_seq,
            event_id: event_id.map(ToOwned::to_owned),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
            ..Default::default()
        };
        self.post_c2c_message_with_params(token, openid, params)
            .await
    }

    async fn post_open_api_payload<T>(
        &self,
        token: &Token,
        target: OpenMessageTarget<'_>,
        send_type: SendType,
        msg: &T,
    ) -> Result<Message>
    where
        T: Serialize + ?Sized,
    {
        debug!("Sending {} message to {}", target.name(), target.id());
        self.request_json(
            token,
            Method::POST,
            &target.send_path(send_type),
            None::<&()>,
            Some(msg),
        )
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
    async fn group_message_params_send_botgo_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_group_message_with_params(
                api.token_required().unwrap(),
                "group-openid-1",
                GroupMessageParams::new_text("hello"),
            )
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /v2/groups/group-openid-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello"
            })
        );
        server.await.unwrap();
    }

    #[allow(deprecated)]
    #[tokio::test]
    async fn legacy_group_message_matches_botgo_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_group_message(
                api.token_required().unwrap(),
                "group-openid-1",
                None,
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
        assert!(request.starts_with("POST /v2/groups/group-openid-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello"
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn c2c_message_params_send_botgo_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_c2c_message_with_params(
                api.token_required().unwrap(),
                "openid-1",
                C2CMessageParams::new_text("hello"),
            )
            .await
            .unwrap();

        assert_eq!(response.id.as_deref(), Some("message-1"));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /v2/users/openid-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello"
            })
        );
        server.await.unwrap();
    }

    #[allow(deprecated)]
    #[tokio::test]
    async fn legacy_c2c_message_matches_botgo_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_c2c_message(
                api.token_required().unwrap(),
                "openid-1",
                None,
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
        assert!(request.starts_with("POST /v2/users/openid-1/messages HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "content": "hello"
            })
        );
        server.await.unwrap();
    }
}
