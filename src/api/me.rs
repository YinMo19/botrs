use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    api::BotInfo,
    guild::{Guild, GuildPager},
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Fetches information about the current bot.
    pub async fn get_bot_info(&self, token: &Token) -> Result<BotInfo> {
        debug!("Getting bot info");
        let response = self.http.get(token, resource::USER_ME, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists guilds visible to the current bot using inline pagination parameters.
    pub async fn get_guilds(
        &self,
        token: &Token,
        guild_id: Option<&str>,
        limit: Option<u32>,
        desc: Option<bool>,
    ) -> Result<Vec<Guild>> {
        let mut pager = GuildPager::new();
        if let Some(limit) = limit {
            pager = pager.with_limit(limit);
        }
        if let Some(guild_id) = guild_id {
            pager = if desc.unwrap_or(false) {
                pager.with_before(guild_id)
            } else {
                pager.with_after(guild_id)
            };
        }
        self.get_guilds_with_pager(token, &pager).await
    }

    /// Lists guilds visible to the current bot using a pre-built pager.
    pub async fn get_guilds_with_pager(
        &self,
        token: &Token,
        pager: &GuildPager,
    ) -> Result<Vec<Guild>> {
        debug!("Getting guilds");

        let params = pager.query_params();

        let response = self
            .http
            .get(
                token,
                resource::USER_ME_GUILDS,
                if params.is_empty() {
                    None
                } else {
                    Some(&params)
                },
            )
            .await?;
        Self::decode_json(response)
    }
}
