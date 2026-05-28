use super::{BotApi, resource};
use crate::error::{BotError, Result};
use crate::models::mute::{GuildMute, GuildMuteResponse};
use reqwest::Method;
use tracing::debug;

impl BotApi {
    /// Mutes an entire guild.
    pub async fn mute_guild(&self, guild_id: &str, mute: &GuildMute) -> Result<()> {
        debug!("Muting guild {}", guild_id);
        let path = resource::guild_mute(guild_id);
        self.request_json::<serde_json::Value, _, _>(Method::PATCH, &path, None::<&()>, Some(mute))
            .await?;
        Ok(())
    }

    /// Mutes one guild member.
    pub async fn mute_member(&self, guild_id: &str, user_id: &str, mute: &GuildMute) -> Result<()> {
        debug!("Muting member {} in guild {}", user_id, guild_id);
        let path = resource::guild_member_mute(guild_id, user_id);
        self.request_json::<serde_json::Value, _, _>(Method::PATCH, &path, None::<&()>, Some(mute))
            .await?;
        Ok(())
    }

    /// Mutes multiple guild members.
    pub async fn mute_members(
        &self,
        guild_id: &str,
        mute: &GuildMute,
    ) -> Result<GuildMuteResponse> {
        if mute.user_ids.is_empty() {
            return Err(BotError::invalid_data("no user id param"));
        }
        debug!("Muting multiple members in guild {}", guild_id);
        let path = resource::guild_mute(guild_id);
        self.request_json(Method::PATCH, &path, None::<&()>, Some(mute))
            .await
    }
}
