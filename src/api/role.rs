use super::{BotApi, resource};
use crate::error::Result;
use crate::models::role::{
    DEFAULT_ROLE_COLOR, GuildRoles, MemberRoleParams, Role, UpdateRoleBody, UpdateRoleFilter,
    UpdateRoleResult,
};
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Lists guild roles.
    pub async fn list_roles(&self, guild_id: &str) -> Result<GuildRoles> {
        debug!("Listing roles in guild {}", guild_id);
        let path = resource::guild_roles(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a guild role.
    pub async fn create_role(&self, guild_id: &str, role: &Role) -> Result<UpdateRoleResult> {
        debug!("Creating role in guild {}", guild_id);
        let body = role_body(guild_id, role);
        let path = resource::guild_roles(guild_id);
        self.request_json(Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Updates a guild role.
    pub async fn update_role(
        &self,
        guild_id: &str,
        role_id: &str,
        role: &Role,
    ) -> Result<UpdateRoleResult> {
        debug!("Updating role {} in guild {}", role_id, guild_id);
        let body = role_body(guild_id, role);
        let path = resource::guild_role(guild_id, role_id);
        self.request_json(Method::PATCH, &path, None::<&()>, Some(&body))
            .await
    }

    /// Deletes a guild role.
    pub async fn delete_role(&self, guild_id: &str, role_id: &str) -> Result<()> {
        debug!("Deleting role {} in guild {}", role_id, guild_id);
        let path = resource::guild_role(guild_id, role_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
    }

    /// Adds a role to a member.
    pub async fn add_member_role(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        params: &MemberRoleParams,
    ) -> Result<()> {
        debug!(
            "Adding role {} to member {} in guild {}",
            role_id, user_id, guild_id
        );
        let path = resource::member_role(guild_id, user_id, role_id);
        self.http
            .put(self.token(), &path, None::<&()>, Some(params))
            .await?;
        Ok(())
    }

    /// Removes a role from a member.
    pub async fn delete_member_role(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        params: &MemberRoleParams,
    ) -> Result<()> {
        debug!(
            "Deleting role {} from member {} in guild {}",
            role_id, user_id, guild_id
        );
        let path = resource::member_role(guild_id, user_id, role_id);
        self.request_json::<serde_json::Value, _, _>(
            Method::DELETE,
            &path,
            None::<&()>,
            Some(params),
        )
        .await?;
        Ok(())
    }
}

fn role_body(guild_id: &str, role: &Role) -> UpdateRoleBody {
    let mut role = role.clone();
    if role.color == 0 {
        role.color = DEFAULT_ROLE_COLOR;
    }
    UpdateRoleBody {
        guild_id: guild_id.to_string(),
        filter: UpdateRoleFilter::default(),
        role,
    }
}
