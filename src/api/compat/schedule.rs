use crate::api::BotApi;
use crate::error::Result;
use crate::models::schedule::Schedule;

impl BotApi {
    /// Schedule list API.
    #[allow(non_snake_case)]
    pub async fn ListSchedules(&self, channel_id: &str, since: u64) -> Result<Vec<Schedule>> {
        let since = since.to_string();
        self.list_schedules_with_query(self.token_required()?, channel_id, Some(since.as_str()))
            .await
    }

    /// Schedule lookup API.
    #[allow(non_snake_case)]
    pub async fn GetSchedule(&self, channel_id: &str, schedule_id: &str) -> Result<Schedule> {
        self.get_schedule(self.token_required()?, channel_id, schedule_id)
            .await
    }

    /// Schedule creation API.
    #[allow(non_snake_case)]
    pub async fn CreateSchedule(&self, channel_id: &str, schedule: &Schedule) -> Result<Schedule> {
        self.create_schedule_with_model(self.token_required()?, channel_id, schedule)
            .await
    }

    /// Schedule modification API.
    #[allow(non_snake_case)]
    pub async fn ModifySchedule(
        &self,
        channel_id: &str,
        schedule_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule> {
        self.update_schedule_with_model(self.token_required()?, channel_id, schedule_id, schedule)
            .await
    }

    /// Schedule delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteSchedule(&self, channel_id: &str, schedule_id: &str) -> Result<()> {
        self.delete_schedule(self.token_required()?, channel_id, schedule_id)
            .await?;
        Ok(())
    }
}
