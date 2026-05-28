use super::{BotApi, resource};
use crate::error::Result;
use crate::models::guild::{
    Guild, GuildMembersPager, GuildPager, GuildRoleMembers, GuildRoleMembersPager, Member,
    MemberDeleteOptions, MessageSetting,
};
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Fetches one guild.
    pub async fn get_guild(&self, guild_id: &str) -> Result<Guild> {
        debug!("Getting guild {}", guild_id);
        let path = resource::guild(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists guilds the bot belongs to.
    pub async fn list_bot_guilds(&self, pager: &GuildPager) -> Result<Vec<Guild>> {
        debug!("Listing bot guilds");
        let query = pager.to_query_params();
        let response = if query.is_empty() {
            self.http
                .get(self.token(), resource::USER_ME_GUILDS, None::<&()>)
                .await?
        } else {
            self.http
                .get(self.token(), resource::USER_ME_GUILDS, Some(&query))
                .await?
        };
        Self::decode_json(response)
    }

    /// Fetches one guild member.
    pub async fn get_guild_member(&self, guild_id: &str, user_id: &str) -> Result<Member> {
        debug!("Getting member {} in guild {}", user_id, guild_id);
        let path = resource::guild_member(guild_id, user_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists guild members.
    pub async fn list_guild_members(
        &self,
        guild_id: &str,
        pager: &GuildMembersPager,
    ) -> Result<Vec<Member>> {
        debug!("Listing members in guild {}", guild_id);
        let query = pager.to_query_params();
        let path = resource::guild_members(guild_id);
        let response = if query.is_empty() {
            self.http.get(self.token(), &path, None::<&()>).await?
        } else {
            self.http.get(self.token(), &path, Some(&query)).await?
        };
        Self::decode_json(response)
    }

    /// Lists members assigned to a guild role.
    pub async fn list_guild_role_members(
        &self,
        guild_id: &str,
        role_id: &str,
        pager: &GuildRoleMembersPager,
    ) -> Result<GuildRoleMembers> {
        debug!("Listing members in role {} for guild {}", role_id, guild_id);
        let query = pager.to_query_params();
        let path = resource::guild_role_members(guild_id, role_id);
        let response = if query.is_empty() {
            self.http.get(self.token(), &path, None::<&()>).await?
        } else {
            self.http.get(self.token(), &path, Some(&query)).await?
        };
        Self::decode_json(response)
    }

    /// Deletes a guild member.
    pub async fn delete_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
        options: &MemberDeleteOptions,
    ) -> Result<()> {
        debug!("Deleting member {} in guild {}", user_id, guild_id);
        let path = resource::guild_member(guild_id, user_id);
        self.request_json::<serde_json::Value, _, _>(
            Method::DELETE,
            &path,
            None::<&()>,
            Some(options),
        )
        .await?;
        Ok(())
    }

    /// Fetches guild message push settings.
    pub async fn get_message_setting(&self, guild_id: &str) -> Result<MessageSetting> {
        debug!("Getting message setting for guild {}", guild_id);
        let path = resource::message_setting(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }
}
