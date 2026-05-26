use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::BotInfo,
    guild::{Guild, GuildPager},
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Fetches information about the current bot.
    pub async fn get_bot_info(&self, token: &Token) -> Result<BotInfo> {
        debug!("Getting bot info");
        let response = self.http.get(token, resource::USER_ME, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists guilds visible to the current bot using inline pagination parameters.
    pub async fn get_guilds(
        &self,
        token: &Token,
        guild_id: Option<&str>,
        limit: Option<u32>,
        desc: Option<bool>,
    ) -> Result<Vec<Guild>> {
        let mut pager = GuildPager::new();
        pager = pager.with_limit(limit.unwrap_or(100));
        if let Some(guild_id) = guild_id {
            pager = if desc.unwrap_or(false) {
                pager.with_before(guild_id)
            } else {
                pager.with_after(guild_id)
            };
        }
        self.get_guilds_with_pager(token, &pager).await
    }

    /// Lists guilds visible to the current bot using a pre-built pager.
    pub async fn get_guilds_with_pager(
        &self,
        token: &Token,
        pager: &GuildPager,
    ) -> Result<Vec<Guild>> {
        debug!("Getting guilds");

        let params = pager.query_params();

        let response = self
            .http
            .get(
                token,
                resource::USER_ME_GUILDS,
                if params.is_empty() {
                    None
                } else {
                    Some(&params)
                },
            )
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

            let body = r#"[{"id":"guild-1","name":"Guild One"}]"#;
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
    async fn get_guilds_matches_botpy_default_limit() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let guilds = api
            .get_guilds(api.token_required().unwrap(), None, None, None)
            .await
            .unwrap();

        assert_eq!(guilds[0].id, "guild-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /users/@me/guilds?limit=100 HTTP/1.1"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn get_guilds_matches_botpy_desc_cursor() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let guilds = api
            .get_guilds(
                api.token_required().unwrap(),
                Some("guild-cursor-1"),
                Some(20),
                Some(true),
            )
            .await
            .unwrap();

        assert_eq!(guilds[0].id, "guild-1");
        let request = request.await.unwrap();
        assert!(
            request.starts_with("GET /users/@me/guilds?limit=20&before=guild-cursor-1 HTTP/1.1")
                || request
                    .starts_with("GET /users/@me/guilds?before=guild-cursor-1&limit=20 HTTP/1.1")
        );
        server.await.unwrap();
    }
}
