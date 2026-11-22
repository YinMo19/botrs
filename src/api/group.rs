use super::{BotApi, resource};
use crate::{Result, models::group::GroupBotState};

impl BotApi {
    /// Reads the receive mode, proactive messaging permission and role in one group.
    /// QQ restricts this endpoint by application (platform code 11253).
    pub async fn get_group_bot_state(&self, group_openid: &str) -> Result<GroupBotState> {
        let path = resource::group_bot_state(group_openid);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }
}
