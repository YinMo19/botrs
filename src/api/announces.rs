use super::{BotApi, resource};
use crate::error::Result;
use crate::models::announce::{Announce, AnnouncesType, RecommendChannel};
use crate::token::Token;
use serde_json::{Value, json};
use tracing::debug;

impl BotApi {
    /// Uploads a group file.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `group_openid` - The group OpenID
    /// * `file_type` - File type (1=image, 2=video, 3=audio, 4=file)
    /// * `url` - File URL
    /// * `srv_send_msg` - Whether to send directly
    ///
    /// # Returns
    ///
    /// Media response.
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

    /// Uploads a C2C file.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `openid` - The user's OpenID
    /// * `file_type` - File type (1=image, 2=video, 3=audio, 4=file)
    /// * `url` - File URL
    /// * `srv_send_msg` - Whether to send directly
    ///
    /// # Returns
    ///
    /// Media response.
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

    /// Creates a channel announcement from a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID where the announcement will be created
    /// * `message_id` - The message ID to turn into an announcement
    ///
    /// # Returns
    ///
    /// The created announcement.
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

    /// Deletes a channel announcement.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID of the announcement to delete
    ///
    /// # Returns
    ///
    /// Success indication.
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

    /// Creates a message-type guild announcement.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID where the announcement will be created
    /// * `channel_id` - The channel ID containing the message to announce
    /// * `message_id` - The message ID to turn into an announcement
    ///
    /// # Returns
    ///
    /// The created announcement.
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

        let body = json!({
            "channel_id": channel_id,
            "message_id": message_id
        });

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

    /// Creates a recommended channel announcement.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID where the announcement will be created
    /// * `announces_type` - The type of announcement
    /// * `recommend_channels` - List of channels to recommend
    ///
    /// # Returns
    ///
    /// The created announcement.
    pub async fn create_recommend_announce(
        &self,
        token: &Token,
        guild_id: &str,
        announces_type: AnnouncesType,
        recommend_channels: Vec<RecommendChannel>,
    ) -> Result<Announce> {
        debug!("Creating recommend announcement in guild {}", guild_id);

        let body = json!({
            "announces_type": u8::from(announces_type),
            "recommend_channels": recommend_channels
        });

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

    /// Deletes a guild announcement.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `message_id` - The message ID of the announcement to delete, or "all" to delete all
    ///
    /// # Returns
    ///
    /// Success indication.
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
