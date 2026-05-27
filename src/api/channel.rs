use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    channel::{Channel, ChannelValueObject, PrivateType},
    guild::Member,
};
use serde_json::Value;
use tracing::debug;

impl BotApi {
    // Channel APIs

    /// Fetches one channel by ID.
    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel> {
        debug!("Getting channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists channels in a guild.
    pub async fn get_channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        debug!("Getting channels for guild {}", guild_id);
        let path = resource::guild_channels(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a guild channel.
    pub async fn create_channel(
        &self,
        guild_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        debug!("Creating channel in guild {}", guild_id);
        let path = resource::guild_channels(guild_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(value))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a private channel.
    ///
    /// If `user_ids` is empty, the channel is visible to admins and members.
    /// If `user_ids` is not empty, the channel is created as admin-only and the
    /// members are added through `private_user_ids`.
    pub async fn create_private_channel(
        &self,
        guild_id: &str,
        value: &ChannelValueObject,
        user_ids: Vec<String>,
    ) -> Result<Channel> {
        let mut value = value.clone();
        value.private_type = Some(PrivateType::AdminAndMember);
        if !user_ids.is_empty() {
            value.private_user_ids = Some(user_ids);
            value.private_type = Some(PrivateType::OnlyAdmin);
        }
        self.create_channel(guild_id, &value).await
    }

    /// Updates a channel.
    pub async fn update_channel(
        &self,
        channel_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        debug!("Updating channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self
            .http
            .patch(self.token(), &path, None::<&()>, Some(value))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a channel.
    ///
    /// The platform may return the deleted channel object or an empty success
    /// response. Empty responses are represented as `None`.
    pub async fn delete_channel(&self, channel_id: &str) -> Result<Option<Channel>> {
        debug!("Deleting channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self.http.delete(self.token(), &path, None::<&()>).await?;
        if response == Value::Null {
            Ok(None)
        } else {
            Self::decode_json(response).map(Some)
        }
    }

    /// Lists members currently present in a voice channel.
    pub async fn list_voice_channel_members(&self, channel_id: &str) -> Result<Vec<Member>> {
        debug!("Listing voice channel members for channel {}", channel_id);
        let path = resource::voice_channel_members(channel_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
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
        spawn_capture_server_with_response(
            "200 OK",
            Some(
                r#"{"id":"channel-1","guild_id":"guild-1","name":"channel_test","type":0,"sub_type":0}"#,
            ),
        )
        .await
    }

    async fn spawn_capture_server_with_response(
        status: &'static str,
        body: Option<&'static str>,
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

            let body = body.unwrap_or_default();
            let response = if body.is_empty() {
                format!("HTTP/1.1 {status}\r\ncontent-length: 0\r\nconnection: close\r\n\r\n")
            } else {
                format!(
                    "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    body.len(),
                    body
                )
            };
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    #[tokio::test]
    async fn create_channel_uses_structured_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let value = ChannelValueObject::new(
            "channel_test",
            crate::models::channel::ChannelType::Text,
            crate::models::channel::ChannelSubType::Chat,
        );
        let channel = api.create_channel("guild-1", &value).await.unwrap();

        assert_eq!(channel.id, "channel-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /guilds/guild-1/channels HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n{\"name\":\"channel_test\"}"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn update_channel_uses_structured_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let value = ChannelValueObject {
            name: Some("updated".to_string()),
            channel_type: Some(crate::models::channel::ChannelType::Voice),
            sub_type: Some(crate::models::channel::ChannelSubType::Notice),
            position: Some(1),
            ..Default::default()
        };
        let channel = api.update_channel("channel-1", &value).await.unwrap();

        assert_eq!(channel.id, "channel-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("PATCH /channels/channel-1 HTTP/1.1"));
        assert!(
            request.ends_with(
                "\r\n\r\n{\"name\":\"updated\",\"type\":2,\"position\":1,\"sub_type\":1}"
            )
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn delete_channel_keeps_deleted_channel_when_returned() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let channel = api.delete_channel("channel-1").await.unwrap();

        assert_eq!(
            channel.as_ref().map(|channel| channel.id.as_str()),
            Some("channel-1")
        );
        let request = request.await.unwrap();
        assert!(request.starts_with("DELETE /channels/channel-1 HTTP/1.1"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn delete_channel_accepts_empty_success_response() {
        let (base_url, request, server) =
            spawn_capture_server_with_response("204 No Content", None).await;
        let api = test_api(base_url).await;
        let channel = api.delete_channel("channel-1").await.unwrap();

        assert!(channel.is_none());
        let request = request.await.unwrap();
        assert!(request.starts_with("DELETE /channels/channel-1 HTTP/1.1"));
        server.await.unwrap();
    }
}
