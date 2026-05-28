use super::{BotApi, resource};
use crate::error::{BotError, Result};
use reqwest::{
    Method,
    header::{HeaderMap, HeaderValue},
};
use serde::Serialize;
use serde_json::Value;
use tracing::debug;

impl BotApi {
    /// Updates an interaction response.
    pub async fn put_interaction<T>(&self, interaction_id: &str, body: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        debug!("Putting interaction {}", interaction_id);
        let path = resource::interaction(interaction_id);
        let body = interaction_body(body)?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Callback-AppID",
            HeaderValue::from_str(self.token().app_id()).map_err(|err| {
                BotError::internal(format!("invalid X-Callback-AppID header: {err}"))
            })?,
        );
        self.http
            .request_with_headers(
                Method::PUT,
                self.token(),
                &path,
                None::<&()>,
                Some(&body),
                headers,
            )
            .await?;
        Ok(())
    }
}

fn interaction_body<T>(body: &T) -> Result<Value>
where
    T: Serialize + ?Sized,
{
    let value = serde_json::to_value(body)?;
    if let Value::String(raw) = &value
        && let Ok(raw_json) = serde_json::from_str::<Value>(raw)
    {
        return Ok(raw_json);
    }
    Ok(value)
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

            let response = "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}";
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    fn request_body(request: &str) -> serde_json::Value {
        let body = request
            .split_once("\r\n\r\n")
            .map(|(_, body)| body)
            .unwrap_or_default();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn put_interaction_sends_typed_json_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        api.put_interaction("interaction-1", &serde_json::json!({"ok": true}))
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("PUT /interactions/interaction-1 HTTP/1.1"));
        assert!(request.contains("x-callback-appid: APPID_XXXXXX"));
        assert_eq!(request_body(&request), serde_json::json!({"ok": true}));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn put_interaction_accepts_raw_json_string_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let body = r#"{"layouts":[]}"#.to_string();
        api.put_interaction("interaction-1", &body).await.unwrap();

        let request = request.await.unwrap();
        assert_eq!(request_body(&request), serde_json::json!({"layouts": []}));
        server.await.unwrap();
    }
}
