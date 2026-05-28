use crate::api_impl::{BotApi, resource};
use crate::error::Result;
use tracing::debug;

impl BotApi {
    /// Recalls (deletes) a message.
    pub async fn recall_message(
        &self,
        channel_id: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Recalling message {} in channel {}", message_id, channel_id);
        let params = hidetip.and_then(Self::hide_tip_query);
        let path = resource::channel_message(channel_id, message_id);
        self.http
            .delete(self.token(), &path, params.as_ref())
            .await?;
        Ok(())
    }

    /// Recalls (deletes) a group message.
    pub async fn recall_group_message(
        &self,
        group_openid: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!(
            "Recalling group message {} in group {}",
            message_id, group_openid
        );
        let params = hidetip.and_then(Self::hide_tip_query);
        let path = resource::group_message(group_openid, message_id);
        self.http
            .delete(self.token(), &path, params.as_ref())
            .await?;
        Ok(())
    }

    /// Recalls (deletes) a C2C message.
    pub async fn recall_c2c_message(
        &self,
        openid: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Recalling C2C message {} for {}", message_id, openid);
        let params = hidetip.and_then(Self::hide_tip_query);
        let path = resource::c2c_message(openid, message_id);
        self.http
            .delete(self.token(), &path, params.as_ref())
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
                if request.contains("\r\n\r\n") {
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

    #[tokio::test]
    async fn recall_message_omits_default_hidetip() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        api.recall_message("channel-1", "message-1", None)
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(request.starts_with("DELETE /channels/channel-1/messages/message-1 HTTP/1.1"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn recall_message_sends_true_hidetip() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        api.recall_message("channel-1", "message-1", Some(true))
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(
            request
                .starts_with("DELETE /channels/channel-1/messages/message-1?hidetip=true HTTP/1.1")
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn recall_group_message_uses_open_group_route() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        api.recall_group_message("group-openid-1", "message-1", None)
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(
            request.starts_with("DELETE /v2/groups/group-openid-1/messages/message-1 HTTP/1.1")
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn recall_c2c_message_uses_open_user_route() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;

        api.recall_c2c_message("openid-1", "message-1", Some(true))
            .await
            .unwrap();

        let request = request.await.unwrap();
        assert!(
            request
                .starts_with("DELETE /v2/users/openid-1/messages/message-1?hidetip=true HTTP/1.1")
        );
        server.await.unwrap();
    }
}
