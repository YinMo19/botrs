use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn create_channel_announce(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        self.api
            .create_channel_announce(&self.token, channel_id, message_id)
            .await
    }

    /// Deletes a channel announcement.

    pub async fn delete_channel_announce(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.api
            .delete_channel_announce(&self.token, channel_id, message_id)
            .await
    }

    /// Clears all channel announcements.

    pub async fn clean_channel_announces(&self, channel_id: &str) -> Result<()> {
        self.api
            .clean_channel_announces(&self.token, channel_id)
            .await
    }

    /// Creates a message-type guild announcement.

    pub async fn create_guild_announce(
        &self,
        guild_id: &str,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        self.api
            .create_guild_announce(&self.token, guild_id, channel_id, message_id)
            .await
    }

    /// Creates a recommended channel guild announcement.

    pub async fn create_guild_recommend_announce(
        &self,
        guild_id: &str,
        announces_type: AnnouncesType,
        recommend_channels: Vec<RecommendChannel>,
    ) -> Result<Announce> {
        self.api
            .create_guild_recommend_announce(
                &self.token,
                guild_id,
                announces_type,
                recommend_channels,
            )
            .await
    }

    /// Deletes a guild announcement.

    pub async fn delete_guild_announce(&self, guild_id: &str, message_id: &str) -> Result<()> {
        self.api
            .delete_guild_announce(&self.token, guild_id, message_id)
            .await
    }

    /// Clears all guild announcements.

    pub async fn clean_guild_announces(&self, guild_id: &str) -> Result<()> {
        self.api.clean_guild_announces(&self.token, guild_id).await
    }
}
