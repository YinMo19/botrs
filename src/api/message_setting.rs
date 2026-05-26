use super::{BotApi, resource};
use crate::error::Result;
use crate::models::message_setting::MessageSetting;
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Gets guild message frequency settings.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Message settings for the guild.
    pub async fn get_message_setting(
        &self,
        token: &Token,
        guild_id: &str,
    ) -> Result<MessageSetting> {
        debug!("Getting message setting for guild {}", guild_id);
        let path = resource::guild_message_setting(guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }
}
