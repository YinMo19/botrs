use crate::api_impl::{BotApi, resource};
use crate::error::Result;
use crate::models::{api::MessageResponse, message::Message};
use reqwest::Method;

impl BotApi {
    /// Posts a DM setting guide message.
    pub async fn post_dm_setting_guide(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<MessageResponse> {
        self.post_dm_setting_guide_result(guild_id, jump_guild_id)
            .await
    }

    /// Posts a DM setting guide message and returns the full message.
    pub async fn post_dm_setting_guide_message(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<Message> {
        self.post_dm_setting_guide_result(guild_id, jump_guild_id)
            .await
    }

    async fn post_dm_setting_guide_result<T>(
        &self,
        guild_id: &str,
        jump_guild_id: &str,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let body = Self::dm_setting_guide_body(jump_guild_id);
        let path = resource::dms_setting_guide(guild_id);
        self.request_json(Method::POST, &path, None::<&()>, Some(&body))
            .await
    }
}
