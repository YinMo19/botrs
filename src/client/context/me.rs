use super::Context;
use crate::client::prelude::*;

impl Context {
    /// Fetches information about the current bot.
    pub async fn get_bot_info(&self) -> Result<BotInfo> {
        self.api.get_bot_info(&self.token).await
    }

    /// Fetches websocket gateway startup data.
    pub async fn get_gateway(&self) -> Result<GatewayResponse> {
        self.api.get_gateway(&self.token).await
    }
}
