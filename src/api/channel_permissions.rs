use super::{BotApi, resource};
use crate::error::Result;
use crate::models::channel::{
    ChannelPermissions, ChannelRolesPermissions, UpdateChannelPermissions,
};
use crate::token::Token;
use serde::Serialize;
use tracing::debug;

#[derive(Debug, Serialize)]
struct BotpyUpdateChannelPermissions {
    add: Option<String>,
    remove: Option<String>,
}

impl BotpyUpdateChannelPermissions {
    fn new(add: Option<&str>, remove: Option<&str>) -> Self {
        Self {
            add: add.map(ToOwned::to_owned),
            remove: remove.map(ToOwned::to_owned),
        }
    }
}

impl BotApi {
    /// Fetches channel permissions for one user.
    pub async fn get_channel_user_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        debug!(
            "Getting channel permissions for user {} in channel {}",
            user_id, channel_id
        );
        let path = resource::channel_member_permissions(channel_id, user_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Updates channel permissions for one user using a structured body.
    pub async fn put_channel_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        user_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        permissions.validate()?;
        debug!(
            "Updating channel permissions for user {} in channel {}",
            user_id, channel_id
        );
        let path = resource::channel_member_permissions(channel_id, user_id);
        self.http
            .put(token, &path, None::<&()>, Some(permissions))
            .await?;
        Ok(())
    }

    /// Updates channel permissions for one user using add/remove bitsets.
    pub async fn update_channel_user_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        user_id: &str,
        add: Option<&str>,
        remove: Option<&str>,
    ) -> Result<()> {
        UpdateChannelPermissions::new(add, remove).validate()?;
        let permissions = BotpyUpdateChannelPermissions::new(add, remove);
        debug!(
            "Updating channel permissions for user {} in channel {}",
            user_id, channel_id
        );
        let path = resource::channel_member_permissions(channel_id, user_id);
        self.http
            .put(token, &path, None::<&()>, Some(&permissions))
            .await?;
        Ok(())
    }

    /// Fetches channel permissions for one role.
    pub async fn get_channel_role_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelRolesPermissions> {
        debug!(
            "Getting channel permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = resource::channel_role_permissions(channel_id, role_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Updates channel permissions for one role using a structured body.
    pub async fn put_channel_roles_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        role_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        permissions.validate()?;
        debug!(
            "Updating channel permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = resource::channel_role_permissions(channel_id, role_id);
        self.http
            .put(token, &path, None::<&()>, Some(permissions))
            .await?;
        Ok(())
    }

    /// Updates channel permissions for one role using add/remove bitsets.
    pub async fn update_channel_role_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        role_id: &str,
        add: Option<&str>,
        remove: Option<&str>,
    ) -> Result<()> {
        UpdateChannelPermissions::new(add, remove).validate()?;
        let permissions = BotpyUpdateChannelPermissions::new(add, remove);
        debug!(
            "Updating channel permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = resource::channel_role_permissions(channel_id, role_id);
        self.http
            .put(token, &path, None::<&()>, Some(&permissions))
            .await?;
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
    async fn inline_user_permission_update_matches_botpy_null_fields() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.update_channel_user_permissions(
            api.token_required().unwrap(),
            "channel-1",
            "user-1",
            Some("2"),
            None,
        )
        .await
        .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PUT /channels/channel-1/members/user-1/permissions HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n{\"add\":\"2\",\"remove\":null}"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn inline_role_permission_update_matches_botpy_null_fields() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.update_channel_role_permissions(
            api.token_required().unwrap(),
            "channel-1",
            "role-1",
            None,
            Some("4"),
        )
        .await
        .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PUT /channels/channel-1/roles/role-1/permissions HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n{\"add\":null,\"remove\":\"4\"}"));
        server.await.unwrap();
    }
}
