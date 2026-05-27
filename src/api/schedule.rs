use super::{BotApi, resource};
use crate::error::Result;
use crate::models::schedule::{Schedule, ScheduleWrapper};
use serde::Serialize;
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

impl BotApi {
    // Schedule APIs

    /// Lists schedules in a channel, optionally filtering by start timestamp.
    pub async fn get_schedules(
        &self,
        channel_id: &str,
        since: Option<&str>,
    ) -> Result<Vec<Schedule>> {
        debug!("Getting schedules for channel {}", channel_id);

        let query = schedule_query(since);
        let path = resource::channel_schedules(channel_id);
        let response = self.http.get(self.token(), &path, Some(&query)).await?;
        Self::decode_json(response)
    }

    /// Fetches one schedule by ID.
    pub async fn get_schedule(&self, channel_id: &str, schedule_id: &str) -> Result<Schedule> {
        debug!("Getting schedule {} in channel {}", schedule_id, channel_id);

        let path = resource::channel_schedule(channel_id, schedule_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a schedule.
    pub async fn create_schedule(&self, channel_id: &str, schedule: &Schedule) -> Result<Schedule> {
        debug!(
            "Creating schedule '{}' in channel {}",
            schedule.name, channel_id
        );
        let wrapper = ScheduleWrapper {
            schedule: Some(schedule.clone()),
        };
        let path = resource::channel_schedules(channel_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&wrapper))
            .await?;
        Self::decode_json(response)
    }

    /// Updates a schedule.
    pub async fn update_schedule(
        &self,
        channel_id: &str,
        schedule_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule> {
        debug!(
            "Updating schedule {} in channel {}",
            schedule_id, channel_id
        );

        let wrapper = ScheduleWrapper {
            schedule: Some(schedule.clone()),
        };
        let path = resource::channel_schedule(channel_id, schedule_id);
        let response = self
            .http
            .patch(self.token(), &path, None::<&()>, Some(&wrapper))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a schedule.
    pub async fn delete_schedule(&self, channel_id: &str, schedule_id: &str) -> Result<()> {
        debug!(
            "Deleting schedule {} in channel {}",
            schedule_id, channel_id
        );

        let path = resource::channel_schedule(channel_id, schedule_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::schedule_query;
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
        crate::api::BotApi::new(http, token)
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
        let schedules = api.get_schedules("channel-1", None).await.unwrap();

        assert_eq!(schedules[0].id, "schedule-1");
        let request = request.await.unwrap();
        assert!(request.starts_with("GET /channels/channel-1/schedules?since=0 HTTP/1.1"));
        assert!(request.ends_with("\r\n\r\n"));
        server.await.unwrap();
    }
}
