use super::{BotApi, resource};
use crate::error::Result;
use crate::models::schedule::{RemindType, Schedule, ScheduleWrapper};
use crate::token::Token;
use serde::Serialize;
use serde_json::Value;
use tracing::debug;

fn schedule_query(since: Option<&str>) -> ScheduleQuery<'_> {
    ScheduleQuery {
        since: since.unwrap_or("0"),
    }
}

#[derive(Serialize)]
struct ScheduleQuery<'a> {
    since: &'a str,
}

#[derive(Serialize)]
struct InlineScheduleWrapper<'a> {
    schedule: InlineScheduleBody<'a>,
}

#[derive(Serialize)]
struct InlineScheduleBody<'a> {
    name: &'a str,
    start_timestamp: &'a str,
    end_timestamp: &'a str,
    jump_channel_id: &'a str,
    reminder_id: String,
}

impl BotApi {
    // Schedule APIs

    /// Lists schedules in a channel, optionally filtering by start timestamp.
    pub async fn get_schedules(
        &self,
        token: &Token,
        channel_id: &str,
        since: Option<&str>,
    ) -> Result<Vec<Schedule>> {
        debug!("Getting schedules for channel {}", channel_id);

        self.list_schedules_with_query(token, channel_id, since)
            .await
    }

    /// Lists schedules using botgo's query-parameter request shape.
    pub(crate) async fn list_schedules_with_query(
        &self,
        token: &Token,
        channel_id: &str,
        since: Option<&str>,
    ) -> Result<Vec<Schedule>> {
        debug!("Getting schedules for channel {} with query", channel_id);

        let query = schedule_query(since);
        let path = resource::channel_schedules(channel_id);
        let response = self.http.get(token, &path, Some(&query)).await?;
        Self::decode_json(response)
    }

    /// Fetches one schedule by ID.
    pub async fn get_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<Schedule> {
        debug!("Getting schedule {} in channel {}", schedule_id, channel_id);

        let path = resource::channel_schedule(channel_id, schedule_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a schedule from inline fields.
    pub async fn create_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        remind_type: RemindType,
    ) -> Result<Schedule> {
        self.create_schedule_with_reminder_id(
            token,
            channel_id,
            name,
            start_timestamp,
            end_timestamp,
            jump_channel_id,
            remind_type.to_wire_string(),
        )
        .await
    }

    /// Creates a schedule from inline fields using a raw reminder ID.
    pub async fn create_schedule_with_reminder_id(
        &self,
        token: &Token,
        channel_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        reminder_id: impl ToString,
    ) -> Result<Schedule> {
        let wrapper = InlineScheduleWrapper {
            schedule: InlineScheduleBody {
                name,
                start_timestamp,
                end_timestamp,
                jump_channel_id,
                reminder_id: reminder_id.to_string(),
            },
        };
        let path = resource::channel_schedules(channel_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&wrapper))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a schedule from a structured model.
    pub async fn create_schedule_with_model(
        &self,
        token: &Token,
        channel_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule> {
        debug!(
            "Creating schedule '{}' in channel {}",
            schedule.name, channel_id
        );
        let wrapper = ScheduleWrapper::new(schedule.clone());
        let path = resource::channel_schedules(channel_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&wrapper))
            .await?;
        Self::decode_json(response)
    }

    /// Updates a schedule from inline fields.
    pub async fn update_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        remind_type: RemindType,
    ) -> Result<Schedule> {
        self.update_schedule_with_reminder_id(
            token,
            channel_id,
            schedule_id,
            name,
            start_timestamp,
            end_timestamp,
            jump_channel_id,
            remind_type.to_wire_string(),
        )
        .await
    }

    /// Updates a schedule from inline fields using a raw reminder ID.
    pub async fn update_schedule_with_reminder_id(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        reminder_id: impl ToString,
    ) -> Result<Schedule> {
        let wrapper = InlineScheduleWrapper {
            schedule: InlineScheduleBody {
                name,
                start_timestamp,
                end_timestamp,
                jump_channel_id,
                reminder_id: reminder_id.to_string(),
            },
        };
        let path = resource::channel_schedule(channel_id, schedule_id);
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(&wrapper))
            .await?;
        Self::decode_json(response)
    }

    /// Updates a schedule from a structured model.
    pub async fn update_schedule_with_model(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule> {
        debug!(
            "Updating schedule {} in channel {}",
            schedule_id, channel_id
        );

        let wrapper = ScheduleWrapper::new(schedule.clone());
        let path = resource::channel_schedule(channel_id, schedule_id);
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(&wrapper))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a schedule and returns the raw platform response.
    pub async fn delete_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<Value> {
        debug!(
            "Deleting schedule {} in channel {}",
            schedule_id, channel_id
        );

        let path = resource::channel_schedule(channel_id, schedule_id);
        let response = self.http.delete(token, &path, None::<&()>).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::{InlineScheduleBody, InlineScheduleWrapper, schedule_query};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn test_api(base_url: String) -> crate::api::BotApi {
        let token = crate::Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let mut http = crate::http::HttpClient::new(30, false).unwrap();
        http.base_url = base_url;
        crate::api::BotApi::with_token(http, token)
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

            let body = r#"[{"id":"schedule-1","name":"meeting"}]"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    #[test]
    fn schedule_query_defaults_since_to_zero() {
        let value = serde_json::to_value(schedule_query(None)).unwrap();
        assert_eq!(value["since"], "0");

        let value = serde_json::to_value(schedule_query(Some("1710000000"))).unwrap();
        assert_eq!(value["since"], "1710000000");
    }

    #[tokio::test]
    async fn get_schedules_uses_query_parameters() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let schedules = api
            .get_schedules(api.token_required().unwrap(), "channel-1", None)
            .await
            .unwrap();

        assert_eq!(schedules[0].id, "schedule-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /channels/channel-1/schedules?since=0 HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn list_schedules_with_query_preserves_botgo_shape() {
        let (base_url, request, server) = spawn_capture_server().await;
        let api = test_api(base_url).await;
        let schedules = api
            .list_schedules_with_query(api.token_required().unwrap(), "channel-1", Some("0"))
            .await
            .unwrap();

        assert_eq!(schedules[0].id, "schedule-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /channels/channel-1/schedules?since=0 HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n"));
        server.await.unwrap();
    }

    #[test]
    fn inline_schedule_body_uses_schedule_wrapper() {
        let value = serde_json::to_value(InlineScheduleWrapper {
            schedule: InlineScheduleBody {
                name: "meeting",
                start_timestamp: "1640995200",
                end_timestamp: "1640998800",
                jump_channel_id: "channel-1",
                reminder_id: "0".to_string(),
            },
        })
        .unwrap();

        assert_eq!(value["schedule"]["name"], "meeting");
        assert_eq!(value["schedule"]["start_timestamp"], "1640995200");
        assert_eq!(value["schedule"]["end_timestamp"], "1640998800");
        assert_eq!(value["schedule"]["jump_channel_id"], "channel-1");
        assert_eq!(value["schedule"]["reminder_id"], "0");
        assert!(value["schedule"].get("remind_type").is_none());
    }
}
