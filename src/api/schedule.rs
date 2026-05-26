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
        let schedule = Schedule::new(
            name,
            start_timestamp,
            end_timestamp,
            Some(jump_channel_id.to_string()),
            remind_type,
        );
        self.create_schedule_with_model(token, channel_id, &schedule)
            .await
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
        let schedule = Schedule::new(
            name,
            start_timestamp,
            end_timestamp,
            Some(jump_channel_id.to_string()),
            remind_type,
        );
        self.update_schedule_with_model(token, channel_id, schedule_id, &schedule)
            .await
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
    use super::schedule_query;

    #[test]
    fn schedule_query_defaults_since_to_zero() {
        let value = serde_json::to_value(schedule_query(None)).unwrap();
        assert_eq!(value["since"], "0");

        let value = serde_json::to_value(schedule_query(Some("1710000000"))).unwrap();
        assert_eq!(value["since"], "1710000000");
    }
}
