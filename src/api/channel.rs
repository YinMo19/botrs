use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    channel::{Channel, ChannelSubType, ChannelType, ChannelValueObject, PrivateType},
    guild::Member,
};
use crate::token::Token;
use serde::Serialize;
use serde_json::Value;
use tracing::debug;

#[derive(Debug, Serialize)]
struct BotpyCreateChannel {
    name: String,
    #[serde(rename = "type")]
    channel_type: ChannelType,
    #[serde(rename = "subtype")]
    sub_type: ChannelSubType,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speak_permission: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<String>,
}

impl BotpyCreateChannel {
    #[allow(clippy::too_many_arguments)]
    fn new(
        name: &str,
        channel_type: ChannelType,
        sub_type: ChannelSubType,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        private_user_ids: Option<Vec<String>>,
        speak_permission: Option<u32>,
        application_id: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_string(),
            channel_type,
            sub_type,
            position: position.filter(|value| *value != 0),
            parent_id: parent_id
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
            private_type: private_type.filter(|value| *value != 0),
            private_user_ids: private_user_ids.filter(|value| !value.is_empty()),
            speak_permission: speak_permission.filter(|value| *value != 0),
            application_id: application_id
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
        }
    }
}

#[derive(Debug, Serialize)]
struct BotpyUpdateChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speak_permission: Option<u32>,
}

impl BotpyUpdateChannel {
    fn new(
        name: Option<&str>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        speak_permission: Option<u32>,
    ) -> Self {
        Self {
            name: name.map(ToOwned::to_owned),
            position,
            parent_id: parent_id.map(ToOwned::to_owned),
            private_type,
            speak_permission,
        }
    }
}

impl BotApi {
    // Channel APIs

    /// Fetches one channel by ID.
    pub async fn get_channel(&self, token: &Token, channel_id: &str) -> Result<Channel> {
        debug!("Getting channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists channels in a guild.
    pub async fn get_channels(&self, token: &Token, guild_id: &str) -> Result<Vec<Channel>> {
        debug!("Getting channels for guild {}", guild_id);
        let path = resource::guild_channels(guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a guild channel from a structured channel body.
    pub async fn post_channel(
        &self,
        token: &Token,
        guild_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        debug!("Creating channel in guild {}", guild_id);
        let path = resource::guild_channels(guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(value))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a guild channel from inline fields.
    pub async fn create_channel(
        &self,
        token: &Token,
        guild_id: &str,
        name: &str,
        channel_type: ChannelType,
        sub_type: ChannelSubType,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        private_user_ids: Option<Vec<String>>,
        speak_permission: Option<u32>,
        application_id: Option<&str>,
    ) -> Result<Channel> {
        debug!("Creating channel in guild {}", guild_id);
        let body = BotpyCreateChannel::new(
            name,
            channel_type,
            sub_type,
            position,
            parent_id,
            private_type,
            private_user_ids,
            speak_permission,
            application_id,
        );
        let path = resource::guild_channels(guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
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
        token: &Token,
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
        self.post_channel(token, guild_id, &value).await
    }

    /// Updates a channel from inline fields.
    pub async fn update_channel(
        &self,
        token: &Token,
        channel_id: &str,
        name: Option<&str>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        speak_permission: Option<u32>,
    ) -> Result<Channel> {
        debug!("Updating channel {}", channel_id);
        let body =
            BotpyUpdateChannel::new(name, position, parent_id, private_type, speak_permission);
        let path = resource::channel(channel_id);
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Updates a channel from a structured channel body.
    pub async fn patch_channel(
        &self,
        token: &Token,
        channel_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        debug!("Updating channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(value))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a channel.
    ///
    /// The platform may return the deleted channel object or an empty success
    /// response. Empty responses are represented as `None`.
    pub async fn delete_channel(&self, token: &Token, channel_id: &str) -> Result<Option<Channel>> {
        debug!("Deleting channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self.http.delete(token, &path, None::<&()>).await?;
        if response == Value::Null {
            Ok(None)
        } else {
            Self::decode_json(response).map(Some)
        }
    }

    /// Lists members currently present in a voice channel.
    pub async fn list_voice_channel_members(
        &self,
        token: &Token,
        channel_id: &str,
    ) -> Result<Vec<Member>> {
        debug!("Listing voice channel members for channel {}", channel_id);
        let path = resource::voice_channel_members(channel_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
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
        BotApi::with_token(http, token)
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
    async fn inline_create_channel_matches_botpy_subtype_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let channel = api
            .create_channel(
                api.token_required().unwrap(),
                "guild-1",
                "channel_test",
                ChannelType::Text,
                ChannelSubType::Chat,
                Some(0),
                Some(""),
                Some(0),
                Some(Vec::new()),
                Some(0),
                Some(""),
            )
            .await
            .unwrap();

        assert_eq!(channel.id, "channel-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /guilds/guild-1/channels HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n{\"name\":\"channel_test\",\"type\":0,\"subtype\":0}"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn inline_update_channel_matches_botpy_kwargs_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let channel = api
            .update_channel(
                api.token_required().unwrap(),
                "channel-1",
                Some(""),
                Some(0),
                Some(""),
                Some(0),
                Some(0),
            )
            .await
            .unwrap();

        assert_eq!(channel.id, "channel-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("PATCH /channels/channel-1 HTTP/1.1"));
        assert!(request.ends_with(
            "\r\n\r\n{\"name\":\"\",\"position\":0,\"parent_id\":\"\",\"private_type\":0,\"speak_permission\":0}"
        ));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn delete_channel_keeps_deleted_channel_when_returned() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let channel = api
            .delete_channel(api.token_required().unwrap(), "channel-1")
            .await
            .unwrap();

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
        let channel = api
            .delete_channel(api.token_required().unwrap(), "channel-1")
            .await
            .unwrap();

        assert!(channel.is_none());
        let request = request.await.unwrap();
        assert!(request.starts_with("DELETE /channels/channel-1 HTTP/1.1"));
        server.await.unwrap();
    }
}
