use super::{BotApi, resource};
use crate::error::Result;
use crate::models::announce::{Announce, AnnouncesType, RecommendChannel};
use crate::models::message::Media;
use serde::Serialize;
use serde_json::{Value, json};
use tracing::debug;

#[derive(Serialize)]
struct GroupFileBody<'a> {
    group_openid: &'a str,
    file_type: u32,
    url: &'a str,
    srv_send_msg: bool,
}

#[derive(Serialize)]
struct C2cFileBody<'a> {
    openid: &'a str,
    file_type: u32,
    url: &'a str,
    srv_send_msg: bool,
}

#[derive(Serialize)]
struct GuildMessageAnnounceBody<'a> {
    channel_id: &'a str,
    message_id: &'a str,
}

#[derive(Serialize)]
struct GuildRecommendAnnounceBody {
    announces_type: u32,
    recommend_channels: Vec<RecommendChannel>,
}

impl BotApi {
    /// Sends a file message to an open-platform group conversation.
    ///
    /// `file_type` follows the platform numeric values: image, video, audio,
    /// or generic file.
    pub async fn post_group_file(
        &self,
        group_openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<Media> {
        debug!("Uploading group file to {}", group_openid);

        let body = GroupFileBody {
            group_openid,
            file_type,
            url,
            srv_send_msg: srv_send_msg.unwrap_or(false),
        };

        let path = resource::group_file(group_openid);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Sends a file message to a C2C conversation.
    ///
    /// `file_type` follows the platform numeric values: image, video, audio,
    /// or generic file.
    pub async fn post_c2c_file(
        &self,
        openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<Media> {
        debug!("Uploading C2C file to {}", openid);

        let body = C2cFileBody {
            openid,
            file_type,
            url,
            srv_send_msg: srv_send_msg.unwrap_or(false),
        };

        let path = resource::c2c_file(openid);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    // Announcement APIs

    /// Creates a channel announcement from an existing message.
    pub async fn create_channel_announce(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        debug!(
            "Creating channel announcement in channel {} for message {}",
            channel_id, message_id
        );

        let body = json!({
            "message_id": message_id
        });

        let path = resource::channel_announces(channel_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a channel announcement by message ID.
    pub async fn delete_channel_announce(&self, channel_id: &str, message_id: &str) -> Result<()> {
        debug!(
            "Deleting announcement {} in channel {}",
            message_id, channel_id
        );

        let path = resource::channel_announce(channel_id, message_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
    }

    /// Clears all channel announcements without checking a message ID.
    pub async fn clean_channel_announces(&self, channel_id: &str) -> Result<()> {
        debug!("Clearing announcements in channel {}", channel_id);
        let path = resource::channel_announces_all(channel_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
    }

    /// Creates a message-type guild announcement from an existing message.
    pub async fn create_announce(
        &self,
        guild_id: &str,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        debug!(
            "Creating announcement in guild {} for message {}",
            guild_id, message_id
        );

        let body = GuildMessageAnnounceBody {
            channel_id,
            message_id,
        };

        let path = resource::guild_announces(guild_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a message-type guild announcement.
    pub async fn create_guild_announce(
        &self,
        guild_id: &str,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        self.create_announce(guild_id, channel_id, message_id).await
    }

    /// Creates a guild announcement that recommends channels.
    pub async fn create_recommend_announce(
        &self,
        guild_id: &str,
        announces_type: AnnouncesType,
        recommend_channels: Vec<RecommendChannel>,
    ) -> Result<Announce> {
        debug!("Creating recommend announcement in guild {}", guild_id);

        let body = GuildRecommendAnnounceBody {
            announces_type: u8::from(announces_type) as u32,
            recommend_channels,
        };

        let path = resource::guild_announces(guild_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a recommended channel guild announcement.
    pub async fn create_guild_recommend_announce(
        &self,
        guild_id: &str,
        announces_type: AnnouncesType,
        recommend_channels: Vec<RecommendChannel>,
    ) -> Result<Announce> {
        self.create_recommend_announce(guild_id, announces_type, recommend_channels)
            .await
    }

    /// Deletes a guild announcement by message ID and returns the raw response.
    ///
    /// Passing `None::<&str>` deletes all guild announcements.
    pub async fn delete_announce<'a>(
        &self,
        guild_id: &str,
        message_id: impl Into<Option<&'a str>>,
    ) -> Result<Value> {
        let message_id = message_id.into().unwrap_or("all");
        debug!("Deleting announcement {} in guild {}", message_id, guild_id);

        let path = resource::guild_announce(guild_id, message_id);
        let response = self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(response)
    }

    /// Deletes a guild announcement.
    pub async fn delete_guild_announce(&self, guild_id: &str, message_id: &str) -> Result<()> {
        self.delete_announce(guild_id, message_id).await?;
        Ok(())
    }

    /// Clears all guild announcements without checking a message ID.
    pub async fn clean_guild_announces(&self, guild_id: &str) -> Result<()> {
        debug!("Clearing announcements in guild {}", guild_id);
        let path = resource::guild_announces_all(guild_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{C2cFileBody, GroupFileBody, GuildMessageAnnounceBody, GuildRecommendAnnounceBody};
    use crate::models::announce::{AnnouncesType, RecommendChannel};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn test_api(base_url: String) -> crate::api::BotApi {
        let token = crate::Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let mut http = crate::http::HttpClient::new(30, false).unwrap();
        http.base_url = base_url;
        crate::api::BotApi::new(http, token)
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

            let body = r#"{"file_uuid":"file-1","file_info":"info-1","ttl":60}"#;
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

    #[test]
    fn open_file_bodies_use_platform_shape() {
        let group = serde_json::to_value(GroupFileBody {
            group_openid: "group-openid-1",
            file_type: 1,
            url: "https://example.com/a.png",
            srv_send_msg: false,
        })
        .unwrap();
        assert_eq!(
            group,
            serde_json::json!({
                "group_openid": "group-openid-1",
                "file_type": 1,
                "url": "https://example.com/a.png",
                "srv_send_msg": false
            })
        );

        let c2c = serde_json::to_value(C2cFileBody {
            openid: "openid-1",
            file_type: 1,
            url: "https://example.com/a.png",
            srv_send_msg: false,
        })
        .unwrap();
        assert_eq!(
            c2c,
            serde_json::json!({
                "openid": "openid-1",
                "file_type": 1,
                "url": "https://example.com/a.png",
                "srv_send_msg": false
            })
        );
    }

    #[tokio::test]
    async fn post_group_file_uses_platform_request() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_group_file("group-openid-1", 1, "https://example.com/a.png", None)
            .await
            .unwrap();

        assert_eq!(response.file_uuid.as_deref(), Some("file-1"));
        assert_eq!(response.file_info.as_deref(), Some("info-1"));
        assert_eq!(response.ttl, Some(60));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /v2/groups/group-openid-1/files HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "group_openid": "group-openid-1",
                "file_type": 1,
                "url": "https://example.com/a.png",
                "srv_send_msg": false
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn post_c2c_file_uses_platform_request() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let response = api
            .post_c2c_file("openid-1", 1, "https://example.com/a.png", None)
            .await
            .unwrap();

        assert_eq!(response.file_uuid.as_deref(), Some("file-1"));
        assert_eq!(response.file_info.as_deref(), Some("info-1"));
        assert_eq!(response.ttl, Some(60));
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /v2/users/openid-1/files HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "openid": "openid-1",
                "file_type": 1,
                "url": "https://example.com/a.png",
                "srv_send_msg": false
            })
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn delete_announce_defaults_to_all() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        let response = api.delete_announce("guild-1", None::<&str>).await.unwrap();

        assert_eq!(response["file_uuid"], "file-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("DELETE /guilds/guild-1/announces/all HTTP/1.1"));
        server.await.unwrap();
    }

    #[test]
    fn high_level_guild_announce_body_uses_platform_shape() {
        let message = serde_json::to_value(GuildMessageAnnounceBody {
            channel_id: "channel-1",
            message_id: "message-1",
        })
        .unwrap();
        assert_eq!(
            message,
            serde_json::json!({
                "channel_id": "channel-1",
                "message_id": "message-1"
            })
        );

        let recommend = serde_json::to_value(GuildRecommendAnnounceBody {
            announces_type: u8::from(AnnouncesType::Welcome) as u32,
            recommend_channels: vec![RecommendChannel {
                channel_id: "channel-2".to_string(),
                introduce: "intro".to_string(),
            }],
        })
        .unwrap();
        assert_eq!(recommend["announces_type"], 1);
        assert_eq!(
            recommend["recommend_channels"][0]["channel_id"],
            "channel-2"
        );
        assert!(recommend.get("channel_id").is_none());
        assert!(recommend.get("message_id").is_none());
    }
}
