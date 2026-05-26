use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    channel::{
        Channel, ChannelSubType, ChannelType, ChannelValueObject, PrivateType, SpeakPermission,
    },
    guild::Member,
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    // Channel APIs

    /// Fetches one channel by ID.
    pub async fn get_channel(&self, token: &Token, channel_id: &str) -> Result<Channel> {
        debug!("Getting channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists channels in a guild.
    pub async fn get_channels(&self, token: &Token, guild_id: &str) -> Result<Vec<Channel>> {
        debug!("Getting channels for guild {}", guild_id);
        let path = resource::guild_channels(guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a guild channel from a structured channel body.
    pub async fn post_channel(
        &self,
        token: &Token,
        guild_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        debug!("Creating channel in guild {}", guild_id);
        let path = resource::guild_channels(guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(value))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a guild channel from inline fields.
    pub async fn create_channel(
        &self,
        token: &Token,
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
        let value = ChannelValueObject {
            name: Some(name.to_string()),
            channel_type: Some(channel_type),
            sub_type: Some(sub_type),
            position: position.map(i64::from),
            parent_id: parent_id.map(String::from),
            private_type: private_type.map(|value| PrivateType::from(value as u8)),
            private_user_ids,
            speak_permission: speak_permission.map(|value| SpeakPermission::from(value as u8)),
            application_id: application_id.map(String::from),
            ..Default::default()
        };

        self.post_channel(token, guild_id, &value).await
    }

    /// Creates a private channel.
    ///
    /// If `user_ids` is empty, the channel is visible to admins and members.
    /// If `user_ids` is not empty, the channel is created as admin-only and the
    /// members are added through `private_user_ids`.
    pub async fn create_private_channel(
        &self,
        token: &Token,
        guild_id: &str,
        value: &ChannelValueObject,
        user_ids: Vec<String>,
    ) -> Result<Channel> {
        let mut value = value.clone();
        value.private_type = Some(PrivateType::AdminAndMember);
        if !user_ids.is_empty() {
            value.private_user_ids = Some(user_ids);
            value.private_type = Some(PrivateType::OnlyAdmin);
        }
        self.post_channel(token, guild_id, &value).await
    }

    /// Updates a channel from inline fields.
    pub async fn update_channel(
        &self,
        token: &Token,
        channel_id: &str,
        name: Option<&str>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        speak_permission: Option<u32>,
    ) -> Result<Channel> {
        let value = ChannelValueObject {
            name: name.map(String::from),
            position: position.map(i64::from),
            parent_id: parent_id.map(String::from),
            private_type: private_type.map(|value| PrivateType::from(value as u8)),
            speak_permission: speak_permission.map(|value| SpeakPermission::from(value as u8)),
            ..Default::default()
        };

        self.patch_channel(token, channel_id, &value).await
    }

    /// Updates a channel from a structured channel body.
    pub async fn patch_channel(
        &self,
        token: &Token,
        channel_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        debug!("Updating channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(value))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a channel and returns the deleted channel model.
    pub async fn delete_channel(&self, token: &Token, channel_id: &str) -> Result<Channel> {
        debug!("Deleting channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self.http.delete(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists members currently present in a voice channel.
    pub async fn list_voice_channel_members(
        &self,
        token: &Token,
        channel_id: &str,
    ) -> Result<Vec<Member>> {
        debug!("Listing voice channel members for channel {}", channel_id);
        let path = resource::voice_channel_members(channel_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }
}
