use super::{BotApi, resource};
use crate::error::Result;
use crate::models::guild::{GuildRole, GuildRoles, UpdateResult, UpdateRole};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    // Guild Role APIs

    /// Lists roles configured in a guild.
    pub async fn get_guild_roles(&self, token: &Token, guild_id: &str) -> Result<GuildRoles> {
        debug!("Getting guild roles for {}", guild_id);
        let path = resource::guild_roles(guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a guild role from a structured role body.
    pub async fn create_guild_role_with_update(
        &self,
        token: &Token,
        guild_id: &str,
        role: GuildRole,
    ) -> Result<UpdateResult> {
        debug!("Creating guild role in {}", guild_id);
        let body = UpdateRole::new(guild_id, role);
        let path = resource::guild_roles(guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    pub async fn create_guild_role(
        &self,
        token: &Token,
        guild_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<GuildRole> {
        let role = GuildRole {
            name: name.unwrap_or_default().to_string(),
            color: color.unwrap_or_default(),
            hoist: hoist.map(u32::from).unwrap_or_default(),
            ..Default::default()
        };
        let result = self
            .create_guild_role_with_update(token, guild_id, role)
            .await?;
        Ok(result.role.unwrap_or_default())
    }

    /// Updates a guild role from a structured role body.
    pub async fn update_guild_role_with_update(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        role: GuildRole,
    ) -> Result<UpdateResult> {
        debug!("Updating guild role {} in {}", role_id, guild_id);
        let body = UpdateRole::new(guild_id, role);
        let path = resource::guild_role(guild_id, role_id);
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    pub async fn update_guild_role(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<GuildRole> {
        let role = GuildRole {
            id: role_id.to_string(),
            name: name.unwrap_or_default().to_string(),
            color: color.unwrap_or_default(),
            hoist: hoist.map(u32::from).unwrap_or_default(),
            ..Default::default()
        };
        let result = self
            .update_guild_role_with_update(token, guild_id, role_id, role)
            .await?;
        Ok(result.role.unwrap_or_default())
    }

    /// Deletes a guild role.
    pub async fn delete_guild_role(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
    ) -> Result<()> {
        debug!("Deleting guild role {} in {}", role_id, guild_id);
        let path = resource::guild_role(guild_id, role_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }
}
