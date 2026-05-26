use super::{BotApi, resource};
use crate::error::Result;
use crate::models::schedule::{RemindType, Schedule, ScheduleWrapper};
use crate::token::Token;
use serde::Serialize;
use serde_json::Value;
use tracing::debug;

impl BotApi {
    // Schedule APIs

    /// Gets the list of schedules for a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `since` - Optional timestamp to get schedules after this time
    ///
    /// # Returns
    ///
    /// List of schedules.
    pub async fn get_schedules(
        &self,
        token: &Token,
        channel_id: &str,
        since: Option<&str>,
    ) -> Result<Vec<Schedule>> {
        debug!("Getting schedules for channel {}", channel_id);

        #[derive(Serialize)]
        struct ScheduleQuery<'a> {
            since: &'a str,
        }

        let query = since.map(|since| ScheduleQuery { since });
        let path = resource::channel_schedules(channel_id);
        let response = self.http.get(token, &path, query.as_ref()).await?;
        Self::decode_json(response)
    }

    /// Gets a specific schedule by ID.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `schedule_id` - The schedule ID
    ///
    /// # Returns
    ///
    /// The schedule details.
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

    /// Creates a new schedule in a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `name` - Name of the schedule
    /// * `start_timestamp` - Start time as Unix timestamp string
    /// * `end_timestamp` - End time as Unix timestamp string
    /// * `jump_channel_id` - Channel ID to jump to when event starts
    /// * `remind_type` - Type of reminder to set
    ///
    /// # Returns
    ///
    /// The created schedule.
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

    /// Creates a new schedule in a channel from a schedule model.
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

    /// Updates an existing schedule.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `schedule_id` - The schedule ID to update
    /// * `name` - New name of the schedule
    /// * `start_timestamp` - New start time as Unix timestamp string
    /// * `end_timestamp` - New end time as Unix timestamp string
    /// * `jump_channel_id` - New channel ID to jump to when event starts
    /// * `remind_type` - New type of reminder to set
    ///
    /// # Returns
    ///
    /// The updated schedule.
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

    /// Updates an existing schedule from a schedule model.
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

    /// Deletes a schedule.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `schedule_id` - The schedule ID to delete
    ///
    /// # Returns
    ///
    /// Success indication.
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
