use super::{BotApi, resource};
use crate::error::{BotError, Result};
use crate::models::permission::{
    ChannelPermissions, ChannelRolePermissions, UpdateChannelPermissions,
};
use tracing::debug;

impl BotApi {
    /// Gets member permissions in a channel.
    pub async fn get_channel_permissions(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        debug!(
            "Getting permissions for member {} in channel {}",
            user_id, channel_id
        );
        let path = resource::channel_permissions(channel_id, user_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Updates member permissions in a channel.
    pub async fn update_channel_permissions(
        &self,
        channel_id: &str,
        user_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        validate_permission_update(permissions)?;
        debug!(
            "Updating permissions for member {} in channel {}",
            user_id, channel_id
        );
        let path = resource::channel_permissions(channel_id, user_id);
        self.http
            .put(self.token(), &path, None::<&()>, Some(permissions))
            .await?;
        Ok(())
    }

    /// Gets role permissions in a channel.
    pub async fn get_channel_role_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelRolePermissions> {
        debug!(
            "Getting permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = resource::channel_role_permissions(channel_id, role_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Updates role permissions in a channel.
    pub async fn update_channel_role_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        validate_permission_update(permissions)?;
        debug!(
            "Updating permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = resource::channel_role_permissions(channel_id, role_id);
        self.http
            .put(self.token(), &path, None::<&()>, Some(permissions))
            .await?;
        Ok(())
    }
}

fn validate_permission_update(permissions: &UpdateChannelPermissions) -> Result<()> {
    for (name, value) in [
        ("add", permissions.add.as_deref()),
        ("remove", permissions.remove.as_deref()),
    ] {
        if let Some(value) = value.filter(|value| !value.is_empty()) {
            value.parse::<u64>().map_err(|err| {
                BotError::invalid_data(format!("invalid parameter {name}: {err}"))
            })?;
        }
    }
    Ok(())
}
