use super::Context;
use crate::client::prelude::*;
use serde_json::Value;

impl Context {
    /// Lists schedules in a channel, optionally filtering by start timestamp.
    pub async fn get_schedules(
        &self,
        channel_id: &str,
        since: Option<&str>,
    ) -> Result<Vec<Schedule>> {
        self.api.get_schedules(&self.token, channel_id, since).await
    }

    /// Fetches one schedule by ID.
    pub async fn get_schedule(&self, channel_id: &str, schedule_id: &str) -> Result<Schedule> {
        self.api
            .get_schedule(&self.token, channel_id, schedule_id)
            .await
    }

    /// Creates a schedule from inline fields.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_schedule(
        &self,
        channel_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        remind_type: RemindType,
    ) -> Result<Schedule> {
        self.api
            .create_schedule(
                &self.token,
                channel_id,
                name,
                start_timestamp,
                end_timestamp,
                jump_channel_id,
                remind_type,
            )
            .await
    }

    /// Creates a schedule from inline fields using a raw reminder ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_schedule_with_reminder_id(
        &self,
        channel_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        reminder_id: impl ToString,
    ) -> Result<Schedule> {
        self.api
            .create_schedule_with_reminder_id(
                &self.token,
                channel_id,
                name,
                start_timestamp,
                end_timestamp,
                jump_channel_id,
                reminder_id,
            )
            .await
    }

    /// Creates a schedule from a structured model.
    pub async fn create_schedule_with_model(
        &self,
        channel_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule> {
        self.api
            .create_schedule_with_model(&self.token, channel_id, schedule)
            .await
    }

    /// Updates a schedule from inline fields.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_schedule(
        &self,
        channel_id: &str,
        schedule_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        remind_type: RemindType,
    ) -> Result<Schedule> {
        self.api
            .update_schedule(
                &self.token,
                channel_id,
                schedule_id,
                name,
                start_timestamp,
                end_timestamp,
                jump_channel_id,
                remind_type,
            )
            .await
    }

    /// Updates a schedule from inline fields using a raw reminder ID.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_schedule_with_reminder_id(
        &self,
        channel_id: &str,
        schedule_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        reminder_id: impl ToString,
    ) -> Result<Schedule> {
        self.api
            .update_schedule_with_reminder_id(
                &self.token,
                channel_id,
                schedule_id,
                name,
                start_timestamp,
                end_timestamp,
                jump_channel_id,
                reminder_id,
            )
            .await
    }

    /// Updates a schedule from a structured model.
    pub async fn update_schedule_with_model(
        &self,
        channel_id: &str,
        schedule_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule> {
        self.api
            .update_schedule_with_model(&self.token, channel_id, schedule_id, schedule)
            .await
    }

    /// Deletes a schedule and returns the raw platform response.
    pub async fn delete_schedule(&self, channel_id: &str, schedule_id: &str) -> Result<Value> {
        self.api
            .delete_schedule(&self.token, channel_id, schedule_id)
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
    async fn get_schedules_uses_botpy_body_shape() {
        let (base_url, request, server) =
            spawn_capture_server(r#"[{"id":"schedule-1","name":"meeting"}]"#).await;
        let ctx = test_context(base_url).await;
        let schedules = ctx.get_schedules("channel-1", None).await.unwrap();

        assert_eq!(schedules[0].id, "schedule-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /channels/channel-1/schedules HTTP/1.1"));
        assert_eq!(request_body(&request), serde_json::json!({"since": null}));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn create_schedule_uses_inline_body_shape() {
        let (base_url, request, server) =
            spawn_capture_server(r#"{"id":"schedule-1","name":"meeting"}"#).await;
        let ctx = test_context(base_url).await;
        let schedule = ctx
            .create_schedule(
                "channel-1",
                "meeting",
                "1640995200",
                "1640998800",
                "channel-2",
                RemindType::None,
            )
            .await
            .unwrap();

        assert_eq!(schedule.id, "schedule-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("POST /channels/channel-1/schedules HTTP/1.1"));
        assert_eq!(
            request_body(&request),
            serde_json::json!({
                "schedule": {
                    "name": "meeting",
                    "start_timestamp": "1640995200",
                    "end_timestamp": "1640998800",
                    "jump_channel_id": "channel-2",
                    "reminder_id": "0"
                }
            })
        );
        server.await.unwrap();
    }
}
