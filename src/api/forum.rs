use super::{BotApi, resource};
use crate::error::Result;
use crate::forum::{Format, ForumRsp, PostThreadRsp, ThreadInfo, ThreadToCreate};
use tracing::debug;

impl BotApi {
    /// Lists forum threads in a channel.
    pub async fn get_threads(&self, channel_id: &str) -> Result<ForumRsp> {
        debug!("Getting forum threads for channel {}", channel_id);
        let path = resource::channel_threads(channel_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Gets one forum thread's detail.
    pub async fn get_thread_detail(&self, channel_id: &str, thread_id: &str) -> Result<ThreadInfo> {
        debug!(
            "Getting forum thread {} detail for channel {}",
            thread_id, channel_id
        );
        let path = resource::channel_thread(channel_id, thread_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a forum thread from inline fields.
    pub async fn post_thread(
        &self,
        channel_id: &str,
        title: &str,
        content: &str,
        format: Format,
    ) -> Result<PostThreadRsp> {
        let body = ThreadToCreate::new(title, content, format);
        self.put_thread(channel_id, &body).await
    }

    /// Creates a forum thread from a structured body.
    pub async fn put_thread(
        &self,
        channel_id: &str,
        thread: &ThreadToCreate,
    ) -> Result<PostThreadRsp> {
        debug!("Creating forum thread in channel {}", channel_id);
        let path = resource::channel_threads(channel_id);
        let response = self
            .http
            .put(self.token(), &path, None::<&()>, Some(thread))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes one forum thread.
    pub async fn delete_thread(&self, channel_id: &str, thread_id: &str) -> Result<()> {
        debug!(
            "Deleting forum thread {} in channel {}",
            thread_id, channel_id
        );
        let path = resource::channel_thread(channel_id, thread_id);
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

            let body = r#"{"task_id":"task-1","create_time":"1710000000"}"#;
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
    async fn post_thread_uses_put_body() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let rsp = api
            .post_thread("channel-1", "Title", "Content", Format::Markdown)
            .await
            .unwrap();

        assert_eq!(rsp.task_id, "task-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("PUT /channels/channel-1/threads HTTP/1.1"));
        assert!(
            request.ends_with("\r\n\r\n{\"title\":\"Title\",\"content\":\"Content\",\"format\":3}")
        );
        server.await.unwrap();
    }
}
