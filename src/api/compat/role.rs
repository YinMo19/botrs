use crate::api::BotApi;
use crate::error::Result;
use crate::models::guild::{GuildRole, GuildRoles, UpdateResult};

impl BotApi {
    /// Role list API.
    #[allow(non_snake_case)]
    pub async fn Roles(&self, guild_id: &str) -> Result<GuildRoles> {
        self.get_guild_roles(self.token_required()?, guild_id).await
    }

    /// Role creation API.
    #[allow(non_snake_case)]
    pub async fn PostRole(&self, guild_id: &str, role: &GuildRole) -> Result<UpdateResult> {
        self.create_guild_role_with_update(self.token_required()?, guild_id, role.clone())
            .await
    }

    /// Role update API.
    #[allow(non_snake_case)]
    pub async fn PatchRole(
        &self,
        guild_id: &str,
        role_id: &str,
        role: &GuildRole,
    ) -> Result<UpdateResult> {
        self.update_guild_role_with_update(self.token_required()?, guild_id, role_id, role.clone())
            .await
    }

    /// Role delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteRole(&self, guild_id: &str, role_id: &str) -> Result<()> {
        self.delete_guild_role(self.token_required()?, guild_id, role_id)
            .await
    }
}
