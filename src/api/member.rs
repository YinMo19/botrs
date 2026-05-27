use super::{BotApi, resource};
use crate::error::Result;
use crate::models::guild::{MemberAddRoleBody, UpdateGuildMute, UpdateGuildMuteResponse};
use tracing::debug;

impl BotApi {
    /// Adds a role to a guild member, optionally scoped to a channel.
    pub async fn create_guild_role_member(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Adding user {} to role {} in guild {}",
            user_id, role_id, guild_id
        );

        let body = channel_id
            .map(MemberAddRoleBody::with_channel_id)
            .unwrap_or_default();
        let path = resource::guild_member_role(guild_id, user_id, role_id);
        self.http
            .put(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Adds a role to a guild member using a structured body.
    pub async fn member_add_role(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        body: &MemberAddRoleBody,
    ) -> Result<()> {
        let path = resource::guild_member_role(guild_id, user_id, role_id);
        self.http
            .put(self.token(), &path, None::<&()>, Some(body))
            .await?;
        Ok(())
    }

    /// Removes a role from a guild member, optionally scoped to a channel.
    pub async fn delete_guild_role_member(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Removing user {} from role {} in guild {}",
            user_id, role_id, guild_id
        );

        let body = channel_id
            .map(MemberAddRoleBody::with_channel_id)
            .unwrap_or_default();
        let path = resource::guild_member_role(guild_id, user_id, role_id);
        self.http
            .delete_with_body(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Removes a role from a guild member using a structured body.
    pub async fn member_delete_role(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        body: &MemberAddRoleBody,
    ) -> Result<()> {
        let path = resource::guild_member_role(guild_id, user_id, role_id);
        self.http
            .delete_with_body(self.token(), &path, None::<&()>, Some(body))
            .await?;
        Ok(())
    }

    /// Mutes one guild member.
    ///
    /// The platform accepts either an end timestamp or a duration in seconds.
    pub async fn mute_member(
        &self,
        guild_id: &str,
        user_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting member {} in guild {}", user_id, guild_id);

        let body = UpdateGuildMute::new(mute_end_timestamp, mute_seconds);

        let path = resource::guild_member_mute(guild_id, user_id);
        self.http
            .patch(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Mutes several guild members with inline parameters.
    pub async fn mute_multi_member(
        &self,
        guild_id: &str,
        user_ids: Vec<String>,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<UpdateGuildMuteResponse> {
        if user_ids.is_empty() {
            return Err(crate::error::BotError::invalid_data("no user id param"));
        }

        let body = UpdateGuildMute::new_multi(user_ids, mute_end_timestamp, mute_seconds);
        debug!("Muting multiple members in guild {}", guild_id);
        let path = resource::guild_mute(guild_id);
        let response = self
            .http
            .patch(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Cancels mute for several guild members.
    pub async fn cancel_mute_multi_member(
        &self,
        guild_id: &str,
        user_ids: Vec<String>,
    ) -> Result<UpdateGuildMuteResponse> {
        if user_ids.is_empty() {
            return Err(crate::error::BotError::invalid_data("no user id param"));
        }

        let body = UpdateGuildMute::cancel_multi(user_ids);
        self.multi_member_mute(guild_id, &body).await
    }

    /// Mutes several guild members using a structured request body.
    pub async fn multi_member_mute(
        &self,
        guild_id: &str,
        mute: &UpdateGuildMute,
    ) -> Result<UpdateGuildMuteResponse> {
        if mute.user_ids.is_empty() {
            return Err(crate::error::BotError::invalid_data("no user id param"));
        }

        debug!("Muting multiple members in guild {}", guild_id);
        let path = resource::guild_mute(guild_id);
        let response = self
            .http
            .patch(self.token(), &path, None::<&()>, Some(mute))
            .await?;
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

            stream
                .write_all(
                    b"HTTP/1.1 204 No Content\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
                )
                .await
                .unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    #[tokio::test]
    async fn inline_add_role_member_matches_botgo_empty_channel() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.create_guild_role_member("guild-1", "role-1", "user-1", None)
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PUT /guilds/guild-1/members/user-1/roles/role-1 HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n{\"channel\":null}"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn inline_delete_role_member_matches_botgo_channel_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.delete_guild_role_member("guild-1", "role-1", "user-1", Some("channel-1"))
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("DELETE /guilds/guild-1/members/user-1/roles/role-1 HTTP/1.1"));
        assert!(
            request.ends_with("\r\n\r\n{\"channel\":{\"id\":\"channel-1\",\"guild_id\":\"\"}}")
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn inline_mute_member_matches_botgo_omitempty_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.mute_member("guild-1", "user-1", None, Some("20"))
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PATCH /guilds/guild-1/members/user-1/mute HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n{\"mute_seconds\":\"20\"}"));
        server.await.unwrap();
    }
}
