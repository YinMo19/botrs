use super::{BotApi, HeaderCallbackAppID, resource};
use crate::error::Result;
use crate::token::Token;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use tracing::debug;

#[derive(Serialize)]
struct InteractionResultBody {
    code: i32,
}

impl BotApi {
    /// Sends an interaction result code.
    pub async fn on_interaction_result(
        &self,
        token: &Token,
        interaction_id: &str,
        code: i32,
    ) -> Result<()> {
        debug!("Sending interaction result {} for {}", code, interaction_id);
        let path = resource::interaction(interaction_id);
        let body = InteractionResultBody { code };
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Sends a raw interaction response body with the callback app ID header.
    pub async fn put_interaction(
        &self,
        token: &Token,
        interaction_id: &str,
        body: &str,
    ) -> Result<()> {
        debug!("Updating interaction {}", interaction_id);
        let mut headers = HeaderMap::new();
        let app_id = if self.app_id.is_empty() {
            token.app_id()
        } else {
            &self.app_id
        };
        let app_id = HeaderValue::from_str(app_id)
            .map_err(|e| crate::BotError::invalid_data(format!("Invalid app ID header: {e}")))?;
        headers.insert(HeaderCallbackAppID, app_id);

        let path = resource::interaction(interaction_id);
        self.http
            .put_raw_with_headers(token, &path, None::<&()>, body, headers)
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
    async fn on_interaction_result_uses_json_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.on_interaction_result(api.token_required().unwrap(), "interaction-1", 0)
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PUT /interactions/interaction-1 HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n{\"code\":0}"));
        assert!(!request.to_ascii_lowercase().contains("x-callback-appid:"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn put_interaction_keeps_botgo_callback_header_and_raw_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.put_interaction(
            api.token_required().unwrap(),
            "interaction-1",
            r#"{"code":0}"#,
        )
        .await
        .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PUT /interactions/interaction-1 HTTP/1.1"));
        assert!(
            request.to_ascii_lowercase().contains(
                "x-callback-appid: APPID_XXXXXX"
                    .to_ascii_lowercase()
                    .as_str()
            )
        );
        assert!(request.ends_with("\r\n\r\n{\"code\":0}"));
        server.await.unwrap();
    }
}
