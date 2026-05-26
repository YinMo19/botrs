use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel> {
        self.api.get_channel(&self.token, channel_id).await
    }

    /// Gets message information.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    ///
    /// # Returns
    ///
    /// The message.

    pub async fn get_channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        self.api.get_channels(&self.token, guild_id).await
    }

    /// Creates a new channel in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `name` - Channel name
    /// * `channel_type` - Channel type
    /// * `sub_type` - Channel sub type
    /// * `position` - Channel position
    /// * `parent_id` - Parent channel ID for category channels
    /// * `private_type` - Private type (0=public, 1=private, 2=voice private)
    /// * `private_user_ids` - List of user IDs for private channels
    /// * `speak_permission` - Speak permission (0=invalid, 1=all members, 2=members with role)
    /// * `application_id` - Application ID for application channels
    ///
    /// # Returns
    ///
    /// The created channel.

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

    /// Creates a new channel from a channel value object.

    pub async fn post_channel(
        &self,
        guild_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        self.api.post_channel(&self.token, guild_id, value).await
    }

    /// Creates a private channel.

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

    /// Gets guild roles.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// List of guild roles.

    pub async fn get_channel_user_permissions(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        self.api
            .get_channel_user_permissions(&self.token, channel_id, user_id)
            .await
    }

    /// Updates channel permissions for a user.

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

    /// Updates channel permissions for a user.

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

    /// Gets channel permissions for a role.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Channel permissions for the role.

    pub async fn get_channel_role_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelRolesPermissions> {
        self.api
            .get_channel_role_permissions(&self.token, channel_id, role_id)
            .await
    }

    /// Updates channel permissions for a role.

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

    /// Updates channel permissions for a role.

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

    /// Updates a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `name` - Optional new name
    /// * `position` - Optional new position
    /// * `parent_id` - Optional new parent ID
    /// * `private_type` - Optional new private type
    /// * `speak_permission` - Optional new speak permission
    ///
    /// # Returns
    ///
    /// The updated channel.

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

    /// Updates a channel from a channel value object.

    pub async fn patch_channel(
        &self,
        channel_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        self.api.patch_channel(&self.token, channel_id, value).await
    }

    /// Deletes a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// The deleted channel.

    pub async fn delete_channel(&self, channel_id: &str) -> Result<Channel> {
        self.api.delete_channel(&self.token, channel_id).await
    }

    /// Lists members in a voice channel.

    pub async fn list_voice_channel_members(&self, channel_id: &str) -> Result<Vec<GuildMember>> {
        self.api
            .list_voice_channel_members(&self.token, channel_id)
            .await
    }
}
