use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::message::{DirectMessageSession, DirectMessageToCreate};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Creates a direct message session using a structured payload.
    pub async fn create_direct_message(
        &self,
        token: &Token,
        dm: &DirectMessageToCreate,
    ) -> Result<DirectMessageSession> {
        debug!(
            "Creating DM session for user {} from guild {}",
            dm.recipient_id, dm.source_guild_id
        );
        let response = self
            .http
            .post(token, resource::USER_ME_DMS, None::<&()>, Some(dm))
            .await?;
        Self::decode_json(response)
    }
}
