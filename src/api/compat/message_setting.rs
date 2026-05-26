use crate::api::BotApi;
use crate::error::Result;
use crate::models::message_setting::MessageSetting;

impl BotApi {
    /// Message setting API.
    #[allow(non_snake_case)]
    pub async fn GetMessageSetting(&self, guild_id: &str) -> Result<MessageSetting> {
        self.get_message_setting(self.token_required()?, guild_id)
            .await
    }
}
