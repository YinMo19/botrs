use crate::api::BotApi;
use crate::error::Result;
use crate::models::channel::{
    ChannelPermissions, ChannelRolesPermissions, UpdateChannelPermissions,
};

impl BotApi {
    /// Channel permissions API.
    #[allow(non_snake_case)]
    pub async fn ChannelPermissions(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        self.get_channel_user_permissions(self.token_required()?, channel_id, user_id)
            .await
    }

    /// Channel permissions update API.
    #[allow(non_snake_case)]
    pub async fn PutChannelPermissions(
        &self,
        channel_id: &str,
        user_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        self.put_channel_permissions(self.token_required()?, channel_id, user_id, permissions)
            .await
    }

    /// Channel role permissions API.
    #[allow(non_snake_case)]
    pub async fn ChannelRolesPermissions(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelRolesPermissions> {
        self.get_channel_role_permissions(self.token_required()?, channel_id, role_id)
            .await
    }

    /// Channel role permissions update API.
    #[allow(non_snake_case)]
    pub async fn PutChannelRolesPermissions(
        &self,
        channel_id: &str,
        role_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        self.put_channel_roles_permissions(self.token_required()?, channel_id, role_id, permissions)
            .await
    }
}
