use super::Context;
use crate::client::prelude::*;

impl Context {
    /// Fetches information about the current bot.
    pub async fn me(&self) -> Result<BotInfo> {
        self.api.get_bot_info(&self.token).await
    }

    /// Lists guilds visible to the current bot.
    pub async fn me_guilds(
        &self,
        guild_id: Option<&str>,
        limit: Option<u32>,
        desc: Option<bool>,
    ) -> Result<Vec<Guild>> {
        self.api.me_guilds(&self.token, guild_id, limit, desc).await
    }

    /// Fetches websocket gateway startup data.
    pub async fn get_ws_url(&self) -> Result<GatewayResponse> {
        self.api.get_ws_url(&self.token).await
    }
}
