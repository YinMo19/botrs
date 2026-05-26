use super::{BotApi, resource};
use crate::error::Result;
use crate::models::api::GatewayResponse;
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Fetches gateway URL and session-start limits for websocket startup.
    pub async fn get_gateway(&self, token: &Token) -> Result<GatewayResponse> {
        debug!("Getting gateway URL");
        let response = self
            .http
            .get(token, resource::GATEWAY_BOT, None::<&()>)
            .await?;
        Self::decode_json(response)
    }
}
