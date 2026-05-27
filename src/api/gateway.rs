use super::{BotApi, resource};
use crate::error::Result;
use crate::models::api::GatewayResponse;
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Fetches gateway URL and session-start limits for websocket startup.
    pub async fn get_gateway(&self, token: &Token) -> Result<GatewayResponse> {
        debug!("Getting gateway URL");
        let response = self
            .http
            .get(token, resource::GATEWAY_BOT, None::<&()>)
            .await?;
        Self::decode_json(response)
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

            let body = r#"{"url":"wss://example.com/websocket","shards":1,"session_start_limit":{"total":1,"remaining":1,"reset_after":1,"max_concurrency":1}}"#;
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
    async fn get_gateway_uses_gateway_path() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        let gateway = api.get_gateway(api.token().unwrap()).await.unwrap();

        assert_eq!(gateway.url, "wss://example.com/websocket");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /gateway/bot HTTP/1.1"));
        server.await.unwrap();
    }
}
