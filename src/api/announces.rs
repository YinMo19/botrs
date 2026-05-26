use super::{BotApi, resource};
use crate::error::Result;
use crate::models::announce::{Announce, AnnouncesType, RecommendChannel};
use crate::token::Token;
use serde::Serialize;
use serde_json::{Value, json};
use tracing::debug;

#[derive(Serialize)]
struct GuildMessageAnnounceBody<'a> {
    channel_id: &'a str,
    message_id: &'a str,
}

#[derive(Serialize)]
struct GuildRecommendAnnounceBody {
    announces_type: u32,
    recommend_channels: Vec<RecommendChannel>,
}

impl BotApi {
    /// Sends a file message to an open-platform group conversation.
    ///
    /// `file_type` follows the platform numeric values: image, video, audio,
    /// or generic file.
    pub async fn post_group_file(
        &self,
        token: &Token,
        group_openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<Value> {
        debug!("Uploading group file to {}", group_openid);

        let body = json!({
            "file_type": file_type,
            "url": url,
            "srv_send_msg": srv_send_msg.unwrap_or(false)
        });

        let path = resource::group_file(group_openid);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(response)
    }

    /// Sends a file message to a C2C conversation.
    ///
    /// `file_type` follows the platform numeric values: image, video, audio,
    /// or generic file.
    pub async fn post_c2c_file(
        &self,
        token: &Token,
        openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<Value> {
        debug!("Uploading C2C file to {}", openid);

        let body = json!({
            "file_type": file_type,
            "url": url,
            "srv_send_msg": srv_send_msg.unwrap_or(false)
        });

        let path = resource::c2c_file(openid);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(response)
    }

    // Announcement APIs

    /// Creates a channel announcement from an existing message.
    pub async fn create_channel_announce(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        debug!(
            "Creating channel announcement in channel {} for message {}",
            channel_id, message_id
        );

        let body = json!({
            "message_id": message_id
        });

        let path = resource::channel_announces(channel_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Deletes a channel announcement by message ID.
    pub async fn delete_channel_announce(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<()> {
        debug!(
            "Deleting announcement {} in channel {}",
            message_id, channel_id
        );

        let path = resource::channel_announce(channel_id, message_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Clears all channel announcements without checking a message ID.
    pub async fn clean_channel_announces(&self, token: &Token, channel_id: &str) -> Result<()> {
        debug!("Clearing announcements in channel {}", channel_id);
        let path = resource::channel_announces_all(channel_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Creates a message-type guild announcement from an existing message.
    pub async fn create_announce(
        &self,
        token: &Token,
        guild_id: &str,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        debug!(
            "Creating announcement in guild {} for message {}",
            guild_id, message_id
        );

        let body = GuildMessageAnnounceBody {
            channel_id,
            message_id,
        };

        let path = resource::guild_announces(guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a message-type guild announcement.
    pub async fn create_guild_announce(
        &self,
        token: &Token,
        guild_id: &str,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Announce> {
        self.create_announce(token, guild_id, channel_id, message_id)
            .await
    }

    /// Creates a guild announcement that recommends channels.
    pub async fn create_recommend_announce(
        &self,
        token: &Token,
        guild_id: &str,
        announces_type: AnnouncesType,
        recommend_channels: Vec<RecommendChannel>,
    ) -> Result<Announce> {
        debug!("Creating recommend announcement in guild {}", guild_id);

        let body = GuildRecommendAnnounceBody {
            announces_type: u8::from(announces_type) as u32,
            recommend_channels,
        };

        let path = resource::guild_announces(guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Self::decode_json(response)
    }

    /// Creates a recommended channel guild announcement.
    pub async fn create_guild_recommend_announce(
        &self,
        token: &Token,
        guild_id: &str,
        announces_type: AnnouncesType,
        recommend_channels: Vec<RecommendChannel>,
    ) -> Result<Announce> {
        self.create_recommend_announce(token, guild_id, announces_type, recommend_channels)
            .await
    }

    /// Deletes a guild announcement by message ID and returns the raw response.
    pub async fn delete_announce(
        &self,
        token: &Token,
        guild_id: &str,
        message_id: &str,
    ) -> Result<Value> {
        debug!("Deleting announcement {} in guild {}", message_id, guild_id);

        let path = resource::guild_announce(guild_id, message_id);
        let response = self.http.delete(token, &path, None::<&()>).await?;
        Ok(response)
    }

    /// Deletes a guild announcement.
    pub async fn delete_guild_announce(
        &self,
        token: &Token,
        guild_id: &str,
        message_id: &str,
    ) -> Result<()> {
        self.delete_announce(token, guild_id, message_id).await?;
        Ok(())
    }

    /// Clears all guild announcements without checking a message ID.
    pub async fn clean_guild_announces(&self, token: &Token, guild_id: &str) -> Result<()> {
        debug!("Clearing announcements in guild {}", guild_id);
        let path = resource::guild_announces_all(guild_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{GuildMessageAnnounceBody, GuildRecommendAnnounceBody};
    use crate::models::announce::{AnnouncesType, RecommendChannel};

    #[test]
    fn high_level_guild_announce_body_matches_botpy_shape() {
        let message = serde_json::to_value(GuildMessageAnnounceBody {
            channel_id: "channel-1",
            message_id: "message-1",
        })
        .unwrap();
        assert_eq!(
            message,
            serde_json::json!({
                "channel_id": "channel-1",
                "message_id": "message-1"
            })
        );

        let recommend = serde_json::to_value(GuildRecommendAnnounceBody {
            announces_type: u8::from(AnnouncesType::Welcome) as u32,
            recommend_channels: vec![RecommendChannel::new(
                "channel-2",
                Some("intro".to_string()),
            )],
        })
        .unwrap();
        assert_eq!(recommend["announces_type"], 1);
        assert_eq!(
            recommend["recommend_channels"][0]["channel_id"],
            "channel-2"
        );
        assert!(recommend.get("channel_id").is_none());
        assert!(recommend.get("message_id").is_none());
    }
}
