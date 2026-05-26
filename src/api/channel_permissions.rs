use super::{BotApi, resource};
use crate::error::Result;
use crate::models::channel::{
    ChannelPermissions, ChannelRolesPermissions, UpdateChannelPermissions,
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Gets channel permissions for a user.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Channel permissions.
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

    /// Updates channel permissions for a user.
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

    /// Updates channel permissions for a user.
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

    /// Gets channel permissions for a role.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Channel permissions.
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

    /// Updates channel permissions for a role.
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

    /// Updates channel permissions for a role.
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
