use super::{BotApi, resource};
use crate::error::Result;
use crate::models::guild::{
    Guild, GuildMembersPager, GuildRoleMembers, GuildRoleMembersPager, Member, MemberDeleteOptions,
    UpdateGuildMute, normalize_delete_history_msg_days,
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    // Guild-related APIs

    /// Fetches one guild by ID.
    pub async fn get_guild(&self, token: &Token, guild_id: &str) -> Result<Guild> {
        debug!("Getting guild {}", guild_id);
        let path = resource::guild(guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    // Member APIs

    /// Fetches one guild member.
    pub async fn get_guild_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
    ) -> Result<Member> {
        debug!("Getting guild member {} in {}", user_id, guild_id);
        let path = resource::guild_member(guild_id, user_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists members currently in a voice channel.
    pub async fn get_voice_members(&self, token: &Token, channel_id: &str) -> Result<Vec<Member>> {
        debug!("Getting voice members for channel {}", channel_id);
        let path = resource::voice_channel_members(channel_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists guild members using inline pagination parameters.
    pub async fn get_guild_members(
        &self,
        token: &Token,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Member>> {
        let pager = GuildMembersPager::new(after.unwrap_or("0"), limit.unwrap_or(1).to_string());
        self.get_guild_members_with_pager(token, guild_id, &pager)
            .await
    }

    /// Lists guild members using a pre-built pager.
    pub async fn get_guild_members_with_pager(
        &self,
        token: &Token,
        guild_id: &str,
        pager: &GuildMembersPager,
    ) -> Result<Vec<Member>> {
        debug!(
            "Getting guild members for {} with pager {:?}",
            guild_id, pager
        );
        let path = resource::guild_members(guild_id);
        let response = self.http.get(token, &path, Some(pager)).await?;
        Self::decode_json(response)
    }

    /// Lists members that have a guild role.
    pub async fn get_guild_role_members(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        start_index: Option<&str>,
        limit: Option<u32>,
    ) -> Result<GuildRoleMembers> {
        let pager =
            GuildRoleMembersPager::new(start_index.unwrap_or("0"), limit.unwrap_or(1).to_string());
        self.get_guild_role_members_with_pager(token, guild_id, role_id, &pager)
            .await
    }

    /// Lists members that have a guild role using a pre-built pager.
    pub async fn get_guild_role_members_with_pager(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        pager: &GuildRoleMembersPager,
    ) -> Result<GuildRoleMembers> {
        debug!(
            "Getting role {} members for guild {} with pager {:?}",
            role_id, guild_id, pager
        );
        let path = resource::guild_role_members(guild_id, role_id);
        let response = self.http.get(token, &path, Some(pager)).await?;
        Self::decode_json(response)
    }

    /// Removes a member from a guild.
    ///
    /// `delete_history_msg_days` is normalized to the platform-supported values.
    pub async fn delete_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        add_blacklist: Option<bool>,
        delete_history_msg_days: Option<i32>,
    ) -> Result<()> {
        let options = MemberDeleteOptions {
            add_blacklist: add_blacklist.unwrap_or(false),
            delete_history_msg_days: normalize_delete_history_msg_days(
                delete_history_msg_days.unwrap_or(0),
            ),
        };

        self.delete_member_with_options(token, guild_id, user_id, &options)
            .await
    }

    /// Removes a member using inline delete options.
    pub async fn get_delete_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        add_blacklist: Option<bool>,
        delete_history_msg_days: Option<i32>,
    ) -> Result<()> {
        self.delete_member(
            token,
            guild_id,
            user_id,
            add_blacklist,
            delete_history_msg_days,
        )
        .await
    }

    /// Removes a member from a guild using explicit delete options.
    pub async fn delete_member_with_options(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        options: &MemberDeleteOptions,
    ) -> Result<()> {
        debug!("Deleting member {} from guild {}", user_id, guild_id);

        let path = resource::guild_member(guild_id, user_id);
        self.http
            .delete_with_body(token, &path, None::<&()>, Some(options))
            .await?;
        Ok(())
    }

    // Muting APIs

    /// Mutes every member in a guild.
    ///
    /// The platform accepts either an end timestamp or a duration in seconds.
    pub async fn mute_all(
        &self,
        token: &Token,
        guild_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting all members in guild {}", guild_id);

        let body = UpdateGuildMute::new(mute_end_timestamp, mute_seconds);

        let path = resource::guild_mute(guild_id);
        self.http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Cancels the guild-wide mute.
    pub async fn cancel_mute_all(&self, token: &Token, guild_id: &str) -> Result<()> {
        debug!("Canceling mute for all members in guild {}", guild_id);

        let body = UpdateGuildMute::cancel();

        let path = resource::guild_mute(guild_id);
        self.http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BotApi;
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
                if request.contains("\r\n\r\n") {
                    break;
                }
            }

            let request = String::from_utf8_lossy(&request_bytes).to_string();
            let _ = tx.send(request);

            let body = r#"[{"guild_id":"guild-1","nick":"voice-user"}]"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    async fn spawn_empty_response_capture_server() -> (
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

            let body = r#"{}"#;
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
    async fn get_voice_members_uses_voice_path() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        let members = api
            .get_voice_members(api.token_required().unwrap(), "channel-1")
            .await
            .unwrap();

        assert_eq!(members[0].nick, "voice-user");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /channels/channel-1/voice/members HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn get_delete_member_alias_uses_default_body() {
        let (base_url, request, server) = spawn_empty_response_capture_server().await;
        let api = test_api(base_url).await;

        api.get_delete_member(
            api.token_required().unwrap(),
            "guild-1",
            "user-1",
            None,
            Some(42),
        )
        .await
        .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("DELETE /guilds/guild-1/members/user-1 HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "add_blacklist": false,
                "delete_history_msg_days": 0
            })
        );
        server.await.unwrap();
    }
}
