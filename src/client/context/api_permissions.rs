use super::Context;
use crate::client::prelude::*;

impl Context {
    /// Lists API permissions available in a guild.
    pub async fn get_api_permissions(&self, guild_id: &str) -> Result<APIPermissions> {
        self.api.get_api_permissions(&self.token, guild_id).await
    }

    /// Creates an API permission demand request with a structured body.
    pub async fn require_api_permissions(
        &self,
        guild_id: &str,
        demand: &APIPermissionDemandToCreate,
    ) -> Result<APIPermissionDemand> {
        self.api
            .require_api_permissions(&self.token, guild_id, demand)
            .await
    }

    /// Creates an API permission demand request from inline fields.
    pub async fn post_permission_demand(
        &self,
        guild_id: &str,
        channel_id: &str,
        api_identify: APIPermissionDemandIdentify,
        desc: &str,
    ) -> Result<APIPermissionDemand> {
        self.api
            .post_permission_demand(&self.token, guild_id, channel_id, api_identify, desc)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn test_context(base_url: String) -> Context {
        let token = Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let mut http = HttpClient::new(30, false).unwrap();
        http.base_url = base_url;
        Context::new(Arc::new(BotApi::with_token(http, token.clone())), token)
    }

    async fn spawn_capture_server(
        response_body: &'static str,
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

            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
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
    async fn get_api_permissions_returns_permission_list() {
        let (base_url, request, server) = spawn_capture_server(
            r#"{"apis":[{"path":"/channels/{channel_id}/messages","method":"POST","desc":"Send message","auth_status":1}]}"#,
        )
        .await;
        let ctx = test_context(base_url).await;
        let permissions = ctx.get_api_permissions("guild-1").await.unwrap();

        assert_eq!(permissions.api_list.len(), 1);
        assert_eq!(
            permissions.api_list[0].path,
            "/channels/{channel_id}/messages"
        );
        assert_eq!(permissions.api_list[0].auth_status, 1);
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /guilds/guild-1/api_permission HTTP/1.1"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn post_permission_demand_uses_platform_body_shape() {
        let (base_url, request, server) = spawn_capture_server(
            r#"{"guild_id":"guild-1","channel_id":"channel-1","api_identify":{"path":"/channels/{channel_id}/messages","method":"POST"},"title":"Send","desc":"Need to send"}"#,
        )
        .await;
        let ctx = test_context(base_url).await;
        let demand = ctx
            .post_permission_demand(
                "guild-1",
                "channel-1",
                APIPermissionDemandIdentify::post_messages(),
                "Need to send",
            )
            .await
            .unwrap();

        assert_eq!(demand.guild_id, "guild-1");
        assert_eq!(demand.api_method(), "POST");
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /guilds/guild-1/api_permission/demand HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "channel_id": "channel-1",
                "api_identify": {
                    "path": "/channels/{channel_id}/messages",
                    "method": "POST"
                },
                "desc": "Need to send"
            })
        );
        server.await.unwrap();
    }
}
