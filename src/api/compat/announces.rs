use crate::api::BotApi;
use crate::api::resource;
use crate::error::Result;
use crate::models::announce::{Announce, ChannelAnnouncesToCreate, GuildAnnouncesToCreate};
use reqwest::Method;

impl BotApi {
    /// Channel announce creation API.
    #[allow(non_snake_case)]
    pub async fn CreateChannelAnnounces(
        &self,
        channel_id: &str,
        announce: &ChannelAnnouncesToCreate,
    ) -> Result<Announce> {
        self.request_json(
            self.token_required()?,
            Method::POST,
            &resource::channel_announces(channel_id),
            None::<&()>,
            Some(announce),
        )
        .await
    }

    /// Channel announce delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteChannelAnnounces(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete_channel_announce(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Channel announces clean API.
    #[allow(non_snake_case)]
    pub async fn CleanChannelAnnounces(&self, channel_id: &str) -> Result<()> {
        self.clean_channel_announces(self.token_required()?, channel_id)
            .await
    }

    /// Guild announce creation API.
    #[allow(non_snake_case)]
    pub async fn CreateGuildAnnounces(
        &self,
        guild_id: &str,
        announce: &GuildAnnouncesToCreate,
    ) -> Result<Announce> {
        self.request_json(
            self.token_required()?,
            Method::POST,
            &resource::guild_announces(guild_id),
            None::<&()>,
            Some(announce),
        )
        .await
    }

    /// Guild announce delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteGuildAnnounces(&self, guild_id: &str, message_id: &str) -> Result<()> {
        self.delete_guild_announce(self.token_required()?, guild_id, message_id)
            .await
    }

    /// Guild announces clean API.
    #[allow(non_snake_case)]
    pub async fn CleanGuildAnnounces(&self, guild_id: &str) -> Result<()> {
        self.clean_guild_announces(self.token_required()?, guild_id)
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

            let body = r#"{"channel_id":"channel-1","message_id":"message-1"}"#;
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
    async fn create_channel_announces_sends_botgo_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        let announce = api
            .CreateChannelAnnounces(
                "channel-1",
                &ChannelAnnouncesToCreate {
                    message_id: "message-1".to_string(),
                },
            )
            .await
            .unwrap();

        assert_eq!(announce.message_id, "message-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /channels/channel-1/announces HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({"message_id": "message-1"})
        );
        server.await.unwrap();
    }
}
