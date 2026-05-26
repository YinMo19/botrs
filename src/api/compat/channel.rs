use crate::api::BotApi;
use crate::error::Result;
use crate::models::channel::{Channel, ChannelValueObject};
use crate::models::guild::Member;

impl BotApi {
    /// Channel lookup API.
    #[allow(non_snake_case)]
    pub async fn Channel(&self, channel_id: &str) -> Result<Channel> {
        self.get_channel(self.token_required()?, channel_id).await
    }

    /// Channel list API.
    #[allow(non_snake_case)]
    pub async fn Channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        self.get_channels(self.token_required()?, guild_id).await
    }

    /// Channel creation API.
    #[allow(non_snake_case)]
    pub async fn PostChannel(&self, guild_id: &str, value: &ChannelValueObject) -> Result<Channel> {
        self.post_channel(self.token_required()?, guild_id, value)
            .await
    }

    /// Channel update API.
    #[allow(non_snake_case)]
    pub async fn PatchChannel(
        &self,
        channel_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        self.patch_channel(self.token_required()?, channel_id, value)
            .await
    }

    /// Channel delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteChannel(&self, channel_id: &str) -> Result<()> {
        self.delete_channel(self.token_required()?, channel_id)
            .await?;
        Ok(())
    }

    /// Private channel creation API.
    #[allow(non_snake_case)]
    pub async fn CreatePrivateChannel(
        &self,
        guild_id: &str,
        value: &ChannelValueObject,
        user_ids: Vec<String>,
    ) -> Result<Channel> {
        self.create_private_channel(self.token_required()?, guild_id, value, user_ids)
            .await
    }

    /// Voice channel member list API.
    #[allow(non_snake_case)]
    pub async fn ListVoiceChannelMembers(&self, channel_id: &str) -> Result<Vec<Member>> {
        self.list_voice_channel_members(self.token_required()?, channel_id)
            .await
    }
}
