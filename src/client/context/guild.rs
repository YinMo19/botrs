use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn get_guild(&self, guild_id: &str) -> Result<Guild> {
        self.api.get_guild(&self.token, guild_id).await
    }

    /// Gets guild message frequency settings.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Message settings for the guild.

    pub async fn get_message_setting(&self, guild_id: &str) -> Result<MessageSetting> {
        self.api.get_message_setting(&self.token, guild_id).await
    }

    /// Gets channel information.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Channel information.

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

    /// Gets current-user guilds with a structured pager.

    pub async fn get_guilds_with_pager(&self, pager: &GuildPager) -> Result<Vec<Guild>> {
        self.api.get_guilds_with_pager(&self.token, pager).await
    }

    /// Gets channels in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// List of channels.

    pub async fn get_guild_roles(&self, guild_id: &str) -> Result<GuildRoles> {
        self.api.get_guild_roles(&self.token, guild_id).await
    }

    /// Creates a new guild role.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `name` - Role name
    /// * `color` - Role color (ARGB hex value converted to decimal)
    /// * `hoist` - Whether to display separately in member list (0=no, 1=yes)
    ///
    /// # Returns
    ///
    /// The created guild role.

    pub async fn create_guild_role(
        &self,
        guild_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<GuildRole> {
        self.api
            .create_guild_role(&self.token, guild_id, name, color, hoist)
            .await
    }

    /// Creates a guild role with a structured role body.

    pub async fn create_guild_role_with_update(
        &self,
        guild_id: &str,
        role: GuildRole,
    ) -> Result<UpdateResult> {
        self.api
            .create_guild_role_with_update(&self.token, guild_id, role)
            .await
    }

    /// Updates a guild role.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    /// * `name` - Role name
    /// * `color` - Role color (ARGB hex value converted to decimal)
    /// * `hoist` - Whether to display separately in member list (0=no, 1=yes)
    ///
    /// # Returns
    ///
    /// The updated guild role.

    pub async fn update_guild_role(
        &self,
        guild_id: &str,
        role_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<GuildRole> {
        self.api
            .update_guild_role(&self.token, guild_id, role_id, name, color, hoist)
            .await
    }

    /// Updates a guild role with a structured role body.

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
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

    pub async fn delete_guild_role(&self, guild_id: &str, role_id: &str) -> Result<()> {
        self.api
            .delete_guild_role(&self.token, guild_id, role_id)
            .await
    }

    /// Adds a role to a guild member.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `role_id` - The role ID
    /// * `channel_id` - Optional channel ID for channel-specific roles
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

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

    /// Adds a role to a guild member with a structured body.

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

    /// Removes a role from a guild member.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `role_id` - The role ID
    /// * `channel_id` - Optional channel ID for channel-specific roles
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

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

    /// Deletes a role from a guild member with a structured body.

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

    /// Gets guild member information.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Member information.

    pub async fn get_guild_member(&self, guild_id: &str, user_id: &str) -> Result<GuildMember> {
        self.api
            .get_guild_member(&self.token, guild_id, user_id)
            .await
    }

    /// Gets guild members list.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `after` - Optional user ID to get members after
    /// * `limit` - Number of members to return (1-400, default 1)
    ///
    /// # Returns
    ///
    /// List of members.

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

    /// Gets guild members list using a structured pager.

    pub async fn get_guild_members_with_pager(
        &self,
        guild_id: &str,
        pager: &GuildMembersPager,
    ) -> Result<Vec<GuildMember>> {
        self.api
            .get_guild_members_with_pager(&self.token, guild_id, pager)
            .await
    }

    /// Gets guild role members list.

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

    /// Gets guild role members using a pager.

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

    /// Kicks a member from the guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID to kick
    /// * `add_blacklist` - Whether to add user to blacklist
    /// * `delete_history_msg_days` - Days of message history to delete (3, 7, 15, 30)
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

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

    /// Kicks a member from the guild with explicit delete options.

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

    /// Updates audio control in a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `audio_control` - Audio control data
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

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

    /// Cancels mute for all members in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

    pub async fn cancel_mute_all(&self, guild_id: &str) -> Result<()> {
        self.api.cancel_mute_all(&self.token, guild_id).await
    }

    /// Mutes a specific member in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID to mute
    /// * `mute_end_timestamp` - Optional end timestamp
    /// * `mute_seconds` - Optional duration in seconds
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.

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

    /// Mutes multiple members in a guild.

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

    /// Cancels mute for multiple members in a guild.

    pub async fn cancel_mute_multi_member(
        &self,
        guild_id: &str,
        user_ids: Vec<String>,
    ) -> Result<UpdateGuildMuteResponse> {
        self.api
            .cancel_mute_multi_member(&self.token, guild_id, user_ids)
            .await
    }

    /// Mutes multiple members with a structured request body.

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
