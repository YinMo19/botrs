use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel> {
        self.api.get_channel(&self.token, channel_id).await
    }

    /// Lists channels in a guild.
    pub async fn get_channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        self.api.get_channels(&self.token, guild_id).await
    }

    /// Creates a guild channel from inline fields.
    pub async fn create_channel(
        &self,
        guild_id: &str,
        name: &str,
        channel_type: ChannelType,
        sub_type: ChannelSubType,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        private_user_ids: Option<Vec<String>>,
        speak_permission: Option<u32>,
        application_id: Option<&str>,
    ) -> Result<Channel> {
        self.api
            .create_channel(
                &self.token,
                guild_id,
                name,
                channel_type,
                sub_type,
                position,
                parent_id,
                private_type,
                private_user_ids,
                speak_permission,
                application_id,
            )
            .await
    }

    /// Creates a guild channel from a structured channel body.
    pub async fn post_channel(
        &self,
        guild_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        self.api.post_channel(&self.token, guild_id, value).await
    }

    /// Creates a private channel and grants access to the supplied users.
    pub async fn create_private_channel(
        &self,
        guild_id: &str,
        value: &ChannelValueObject,
        user_ids: Vec<String>,
    ) -> Result<Channel> {
        self.api
            .create_private_channel(&self.token, guild_id, value, user_ids)
            .await
    }

    /// Fetches channel permissions for one user.
    pub async fn get_channel_user_permissions(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        self.api
            .get_channel_user_permissions(&self.token, channel_id, user_id)
            .await
    }

    /// Updates channel permissions for one user using a structured body.
    pub async fn put_channel_permissions(
        &self,
        channel_id: &str,
        user_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        self.api
            .put_channel_permissions(&self.token, channel_id, user_id, permissions)
            .await
    }

    /// Updates channel permissions for one user using add/remove bitsets.
    pub async fn update_channel_user_permissions(
        &self,
        channel_id: &str,
        user_id: &str,
        add: Option<&str>,
        remove: Option<&str>,
    ) -> Result<()> {
        self.api
            .update_channel_user_permissions(&self.token, channel_id, user_id, add, remove)
            .await
    }

    /// Fetches channel permissions for one role.
    pub async fn get_channel_role_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelRolesPermissions> {
        self.api
            .get_channel_role_permissions(&self.token, channel_id, role_id)
            .await
    }

    /// Updates channel permissions for one role using a structured body.
    pub async fn put_channel_roles_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        self.api
            .put_channel_roles_permissions(&self.token, channel_id, role_id, permissions)
            .await
    }

    /// Updates channel permissions for one role using add/remove bitsets.
    pub async fn update_channel_role_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
        add: Option<&str>,
        remove: Option<&str>,
    ) -> Result<()> {
        self.api
            .update_channel_role_permissions(&self.token, channel_id, role_id, add, remove)
            .await
    }

    /// Updates a channel from inline fields.
    pub async fn update_channel(
        &self,
        channel_id: &str,
        name: Option<&str>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        speak_permission: Option<u32>,
    ) -> Result<Channel> {
        self.api
            .update_channel(
                &self.token,
                channel_id,
                name,
                position,
                parent_id,
                private_type,
                speak_permission,
            )
            .await
    }

    /// Updates a channel from a structured channel body.
    pub async fn patch_channel(
        &self,
        channel_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        self.api.patch_channel(&self.token, channel_id, value).await
    }

    /// Deletes a channel.
    pub async fn delete_channel(&self, channel_id: &str) -> Result<Option<Channel>> {
        self.api.delete_channel(&self.token, channel_id).await
    }

    /// Lists members currently present in a voice channel.
    pub async fn list_voice_channel_members(&self, channel_id: &str) -> Result<Vec<GuildMember>> {
        self.api
            .list_voice_channel_members(&self.token, channel_id)
            .await
    }
}
