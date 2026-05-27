use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn get_guild(&self, guild_id: &str) -> Result<Guild> {
        self.api.get_guild(&self.token, guild_id).await
    }

    /// Fetches the guild message frequency settings.
    pub async fn get_message_setting(&self, guild_id: &str) -> Result<MessageSetting> {
        self.api.get_message_setting(&self.token, guild_id).await
    }

    /// Lists guilds visible to the current bot using inline pagination parameters.
    pub async fn get_guilds(
        &self,
        guild_id: Option<&str>,
        limit: Option<u32>,
        desc: Option<bool>,
    ) -> Result<Vec<Guild>> {
        self.api
            .get_guilds(&self.token, guild_id, limit, desc)
            .await
    }

    /// Lists guilds visible to the current bot using a pre-built pager.
    pub async fn get_guilds_with_pager(&self, pager: &GuildPager) -> Result<Vec<Guild>> {
        self.api.get_guilds_with_pager(&self.token, pager).await
    }

    /// Lists roles configured in a guild.
    pub async fn get_guild_roles(&self, guild_id: &str) -> Result<GuildRoles> {
        self.api.get_guild_roles(&self.token, guild_id).await
    }

    /// Creates a guild role from inline fields.
    pub async fn create_guild_role(
        &self,
        guild_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<UpdateResult> {
        self.api
            .create_guild_role(&self.token, guild_id, name, color, hoist)
            .await
    }

    /// Creates a guild role from a structured role body.
    pub async fn create_guild_role_with_update(
        &self,
        guild_id: &str,
        role: GuildRole,
    ) -> Result<UpdateResult> {
        self.api
            .create_guild_role_with_update(&self.token, guild_id, role)
            .await
    }

    /// Updates a guild role from inline fields.
    pub async fn update_guild_role(
        &self,
        guild_id: &str,
        role_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<UpdateResult> {
        self.api
            .update_guild_role(&self.token, guild_id, role_id, name, color, hoist)
            .await
    }

    /// Updates a guild role from a structured role body.
    pub async fn update_guild_role_with_update(
        &self,
        guild_id: &str,
        role_id: &str,
        role: GuildRole,
    ) -> Result<UpdateResult> {
        self.api
            .update_guild_role_with_update(&self.token, guild_id, role_id, role)
            .await
    }

    /// Deletes a guild role.
    pub async fn delete_guild_role(&self, guild_id: &str, role_id: &str) -> Result<()> {
        self.api
            .delete_guild_role(&self.token, guild_id, role_id)
            .await
    }

    /// Adds a role to a guild member, optionally scoped to a channel.
    pub async fn add_guild_role_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        self.api
            .create_guild_role_member(&self.token, guild_id, role_id, user_id, channel_id)
            .await
    }

    /// Adds a role to a guild member.
    pub async fn create_guild_role_member(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        self.api
            .create_guild_role_member(&self.token, guild_id, role_id, user_id, channel_id)
            .await
    }

    /// Adds a role to a guild member using a structured body.
    pub async fn member_add_role(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        body: &MemberAddRoleBody,
    ) -> Result<()> {
        self.api
            .member_add_role(&self.token, guild_id, role_id, user_id, body)
            .await
    }

    /// Removes a role from a guild member, optionally scoped to a channel.
    pub async fn remove_guild_role_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        self.api
            .delete_guild_role_member(&self.token, guild_id, role_id, user_id, channel_id)
            .await
    }

    /// Removes a role from a guild member.
    pub async fn delete_guild_role_member(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        self.api
            .delete_guild_role_member(&self.token, guild_id, role_id, user_id, channel_id)
            .await
    }

    /// Removes a role from a guild member using a structured body.
    pub async fn member_delete_role(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        body: &MemberAddRoleBody,
    ) -> Result<()> {
        self.api
            .member_delete_role(&self.token, guild_id, role_id, user_id, body)
            .await
    }

    /// Fetches one guild member.
    pub async fn get_guild_member(&self, guild_id: &str, user_id: &str) -> Result<GuildMember> {
        self.api
            .get_guild_member(&self.token, guild_id, user_id)
            .await
    }

    /// Lists members currently in a voice channel.
    pub async fn get_voice_members(&self, channel_id: &str) -> Result<Vec<GuildMember>> {
        self.api.get_voice_members(&self.token, channel_id).await
    }

    /// Lists guild members using inline pagination parameters.
    pub async fn get_guild_members(
        &self,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<GuildMember>> {
        self.api
            .get_guild_members(&self.token, guild_id, after, limit)
            .await
    }

    /// Lists guild members using a pre-built pager.
    pub async fn get_guild_members_with_pager(
        &self,
        guild_id: &str,
        pager: &GuildMembersPager,
    ) -> Result<Vec<GuildMember>> {
        self.api
            .get_guild_members_with_pager(&self.token, guild_id, pager)
            .await
    }

    /// Lists members that have a guild role.
    pub async fn get_guild_role_members(
        &self,
        guild_id: &str,
        role_id: &str,
        start_index: Option<&str>,
        limit: Option<u32>,
    ) -> Result<GuildRoleMembers> {
        self.api
            .get_guild_role_members(&self.token, guild_id, role_id, start_index, limit)
            .await
    }

    /// Lists members that have a guild role using a pre-built pager.
    pub async fn get_guild_role_members_with_pager(
        &self,
        guild_id: &str,
        role_id: &str,
        pager: &GuildRoleMembersPager,
    ) -> Result<GuildRoleMembers> {
        self.api
            .get_guild_role_members_with_pager(&self.token, guild_id, role_id, pager)
            .await
    }

    /// Removes a member from the guild.
    ///
    /// `delete_history_msg_days` follows the platform-supported day values.
    pub async fn kick_member(
        &self,
        guild_id: &str,
        user_id: &str,
        add_blacklist: Option<bool>,
        delete_history_msg_days: Option<i32>,
    ) -> Result<()> {
        self.api
            .delete_member(
                &self.token,
                guild_id,
                user_id,
                add_blacklist,
                delete_history_msg_days,
            )
            .await
    }

    /// Removes a member from the guild using explicit delete options.
    pub async fn delete_member_with_options(
        &self,
        guild_id: &str,
        user_id: &str,
        options: &MemberDeleteOptions,
    ) -> Result<()> {
        self.api
            .delete_member_with_options(&self.token, guild_id, user_id, options)
            .await
    }

    /// Mutes every member in a guild.
    ///
    /// The platform accepts either an end timestamp or a duration in seconds.
    pub async fn mute_all(
        &self,
        guild_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        self.api
            .mute_all(&self.token, guild_id, mute_end_timestamp, mute_seconds)
            .await
    }

    /// Cancels the guild-wide mute.
    pub async fn cancel_mute_all(&self, guild_id: &str) -> Result<()> {
        self.api.cancel_mute_all(&self.token, guild_id).await
    }

    /// Mutes one guild member.
    ///
    /// The platform accepts either an end timestamp or a duration in seconds.
    pub async fn mute_member(
        &self,
        guild_id: &str,
        user_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        self.api
            .mute_member(
                &self.token,
                guild_id,
                user_id,
                mute_end_timestamp,
                mute_seconds,
            )
            .await
    }

    /// Mutes several guild members with inline parameters.
    pub async fn mute_multi_member(
        &self,
        guild_id: &str,
        user_ids: Vec<String>,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<UpdateGuildMuteResponse> {
        self.api
            .mute_multi_member(
                &self.token,
                guild_id,
                user_ids,
                mute_end_timestamp,
                mute_seconds,
            )
            .await
    }

    /// Cancels mute for several guild members.
    pub async fn cancel_mute_multi_member(
        &self,
        guild_id: &str,
        user_ids: Vec<String>,
    ) -> Result<UpdateGuildMuteResponse> {
        self.api
            .cancel_mute_multi_member(&self.token, guild_id, user_ids)
            .await
    }

    /// Mutes several guild members using a structured request body.
    pub async fn multi_member_mute(
        &self,
        guild_id: &str,
        mute: &UpdateGuildMute,
    ) -> Result<UpdateGuildMuteResponse> {
        self.api
            .multi_member_mute(&self.token, guild_id, mute)
            .await
    }
}
