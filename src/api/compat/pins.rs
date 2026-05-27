use crate::api::BotApi;
use crate::error::Result;
use crate::models::api::PinsMessage;

impl BotApi {
    /// Pins add API.
    #[allow(non_snake_case)]
    pub async fn AddPins(&self, channel_id: &str, message_id: &str) -> Result<PinsMessage> {
        self.add_pins_botgo(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Pins delete API.
    #[allow(non_snake_case)]
    pub async fn DeletePins(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete_pin(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Pins clean API.
    #[allow(non_snake_case)]
    pub async fn CleanPins(&self, channel_id: &str) -> Result<()> {
        self.clean_pins(self.token_required()?, channel_id).await
    }

    /// Pins list API.
    #[allow(non_snake_case)]
    pub async fn GetPins(&self, channel_id: &str) -> Result<PinsMessage> {
        self.get_pins(self.token_required()?, channel_id).await
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

            let body =
                r#"{"guild_id":"guild-1","channel_id":"channel-1","message_ids":["message-1"]}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    #[tokio::test]
    async fn high_level_put_pin_uses_empty_json_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let pins = api
            .put_pin(api.token_required().unwrap(), "channel-1", "message-1")
            .await
            .unwrap();
        assert_eq!(pins.message_ids, vec!["message-1"]);

        let request = request.await.unwrap();
        let headers = request.to_ascii_lowercase();
        assert!(request.starts_with("PUT /channels/channel-1/pins/message-1 HTTP/1.1"));
        assert!(headers.contains("content-type: application/json"));
        assert!(request.ends_with("\r\n\r\n{}"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn compat_add_pins_matches_botgo_empty_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let pins = test_api(base_url)
            .await
            .AddPins("channel-1", "message-1")
            .await
            .unwrap();
        assert_eq!(pins.message_ids, vec!["message-1"]);

        let request = request.await.unwrap();
        let headers = request.to_ascii_lowercase();
        assert!(request.starts_with("PUT /channels/channel-1/pins/message-1 HTTP/1.1"));
        assert!(!headers.contains("content-type: application/json"));
        assert!(request.ends_with("\r\n\r\n"));
        server.await.unwrap();
    }
}
