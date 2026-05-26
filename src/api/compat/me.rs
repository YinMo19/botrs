use crate::api::BotApi;
use crate::error::Result;
use crate::models::guild::{Guild, GuildPager};
use crate::models::user::User;

impl BotApi {
    /// Current bot user API.
    #[allow(non_snake_case)]
    pub async fn Me(&self) -> Result<User> {
        Ok(self.get_bot_info(self.token_required()?).await?.into())
    }

    /// Current bot guild list API.
    #[allow(non_snake_case)]
    pub async fn MeGuilds(&self, pager: &GuildPager) -> Result<Vec<Guild>> {
        self.get_guilds_with_pager(self.token_required()?, pager)
            .await
    }
}
