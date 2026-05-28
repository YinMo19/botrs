use super::{BotApi, resource};
use crate::error::Result;
use crate::models::{
    channel::{Channel, PrivateType},
    guild::Member,
    serde_helpers::is_default,
};
use reqwest::Method;
use serde::Serialize;
use tracing::debug;

#[derive(Serialize)]
struct ChannelValueBody {
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    #[serde(rename = "type", skip_serializing_if = "is_default")]
    channel_type: crate::models::channel::ChannelType,
    #[serde(skip_serializing_if = "is_default")]
    position: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    parent_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    owner_id: String,
    #[serde(skip_serializing_if = "is_default")]
    sub_type: crate::models::channel::ChannelSubType,
    #[serde(skip_serializing_if = "is_default")]
    private_type: PrivateType,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    private_user_ids: Vec<String>,
    #[serde(skip_serializing_if = "is_default")]
    speak_permission: crate::models::channel::SpeakPermission,
    #[serde(skip_serializing_if = "String::is_empty")]
    application_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    permissions: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    op_user_id: String,
}

impl From<&Channel> for ChannelValueBody {
    fn from(channel: &Channel) -> Self {
        Self {
            name: channel.name.clone(),
            channel_type: channel.channel_type,
            position: channel.position,
            parent_id: channel.parent_id.clone(),
            owner_id: channel.owner_id.clone(),
            sub_type: channel.sub_type,
            private_type: channel.private_type,
            private_user_ids: channel.private_user_ids.clone(),
            speak_permission: channel.speak_permission,
            application_id: channel.application_id.clone(),
            permissions: channel.permissions.clone(),
            op_user_id: channel.op_user_id.clone(),
        }
    }
}

impl BotApi {
    /// Fetches one channel.
    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel> {
        debug!("Getting channel {}", channel_id);
        let path = resource::channel(channel_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Lists channels in a guild.
    pub async fn list_channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        debug!("Listing channels in guild {}", guild_id);
        let path = resource::guild_channels(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates a channel.
    pub async fn create_channel(&self, guild_id: &str, channel: &Channel) -> Result<Channel> {
        debug!("Creating channel in guild {}", guild_id);
        let body = ChannelValueBody::from(channel);
        let path = resource::guild_channels(guild_id);
        self.request_json(Method::POST, &path, None::<&()>, Some(&body))
            .await
    }

    /// Updates a channel.
    pub async fn update_channel(&self, channel_id: &str, channel: &Channel) -> Result<Channel> {
        debug!("Updating channel {}", channel_id);
        let body = ChannelValueBody::from(channel);
        let path = resource::channel(channel_id);
        self.request_json(Method::PATCH, &path, None::<&()>, Some(&body))
            .await
    }

    /// Deletes a channel.
    pub async fn delete_channel(&self, channel_id: &str) -> Result<()> {
        debug!("Deleting channel {}", channel_id);
        let path = resource::channel(channel_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
    }

    /// Creates a private channel.
    pub async fn create_private_channel(
        &self,
        guild_id: &str,
        channel: &Channel,
        user_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Channel> {
        let mut channel = channel.clone();
        let user_ids = user_ids.into_iter().map(Into::into).collect::<Vec<_>>();
        channel.private_type = if user_ids.is_empty() {
            PrivateType::AdminAndMember
        } else {
            channel.private_user_ids = user_ids;
            PrivateType::OnlyAdmin
        };
        self.create_channel(guild_id, &channel).await
    }

    /// Lists members currently in a voice channel.
    pub async fn list_voice_channel_members(&self, channel_id: &str) -> Result<Vec<Member>> {
        debug!("Listing voice channel members in channel {}", channel_id);
        let path = resource::voice_channel_members(channel_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::channel::{ChannelSubType, ChannelType, SpeakPermission};

    #[test]
    fn channel_value_body_matches_platform_value_object_shape() {
        let channel = Channel {
            id: "channel-1".to_string(),
            guild_id: "guild-1".to_string(),
            name: "general".to_string(),
            channel_type: ChannelType::Text,
            sub_type: ChannelSubType::Notice,
            parent_id: "parent-1".to_string(),
            private_type: PrivateType::OnlyAdmin,
            private_user_ids: vec!["user-1".to_string()],
            speak_permission: SpeakPermission::Public,
            ..Default::default()
        };

        let value = serde_json::to_value(ChannelValueBody::from(&channel)).unwrap();

        assert!(value.get("id").is_none());
        assert!(value.get("guild_id").is_none());
        assert_eq!(value["name"], "general");
        assert!(value.get("type").is_none());
        assert_eq!(value["sub_type"], 1);
        assert_eq!(value["parent_id"], "parent-1");
        assert_eq!(value["private_type"], 1);
        assert_eq!(value["private_user_ids"], serde_json::json!(["user-1"]));
        assert_eq!(value["speak_permission"], 1);
    }
}
