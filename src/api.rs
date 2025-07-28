//! API interface module for the QQ Guild Bot API.
//!
//! This module provides high-level API methods for interacting with the QQ Guild Bot API,
//! including methods for sending messages, managing guilds, channels, and users.

use crate::error::Result;
use crate::http::HttpClient;
use crate::models::api::{BotInfo, GatewayResponse};
use crate::models::*;
use crate::token::Token;

use std::collections::HashMap;
use tracing::{debug, info};

/// High-level API client for the QQ Guild Bot API.
#[derive(Clone)]
pub struct BotApi {
    /// The underlying HTTP client
    http: HttpClient,
}

impl BotApi {
    /// Creates a new API client.
    ///
    /// # Arguments
    ///
    /// * `http` - The HTTP client to use for requests
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{BotApi, http::HttpClient};
    ///
    /// let http = HttpClient::new(30, false).unwrap();
    /// let api = BotApi::new(http);
    /// ```
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Gets the bot's information.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    ///
    /// # Returns
    ///
    /// Bot information including ID, username, and avatar.
    pub async fn get_bot_info(&self, token: &Token) -> Result<BotInfo> {
        info!("Getting bot info");
        let response = self.http.get(token, "/users/@me", None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets the WebSocket gateway URL.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    ///
    /// # Returns
    ///
    /// Gateway information including URL and shard configuration.
    pub async fn get_gateway(&self, token: &Token) -> Result<GatewayResponse> {
        info!("Getting gateway URL");
        let response = self.http.get(token, "/gateway/bot", None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a message to a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The ID of the channel to send the message to
    /// * `content` - The message content
    /// * `msg_id` - Optional message ID to reply to
    ///
    /// # Returns
    ///
    /// The sent message.
    pub async fn post_message(
        &self,
        token: &Token,
        channel_id: &str,
        content: &str,
        msg_id: Option<&str>,
    ) -> Result<Message> {
        debug!("Sending message to channel {}", channel_id);

        let mut body = HashMap::new();
        body.insert("content".to_string(), content.to_string());

        if let Some(reply_id) = msg_id {
            let mut reference = HashMap::new();
            reference.insert("message_id", reply_id);
            let reference_str = serde_json::to_string(&reference)?;
            body.insert("message_reference".to_string(), reference_str);
        }

        let path = format!("/channels/{}/messages", channel_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a direct message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `content` - The message content
    /// * `msg_id` - Optional message ID to reply to
    ///
    /// # Returns
    ///
    /// The sent direct message.
    pub async fn post_dms(
        &self,
        token: &Token,
        guild_id: &str,
        content: &str,
        msg_id: Option<&str>,
    ) -> Result<DirectMessage> {
        debug!("Sending direct message to guild {}", guild_id);

        let mut body = HashMap::new();
        body.insert("content".to_string(), content.to_string());

        if let Some(reply_id) = msg_id {
            let mut reference = HashMap::new();
            reference.insert("message_id", reply_id);
            let reference_str = serde_json::to_string(&reference)?;
            body.insert("message_reference".to_string(), reference_str);
        }

        let path = format!("/dms/{}/messages", guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a group message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `group_openid` - The group OpenID
    /// * `content` - The message content
    /// * `msg_id` - Optional message ID to reply to
    ///
    /// # Returns
    ///
    /// The sent group message.
    pub async fn post_group_message(
        &self,
        token: &Token,
        group_openid: &str,
        content: &str,
        msg_id: Option<&str>,
    ) -> Result<GroupMessage> {
        debug!("Sending group message to {}", group_openid);

        let mut body = HashMap::new();
        body.insert("content".to_string(), content.to_string());
        body.insert("msg_type".to_string(), "0".to_string()); // Text message

        if let Some(reply_id) = msg_id {
            let mut reference = HashMap::new();
            reference.insert("message_id", reply_id);
            let reference_str = serde_json::to_string(&reference)?;
            body.insert("message_reference".to_string(), reference_str);
        }

        let path = format!("/v2/groups/{}/messages", group_openid);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a C2C (client-to-client) message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `openid` - The user's OpenID
    /// * `content` - The message content
    /// * `msg_id` - Optional message ID to reply to
    ///
    /// # Returns
    ///
    /// The sent C2C message.
    pub async fn post_c2c_message(
        &self,
        token: &Token,
        openid: &str,
        content: &str,
        msg_id: Option<&str>,
    ) -> Result<C2CMessage> {
        debug!("Sending C2C message to {}", openid);

        let mut body = HashMap::new();
        body.insert("content".to_string(), content.to_string());
        body.insert("msg_type".to_string(), "0".to_string()); // Text message

        if let Some(reply_id) = msg_id {
            let mut reference = HashMap::new();
            reference.insert("message_id", reply_id);
            let reference_str = serde_json::to_string(&reference)?;
            body.insert("message_reference".to_string(), reference_str);
        }

        let path = format!("/v2/users/{}/messages", openid);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets a list of guilds the bot is in.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `before` - Optional guild ID to get guilds before
    /// * `after` - Optional guild ID to get guilds after
    /// * `limit` - Maximum number of guilds to return (1-100)
    ///
    /// # Returns
    ///
    /// A list of guilds.
    pub async fn get_guilds(
        &self,
        token: &Token,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Guild>> {
        debug!("Getting guilds");

        let mut query = HashMap::new();
        if let Some(b) = before {
            query.insert("before".to_string(), b.to_string());
        }
        if let Some(a) = after {
            query.insert("after".to_string(), a.to_string());
        }
        if let Some(l) = limit {
            query.insert("limit".to_string(), l.to_string());
        }

        let query_params = if query.is_empty() { None } else { Some(&query) };
        let response = self
            .http
            .get(token, "/users/@me/guilds", query_params)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets information about a specific guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Guild information.
    pub async fn get_guild(&self, token: &Token, guild_id: &str) -> Result<Guild> {
        debug!("Getting guild {}", guild_id);
        let path = format!("/guilds/{}", guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets a list of channels in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// A list of channels.
    pub async fn get_channels(&self, token: &Token, guild_id: &str) -> Result<Vec<Channel>> {
        debug!("Getting channels for guild {}", guild_id);
        let path = format!("/guilds/{}/channels", guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets information about a specific channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Channel information.
    pub async fn get_channel(&self, token: &Token, channel_id: &str) -> Result<Channel> {
        debug!("Getting channel {}", channel_id);
        let path = format!("/channels/{}", channel_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets a list of members in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `after` - Optional member ID to get members after
    /// * `limit` - Maximum number of members to return (1-1000)
    ///
    /// # Returns
    ///
    /// A list of guild members.
    pub async fn get_guild_members(
        &self,
        token: &Token,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Member>> {
        debug!("Getting members for guild {}", guild_id);

        let mut query = HashMap::new();
        if let Some(a) = after {
            query.insert("after".to_string(), a.to_string());
        }
        if let Some(l) = limit {
            query.insert("limit".to_string(), l.to_string());
        }

        let query_params = if query.is_empty() { None } else { Some(&query) };
        let path = format!("/guilds/{}/members", guild_id);
        let response = self.http.get(token, &path, query_params).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets information about a specific guild member.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Guild member information.
    pub async fn get_guild_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
    ) -> Result<Member> {
        debug!("Getting member {} in guild {}", user_id, guild_id);
        let path = format!("/guilds/{}/members/{}", guild_id, user_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Deletes a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `hide_tip` - Whether to hide the deletion tip
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn delete_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        hide_tip: Option<bool>,
    ) -> Result<()> {
        debug!("Deleting message {} in channel {}", message_id, channel_id);

        let mut query = HashMap::new();
        if let Some(hide) = hide_tip {
            query.insert(
                "hidetip".to_string(),
                if hide { "true" } else { "false" }.to_string(),
            );
        }

        let query_params = if query.is_empty() { None } else { Some(&query) };
        let path = format!("/channels/{}/messages/{}", channel_id, message_id);
        self.http.delete(token, &path, query_params).await?;
        Ok(())
    }

    /// Gets the HTTP client reference.
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Gets roles for a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// A list of guild roles.
    pub async fn get_guild_roles(&self, token: &Token, guild_id: &str) -> Result<Vec<Role>> {
        debug!("Getting roles for guild {}", guild_id);
        let path = format!("/guilds/{}/roles", guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Creates a new role in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `name` - Role name
    /// * `color` - Role color (ARGB hex as decimal)
    /// * `hoist` - Whether to display separately in member list
    ///
    /// # Returns
    ///
    /// The created role.
    pub async fn create_guild_role(
        &self,
        token: &Token,
        guild_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<Role> {
        debug!("Creating role in guild {}", guild_id);

        let mut body = HashMap::new();
        if let Some(n) = name {
            body.insert("name".to_string(), n.to_string());
        }
        if let Some(c) = color {
            body.insert("color".to_string(), c.to_string());
        }
        if let Some(h) = hoist {
            body.insert("hoist".to_string(), if h { "1" } else { "0" }.to_string());
        }

        let path = format!("/guilds/{}/roles", guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Updates a role in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    /// * `name` - Role name
    /// * `color` - Role color (ARGB hex as decimal)
    /// * `hoist` - Whether to display separately in member list
    ///
    /// # Returns
    ///
    /// The updated role.
    pub async fn update_guild_role(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<Role> {
        debug!("Updating role {} in guild {}", role_id, guild_id);

        let mut body = HashMap::new();
        if let Some(n) = name {
            body.insert("name".to_string(), n.to_string());
        }
        if let Some(c) = color {
            body.insert("color".to_string(), c.to_string());
        }
        if let Some(h) = hoist {
            body.insert("hoist".to_string(), if h { "1" } else { "0" }.to_string());
        }

        let path = format!("/guilds/{}/roles/{}", guild_id, role_id);
        let response = self
            .http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Deletes a role from a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn delete_guild_role(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
    ) -> Result<()> {
        debug!("Deleting role {} from guild {}", role_id, guild_id);
        let path = format!("/guilds/{}/roles/{}", guild_id, role_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Adds a role to a guild member.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `role_id` - The role ID
    /// * `channel_id` - Optional channel ID for operations
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn create_guild_role_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Adding role {} to user {} in guild {}",
            role_id, user_id, guild_id
        );

        let mut body = HashMap::new();
        if let Some(cid) = channel_id {
            body.insert("channel".to_string(), cid.to_string());
        }

        let path = format!("/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Removes a role from a guild member.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `role_id` - The role ID
    /// * `channel_id` - Optional channel ID for operations
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn delete_guild_role_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Removing role {} from user {} in guild {}",
            role_id, user_id, guild_id
        );

        let mut query = HashMap::new();
        if let Some(cid) = channel_id {
            query.insert("channel".to_string(), cid.to_string());
        }

        let query_params = if query.is_empty() { None } else { Some(&query) };
        let path = format!("/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);
        self.http.delete(token, &path, query_params).await?;
        Ok(())
    }

    /// Creates a new channel in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `name` - Channel name
    /// * `channel_type` - Channel type
    /// * `sub_type` - Channel sub type
    /// * `position` - Channel position
    /// * `parent_id` - Parent category ID
    /// * `private_type` - Private type (0=public, 1=admin only, 2=role specified)
    ///
    /// # Returns
    ///
    /// The created channel.
    pub async fn create_channel(
        &self,
        token: &Token,
        guild_id: &str,
        name: &str,
        channel_type: u32,
        sub_type: Option<u32>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
    ) -> Result<Channel> {
        debug!("Creating channel {} in guild {}", name, guild_id);

        let mut body = HashMap::new();
        body.insert("name".to_string(), name.to_string());
        body.insert("type".to_string(), channel_type.to_string());

        if let Some(st) = sub_type {
            body.insert("sub_type".to_string(), st.to_string());
        }
        if let Some(pos) = position {
            body.insert("position".to_string(), pos.to_string());
        }
        if let Some(pid) = parent_id {
            body.insert("parent_id".to_string(), pid.to_string());
        }
        if let Some(pt) = private_type {
            body.insert("private_type".to_string(), pt.to_string());
        }

        let path = format!("/guilds/{}/channels", guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Updates a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `name` - Channel name
    /// * `position` - Channel position
    /// * `parent_id` - Parent category ID
    /// * `private_type` - Private type
    ///
    /// # Returns
    ///
    /// The updated channel.
    pub async fn update_channel(
        &self,
        token: &Token,
        channel_id: &str,
        name: Option<&str>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
    ) -> Result<Channel> {
        debug!("Updating channel {}", channel_id);

        let mut body = HashMap::new();
        if let Some(n) = name {
            body.insert("name".to_string(), n.to_string());
        }
        if let Some(pos) = position {
            body.insert("position".to_string(), pos.to_string());
        }
        if let Some(pid) = parent_id {
            body.insert("parent_id".to_string(), pid.to_string());
        }
        if let Some(pt) = private_type {
            body.insert("private_type".to_string(), pt.to_string());
        }

        let path = format!("/channels/{}", channel_id);
        let response = self
            .http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Deletes a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn delete_channel(&self, token: &Token, channel_id: &str) -> Result<()> {
        debug!("Deleting channel {}", channel_id);
        let path = format!("/channels/{}", channel_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Gets a specific message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    ///
    /// # Returns
    ///
    /// The message.
    pub async fn get_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Message> {
        debug!("Getting message {} in channel {}", message_id, channel_id);
        let path = format!("/channels/{}/messages/{}", channel_id, message_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Mutes all members in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `mute_end_timestamp` - When the mute should end
    /// * `mute_seconds` - Duration in seconds
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn mute_all(
        &self,
        token: &Token,
        guild_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting all members in guild {}", guild_id);

        let mut body = HashMap::new();
        if let Some(ts) = mute_end_timestamp {
            body.insert("mute_end_timestamp".to_string(), ts.to_string());
        }
        if let Some(secs) = mute_seconds {
            body.insert("mute_seconds".to_string(), secs.to_string());
        }

        let path = format!("/guilds/{}/mute", guild_id);
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Cancels mute for all members in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn cancel_mute_all(&self, token: &Token, guild_id: &str) -> Result<()> {
        debug!("Canceling mute for all members in guild {}", guild_id);
        let path = format!("/guilds/{}/mute", guild_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Mutes a specific member.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `mute_end_timestamp` - When the mute should end
    /// * `mute_seconds` - Duration in seconds
    ///
    /// # Returns
    ///
    /// Empty response on success.
    pub async fn mute_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting member {} in guild {}", user_id, guild_id);

        let mut body = HashMap::new();
        if let Some(ts) = mute_end_timestamp {
            body.insert("mute_end_timestamp".to_string(), ts.to_string());
        }
        if let Some(secs) = mute_seconds {
            body.insert("mute_seconds".to_string(), secs.to_string());
        }

        let path = format!("/guilds/{}/members/{}/mute", guild_id, user_id);
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Closes the API client and cleans up resources.
    pub async fn close(&self) {
        self.http.close().await;
    }
}

impl std::fmt::Debug for BotApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BotApi").field("http", &self.http).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpClient;

    #[test]
    fn test_api_creation() {
        let http = HttpClient::new(30, false).unwrap();
        let api = BotApi::new(http);
        assert!(!api.http().is_sandbox());
    }
}
