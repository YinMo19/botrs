use super::{BotApi, resource};
use crate::error::Result;
use crate::models::guild::{
    Guild, GuildMembersPager, GuildRoleMembers, GuildRoleMembersPager, Member, MemberDeleteOptions,
    UpdateGuildMute, normalize_delete_history_msg_days,
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    // Guild-related APIs

    /// Fetches one guild by ID.
    pub async fn get_guild(&self, token: &Token, guild_id: &str) -> Result<Guild> {
        debug!("Getting guild {}", guild_id);
        let path = resource::guild(guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    // Member APIs

    /// Fetches one guild member.
    pub async fn get_guild_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
    ) -> Result<Member> {
        debug!("Getting guild member {} in {}", user_id, guild_id);
        let path = resource::guild_member(guild_id, user_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists guild members using inline pagination parameters.
    pub async fn get_guild_members(
        &self,
        token: &Token,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Member>> {
        let pager = GuildMembersPager::new(after.unwrap_or("0"), limit.unwrap_or(1).to_string());
        self.get_guild_members_with_pager(token, guild_id, &pager)
            .await
    }

    /// Lists guild members using a pre-built pager.
    pub async fn get_guild_members_with_pager(
        &self,
        token: &Token,
        guild_id: &str,
        pager: &GuildMembersPager,
    ) -> Result<Vec<Member>> {
        debug!(
            "Getting guild members for {} with pager {:?}",
            guild_id, pager
        );
        let path = resource::guild_members(guild_id);
        let response = self.http.get(token, &path, Some(pager)).await?;
        Self::decode_json(response)
    }

    /// Lists members that have a guild role.
    pub async fn get_guild_role_members(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        start_index: Option<&str>,
        limit: Option<u32>,
    ) -> Result<GuildRoleMembers> {
        let pager =
            GuildRoleMembersPager::new(start_index.unwrap_or("0"), limit.unwrap_or(1).to_string());
        self.get_guild_role_members_with_pager(token, guild_id, role_id, &pager)
            .await
    }

    /// Lists members that have a guild role using a pre-built pager.
    pub async fn get_guild_role_members_with_pager(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        pager: &GuildRoleMembersPager,
    ) -> Result<GuildRoleMembers> {
        debug!(
            "Getting role {} members for guild {} with pager {:?}",
            role_id, guild_id, pager
        );
        let path = resource::guild_role_members(guild_id, role_id);
        let response = self.http.get(token, &path, Some(pager)).await?;
        Self::decode_json(response)
    }

    /// Removes a member from a guild.
    ///
    /// `delete_history_msg_days` is normalized to the platform-supported values.
    pub async fn delete_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        add_blacklist: Option<bool>,
        delete_history_msg_days: Option<i32>,
    ) -> Result<()> {
        let options = MemberDeleteOptions {
            add_blacklist: add_blacklist.unwrap_or(false),
            delete_history_msg_days: normalize_delete_history_msg_days(
                delete_history_msg_days.unwrap_or(0),
            ),
        };

        self.delete_member_with_options(token, guild_id, user_id, &options)
            .await
    }

    /// Removes a member from a guild using explicit delete options.
    pub async fn delete_member_with_options(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        options: &MemberDeleteOptions,
    ) -> Result<()> {
        debug!("Deleting member {} from guild {}", user_id, guild_id);

        let path = resource::guild_member(guild_id, user_id);
        self.http
            .delete_with_body(token, &path, None::<&()>, Some(options))
            .await?;
        Ok(())
    }

    // Muting APIs

    /// Mutes every member in a guild.
    ///
    /// The platform accepts either an end timestamp or a duration in seconds.
    pub async fn mute_all(
        &self,
        token: &Token,
        guild_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting all members in guild {}", guild_id);

        let body = UpdateGuildMute::new(mute_end_timestamp, mute_seconds);

        let path = resource::guild_mute(guild_id);
        self.http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Cancels the guild-wide mute.
    pub async fn cancel_mute_all(&self, token: &Token, guild_id: &str) -> Result<()> {
        debug!("Canceling mute for all members in guild {}", guild_id);

        let body = UpdateGuildMute::cancel();

        let path = resource::guild_mute(guild_id);
        self.http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }
}
