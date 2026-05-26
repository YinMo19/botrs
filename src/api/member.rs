use super::{BotApi, resource};
use crate::error::Result;
use crate::models::guild::{MemberAddRoleBody, UpdateGuildMute, UpdateGuildMuteResponse};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Adds a role to a guild member, optionally scoped to a channel.
    pub async fn create_guild_role_member(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Adding user {} to role {} in guild {}",
            user_id, role_id, guild_id
        );

        let body = if let Some(channel_id) = channel_id {
            MemberAddRoleBody::with_channel_id(channel_id)
        } else {
            MemberAddRoleBody::new()
        };

        self.member_add_role(token, guild_id, role_id, user_id, &body)
            .await?;
        Ok(())
    }

    /// Adds a role to a guild member using a structured body.
    pub async fn member_add_role(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        body: &MemberAddRoleBody,
    ) -> Result<()> {
        let path = resource::guild_member_role(guild_id, user_id, role_id);
        self.http.put(token, &path, None::<&()>, Some(body)).await?;
        Ok(())
    }

    /// Removes a role from a guild member, optionally scoped to a channel.
    pub async fn delete_guild_role_member(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Removing user {} from role {} in guild {}",
            user_id, role_id, guild_id
        );

        let body = if let Some(channel_id) = channel_id {
            MemberAddRoleBody::with_channel_id(channel_id)
        } else {
            MemberAddRoleBody::new()
        };

        self.member_delete_role(token, guild_id, role_id, user_id, &body)
            .await?;
        Ok(())
    }

    /// Removes a role from a guild member using a structured body.
    pub async fn member_delete_role(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        body: &MemberAddRoleBody,
    ) -> Result<()> {
        let path = resource::guild_member_role(guild_id, user_id, role_id);
        self.http
            .delete_with_body(token, &path, None::<&()>, Some(body))
            .await?;
        Ok(())
    }

    /// Mutes one guild member.
    ///
    /// The platform accepts either an end timestamp or a duration in seconds.
    pub async fn mute_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting member {} in guild {}", user_id, guild_id);

        let body = UpdateGuildMute::new(mute_end_timestamp, mute_seconds);

        let path = resource::guild_member_mute(guild_id, user_id);
        self.http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Mutes several guild members with inline parameters.
    pub async fn mute_multi_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_ids: Vec<String>,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<UpdateGuildMuteResponse> {
        if user_ids.is_empty() {
            return Err(crate::error::BotError::invalid_data("no user id param"));
        }

        let body = UpdateGuildMute::new_multi(user_ids, mute_end_timestamp, mute_seconds);
        self.multi_member_mute(token, guild_id, &body).await
    }

    /// Cancels mute for several guild members.
    pub async fn cancel_mute_multi_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_ids: Vec<String>,
    ) -> Result<UpdateGuildMuteResponse> {
        if user_ids.is_empty() {
            return Err(crate::error::BotError::invalid_data("no user id param"));
        }

        let body = UpdateGuildMute::cancel_multi(user_ids);
        self.multi_member_mute(token, guild_id, &body).await
    }

    /// Mutes several guild members using a structured request body.
    pub async fn multi_member_mute(
        &self,
        token: &Token,
        guild_id: &str,
        mute: &UpdateGuildMute,
    ) -> Result<UpdateGuildMuteResponse> {
        if mute.user_ids.is_empty() {
            return Err(crate::error::BotError::invalid_data("no user id param"));
        }

        debug!("Muting multiple members in guild {}", guild_id);
        let path = resource::guild_mute(guild_id);
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(mute))
            .await?;
        Self::decode_json(response)
    }
}
