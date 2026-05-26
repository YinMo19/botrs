use super::{BotApi, resource};
use crate::error::Result;
use crate::models::channel::{
    ChannelPermissions, ChannelRolesPermissions, UpdateChannelPermissions,
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Fetches channel permissions for one user.
    pub async fn get_channel_user_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        debug!(
            "Getting channel permissions for user {} in channel {}",
            user_id, channel_id
        );
        let path = resource::channel_member_permissions(channel_id, user_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Updates channel permissions for one user using a structured body.
    pub async fn put_channel_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        user_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        permissions.validate()?;
        debug!(
            "Updating channel permissions for user {} in channel {}",
            user_id, channel_id
        );
        let path = resource::channel_member_permissions(channel_id, user_id);
        self.http
            .put(token, &path, None::<&()>, Some(permissions))
            .await?;
        Ok(())
    }

    /// Updates channel permissions for one user using add/remove bitsets.
    pub async fn update_channel_user_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        user_id: &str,
        add: Option<&str>,
        remove: Option<&str>,
    ) -> Result<()> {
        let permissions = UpdateChannelPermissions::new(add, remove);
        self.put_channel_permissions(token, channel_id, user_id, &permissions)
            .await
    }

    /// Fetches channel permissions for one role.
    pub async fn get_channel_role_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelRolesPermissions> {
        debug!(
            "Getting channel permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = resource::channel_role_permissions(channel_id, role_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Updates channel permissions for one role using a structured body.
    pub async fn put_channel_roles_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        role_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        permissions.validate()?;
        debug!(
            "Updating channel permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = resource::channel_role_permissions(channel_id, role_id);
        self.http
            .put(token, &path, None::<&()>, Some(permissions))
            .await?;
        Ok(())
    }

    /// Updates channel permissions for one role using add/remove bitsets.
    pub async fn update_channel_role_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        role_id: &str,
        add: Option<&str>,
        remove: Option<&str>,
    ) -> Result<()> {
        let permissions = UpdateChannelPermissions::new(add, remove);
        self.put_channel_roles_permissions(token, channel_id, role_id, &permissions)
            .await
    }
}
