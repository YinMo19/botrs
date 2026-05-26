use crate::api::BotApi;
use crate::api::resource;
use crate::error::Result;
use crate::models::announce::{Announce, ChannelAnnouncesToCreate, GuildAnnouncesToCreate};
use reqwest::Method;

impl BotApi {
    /// Channel announce creation API.
    #[allow(non_snake_case)]
    pub async fn CreateChannelAnnounces(
        &self,
        channel_id: &str,
        announce: &ChannelAnnouncesToCreate,
    ) -> Result<Announce> {
        self.create_channel_announce(self.token_required()?, channel_id, &announce.message_id)
            .await
    }

    /// Channel announce delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteChannelAnnounces(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete_channel_announce(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Channel announces clean API.
    #[allow(non_snake_case)]
    pub async fn CleanChannelAnnounces(&self, channel_id: &str) -> Result<()> {
        self.clean_channel_announces(self.token_required()?, channel_id)
            .await
    }

    /// Guild announce creation API.
    #[allow(non_snake_case)]
    pub async fn CreateGuildAnnounces(
        &self,
        guild_id: &str,
        announce: &GuildAnnouncesToCreate,
    ) -> Result<Announce> {
        self.request_json(
            self.token_required()?,
            Method::POST,
            &resource::guild_announces(guild_id),
            None::<&()>,
            Some(announce),
        )
        .await
    }

    /// Guild announce delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteGuildAnnounces(&self, guild_id: &str, message_id: &str) -> Result<()> {
        self.delete_guild_announce(self.token_required()?, guild_id, message_id)
            .await
    }

    /// Guild announces clean API.
    #[allow(non_snake_case)]
    pub async fn CleanGuildAnnounces(&self, guild_id: &str) -> Result<()> {
        self.clean_guild_announces(self.token_required()?, guild_id)
            .await
    }
}
