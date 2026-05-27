use super::{BotApi, resource};
use crate::error::Result;
use crate::models::guild::{GuildRole, GuildRoles, UpdateResult, UpdateRole};
use tracing::debug;

impl BotApi {
    // Guild Role APIs

    /// Lists roles configured in a guild.
    pub async fn get_guild_roles(&self, guild_id: &str) -> Result<GuildRoles> {
        debug!("Getting guild roles for {}", guild_id);
        let path = resource::guild_roles(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a guild role.
    pub async fn create_guild_role(&self, guild_id: &str, role: GuildRole) -> Result<UpdateResult> {
        debug!("Creating guild role in {}", guild_id);
        let body = UpdateRole::new(guild_id, role);
        let path = resource::guild_roles(guild_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Updates a guild role.
    pub async fn update_guild_role(
        &self,
        guild_id: &str,
        role_id: &str,
        role: GuildRole,
    ) -> Result<UpdateResult> {
        debug!("Updating guild role {} in {}", role_id, guild_id);
        let body = UpdateRole::new(guild_id, role);
        let path = resource::guild_role(guild_id, role_id);
        let response = self
            .http
            .patch(self.token(), &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a guild role.
    pub async fn delete_guild_role(&self, guild_id: &str, role_id: &str) -> Result<()> {
        debug!("Deleting guild role {} in {}", role_id, guild_id);
        let path = resource::guild_role(guild_id, role_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
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

            let body = r#"{"role_id":"role-1","guild_id":"guild-1","role":{"id":"role-1","name":"Test Role","color":4278245297,"hoist":0}}"#;
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
    async fn create_role_sends_update_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let role = GuildRole {
            name: "Test Role".to_string(),
            color: 4_278_245_297,
            ..Default::default()
        };
        let result = api.create_guild_role("guild-1", role).await.unwrap();

        assert_eq!(result.role_id, "role-1");
        assert_eq!(result.guild_id, "guild-1");
        assert_eq!(result.role.as_ref().unwrap().id, "role-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /guilds/guild-1/roles HTTP/1.1"));
        assert!(request.ends_with(
            "\r\n\r\n{\"guild_id\":\"guild-1\",\"filter\":{\"name\":1,\"color\":1,\"hoist\":1},\"info\":{\"name\":\"Test Role\",\"color\":4278245297,\"hoist\":0}}"
        ));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn update_role_sends_update_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let role = GuildRole {
            name: "Test Role".to_string(),
            hoist: 0,
            ..Default::default()
        };
        let result = api
            .update_guild_role("guild-1", "role-1", role)
            .await
            .unwrap();

        assert_eq!(result.role_id, "role-1");
        assert_eq!(result.guild_id, "guild-1");
        assert_eq!(result.role.as_ref().unwrap().name, "Test Role");
        let request = request.await.unwrap();
        assert!(request.starts_with("PATCH /guilds/guild-1/roles/role-1 HTTP/1.1"));
        assert!(request.ends_with(
            "\r\n\r\n{\"guild_id\":\"guild-1\",\"filter\":{\"name\":1,\"color\":1,\"hoist\":1},\"info\":{\"name\":\"Test Role\",\"color\":4278245297,\"hoist\":0}}"
        ));
        server.await.unwrap();
    }
}
