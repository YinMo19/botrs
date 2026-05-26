use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Recalls a direct message.
    pub async fn retract_dm_message(
        &self,
        token: &Token,
        guild_id: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Retracting DM message {} in {}", message_id, guild_id);
        self.retract_open_message(token, resource::dms_message(guild_id, message_id), hidetip)
            .await
    }
}
