//! Bot API implementation for the QQ Guild Bot API.
//!
//! This module provides the main API client for interacting with the QQ Guild Bot API,
//! implementing all endpoints available in the Python SDK.
//!
//! # Message Sending API Refactoring (v0.2.0)
//!
//! ## 🚀 **Major Improvement: Parameter Struct API**
//!
//! We've completely refactored the message sending API to eliminate the problem of
//! functions with many `None` parameters. The new API uses structured parameters
//! with `..Default::default()` for a much cleaner developer experience.
//!
//! ### **Problem Solved**
//!
//! **Before (Multiple None Parameters):**
//! ```rust,no_run
//! # use botrs::*;
//! # async fn example(api: &BotApi, token: &Token) -> Result<()> {
//! api.post_message(
//!     token, "channel_id", Some("Hello!"),
//!     None, None, None, None, None, None, None, None, None  // 😱 Too many Nones!
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! **After (Clean Parameter Structs):**
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token) -> Result<()> {
//! let params = MessageParams::new_text("Hello!");
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## **New API Functions (Recommended)**
//!
//! - [`BotApi::post_message_with_params`] - Send channel messages with [`MessageParams`]
//! - [`BotApi::post_group_message_with_params`] - Send group messages with [`GroupMessageParams`]
//! - [`BotApi::post_c2c_message_with_params`] - Send C2C messages with [`C2CMessageParams`]
//! - [`BotApi::post_dms_with_params`] - Send direct messages with [`DirectMessageParams`]
//!
//! ## **Legacy API Functions (Deprecated)**
//!
//! - [`BotApi::post_message`] ⚠️ Use `post_message_with_params` instead
//! - [`BotApi::post_group_message`] ⚠️ Use `post_group_message_with_params` instead
//! - [`BotApi::post_c2c_message`] ⚠️ Use `post_c2c_message_with_params` instead
//! - [`BotApi::post_dms`] ⚠️ Use `post_dms_with_params` instead
//!
//! ## **Key Benefits**
//!
//!  - **Cleaner Code**: Use `..Default::default()` instead of many `None` parameters
//!  - **Better Readability**: Named fields instead of positional parameters
//!  - **Type Safety**: Structured parameters prevent parameter ordering mistakes
//!  - **Builder Patterns**: Convenient methods like `.with_reply()` and `.with_file_image()`
//!  - **Extensibility**: Easy to add new fields without breaking existing code
//!  - **Compatibility**: Based on official Python botpy API structure
//!
//! ## **Migration Examples**
//!
//! ### Simple Text Message
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token) -> Result<()> {
//! let params = MessageParams::new_text("Hello World!");
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Message with Embed
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::{MessageParams, Embed};
//! # async fn example(api: &BotApi, token: &Token, embed: Embed) -> Result<()> {
//! let params = MessageParams {
//!     content: Some("Check this out!".to_string()),
//!     embed: Some(embed),
//!     ..Default::default()
//! };
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Reply with File
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token, file_data: &[u8]) -> Result<()> {
//! let params = MessageParams::new_text("Here's your file!")
//!     .with_file_image(file_data)
//!     .with_reply("message_id_to_reply_to");
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## **Backward Compatibility**
//!
//! All legacy functions still work but are marked as deprecated. They will be
//! removed in version 1.0.0. Legacy functions internally call the new API
//! to ensure identical behavior.
//!
//! See [`crate::models::message`] for complete migration guide and API documentation.
//!
//! [`MessageParams`]: crate::models::message::MessageParams
//! [`GroupMessageParams`]: crate::models::message::GroupMessageParams
//! [`C2CMessageParams`]: crate::models::message::C2CMessageParams
//! [`DirectMessageParams`]: crate::models::message::DirectMessageParams

use crate::error::Result;
use crate::http::HttpClient;
use crate::models::{
    announce::{Announce, AnnouncesType, RecommendChannel},
    api::{AudioAction, BotInfo, GatewayResponse, MessageResponse},
    channel::{Channel, ChannelPermissions, ChannelSubType, ChannelType},
    emoji::EmojiType,
    guild::{Guild, GuildRole, GuildRoles, Member},
    message::{
        Ark, C2CMessageParams, DirectMessageParams, Embed, GroupMessageParams, Keyboard,
        KeyboardPayload, MarkdownPayload, Media, Message, MessageParams, Reference,
    },
    permission::{APIPermission, APIPermissionDemand, APIPermissionDemandIdentify},
    schedule::{RemindType, Schedule},
};
use crate::reaction::ReactionUsers;
use crate::token::Token;
use base64::Engine;
use serde_json::{Value, json};
use std::collections::HashMap;
use tracing::debug;

/// Bot API client for the QQ Guild Bot API.
#[derive(Clone)]
pub struct BotApi {
    /// The HTTP client used for making requests
    http: HttpClient,
}

impl BotApi {
    /// Creates a new Bot API client.
    ///
    /// # Arguments
    ///
    /// * `http` - The HTTP client to use for requests
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::api::BotApi;
    /// use botrs::http::HttpClient;
    ///
    /// let http = HttpClient::new(30, false).unwrap();
    /// let api = BotApi::new(http);
    /// ```
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Gets information about the current bot.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    ///
    /// # Returns
    ///
    /// The bot's information.
    pub async fn get_bot_info(&self, token: &Token) -> Result<BotInfo> {
        debug!("Getting bot info");
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
    /// Gateway information including WebSocket URL.
    pub async fn get_gateway(&self, token: &Token) -> Result<GatewayResponse> {
        debug!("Getting gateway URL");
        let response = self.http.get(token, "/gateway/bot", None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    // Guild-related APIs

    /// Gets guild information.
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
        let path = format!("/guilds/{guild_id}");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets the current user's guilds.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - Optional starting guild ID
    /// * `limit` - Maximum number of guilds to return (1-100)
    /// * `desc` - Whether to return results in descending order
    ///
    /// # Returns
    ///
    /// List of guilds.
    pub async fn get_guilds(
        &self,
        token: &Token,
        guild_id: Option<&str>,
        limit: Option<u32>,
        desc: Option<bool>,
    ) -> Result<Vec<Guild>> {
        debug!("Getting guilds");

        let mut params = HashMap::new();
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(guild_id) = guild_id {
            if desc.unwrap_or(false) {
                params.insert("before", guild_id.to_string());
            } else {
                params.insert("after", guild_id.to_string());
            }
        }

        let response = self
            .http
            .get(
                token,
                "/users/@me/guilds",
                if params.is_empty() {
                    None
                } else {
                    Some(&params)
                },
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    // Guild Role APIs

    /// Gets guild roles.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Guild roles information.
    pub async fn get_guild_roles(&self, token: &Token, guild_id: &str) -> Result<GuildRoles> {
        debug!("Getting guild roles for {}", guild_id);
        let path = format!("/guilds/{guild_id}/roles");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Creates a new guild role.
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
    ) -> Result<GuildRole> {
        debug!("Creating guild role in {}", guild_id);

        let mut body = HashMap::new();
        if let Some(name) = name {
            body.insert("name", json!(name));
        }
        if let Some(color) = color {
            body.insert("color", json!(color));
        }
        if let Some(hoist) = hoist {
            body.insert("hoist", json!(if hoist { 1 } else { 0 }));
        }

        let path = format!("/guilds/{guild_id}/roles");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Updates a guild role.
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
    ) -> Result<GuildRole> {
        debug!("Updating guild role {} in {}", role_id, guild_id);

        let mut body = HashMap::new();
        if let Some(name) = name {
            body.insert("name", json!(name));
        }
        if let Some(color) = color {
            body.insert("color", json!(color));
        }
        if let Some(hoist) = hoist {
            body.insert("hoist", json!(if hoist { 1 } else { 0 }));
        }

        let path = format!("/guilds/{guild_id}/roles/{role_id}");
        let response = self
            .http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Deletes a guild role.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn delete_guild_role(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
    ) -> Result<()> {
        debug!("Deleting guild role {} in {}", role_id, guild_id);
        let path = format!("/guilds/{guild_id}/roles/{role_id}");
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Adds a member to a guild role.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    /// * `user_id` - The user ID
    /// * `channel_id` - Optional channel ID (for channel-specific roles)
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn create_guild_role_member(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Adding user {} to role {} in guild {}",
            user_id, role_id, guild_id
        );

        let body = if let Some(channel_id) = channel_id {
            json!({ "channel": { "id": channel_id } })
        } else {
            json!({ "channel": { "id": null } })
        };

        let path = format!("/guilds/{guild_id}/members/{user_id}/roles/{role_id}");
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Removes a member from a guild role.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    /// * `user_id` - The user ID
    /// * `channel_id` - Optional channel ID (for channel-specific roles)
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn delete_guild_role_member(
        &self,
        token: &Token,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        debug!(
            "Removing user {} from role {} in guild {}",
            user_id, role_id, guild_id
        );

        let body = if let Some(channel_id) = channel_id {
            json!({ "channel": { "id": channel_id } })
        } else {
            json!({ "channel": { "id": null } })
        };

        let path = format!("/guilds/{guild_id}/members/{user_id}/roles/{role_id}");
        self.http.delete(token, &path, Some(&body)).await?;
        Ok(())
    }

    // Member APIs

    /// Gets a guild member.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Member information.
    pub async fn get_guild_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
    ) -> Result<Member> {
        debug!("Getting guild member {} in {}", user_id, guild_id);
        let path = format!("/guilds/{guild_id}/members/{user_id}");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets guild members list.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `after` - Get members after this user ID
    /// * `limit` - Maximum number of members to return (1-400)
    ///
    /// # Returns
    ///
    /// List of members.
    pub async fn get_guild_members(
        &self,
        token: &Token,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Member>> {
        debug!("Getting guild members for {}", guild_id);

        let mut params = HashMap::new();
        params.insert("after", after.unwrap_or("0").to_string());
        params.insert("limit", limit.unwrap_or(1).to_string());

        let path = format!("/guilds/{guild_id}/members");
        let response = self.http.get(token, &path, Some(&params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Removes a member from a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `add_blacklist` - Whether to add to blacklist
    /// * `delete_history_msg_days` - Days of message history to delete
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn delete_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        add_blacklist: Option<bool>,
        delete_history_msg_days: Option<i32>,
    ) -> Result<()> {
        debug!("Deleting member {} from guild {}", user_id, guild_id);

        let delete_days = match delete_history_msg_days.unwrap_or(0) {
            3 | 7 | 15 | 30 | -1 => delete_history_msg_days.unwrap_or(0),
            _ => 0,
        };

        let body = json!({
            "add_blacklist": add_blacklist.unwrap_or(false),
            "delete_history_msg_days": delete_days
        });

        let path = format!("/guilds/{guild_id}/members/{user_id}");
        self.http.delete(token, &path, Some(&body)).await?;
        Ok(())
    }

    // Channel APIs

    /// Gets channel information.
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
        let path = format!("/channels/{channel_id}");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets channels in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// List of channels.
    pub async fn get_channels(&self, token: &Token, guild_id: &str) -> Result<Vec<Channel>> {
        debug!("Getting channels for guild {}", guild_id);
        let path = format!("/guilds/{guild_id}/channels");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Creates a new channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `name` - Channel name
    /// * `channel_type` - Channel type
    /// * `sub_type` - Channel sub-type
    /// * `position` - Optional position
    /// * `parent_id` - Optional parent category ID
    /// * `private_type` - Optional private type
    /// * `private_user_ids` - Optional private user IDs
    /// * `speak_permission` - Optional speak permission
    /// * `application_id` - Optional application ID
    ///
    /// # Returns
    ///
    /// The created channel.
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
        debug!("Creating channel {} in guild {}", name, guild_id);

        let mut body = json!({
            "name": name,
            "type": u32::from(channel_type),
            "subtype": u32::from(sub_type)
        });

        if let Some(pos) = position {
            body["position"] = json!(pos);
        }
        if let Some(parent) = parent_id {
            body["parent_id"] = json!(parent);
        }
        if let Some(private) = private_type {
            body["private_type"] = json!(private);
        }
        if let Some(users) = private_user_ids {
            body["private_user_ids"] = json!(users);
        }
        if let Some(speak) = speak_permission {
            body["speak_permission"] = json!(speak);
        }
        if let Some(app) = application_id {
            body["application_id"] = json!(app);
        }

        let path = format!("/guilds/{guild_id}/channels");
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
        token: &Token,
        channel_id: &str,
        name: Option<&str>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        speak_permission: Option<u32>,
    ) -> Result<Channel> {
        debug!("Updating channel {}", channel_id);

        let mut body = json!({});
        if let Some(name) = name {
            body["name"] = json!(name);
        }
        if let Some(pos) = position {
            body["position"] = json!(pos);
        }
        if let Some(parent) = parent_id {
            body["parent_id"] = json!(parent);
        }
        if let Some(private) = private_type {
            body["private_type"] = json!(private);
        }
        if let Some(speak) = speak_permission {
            body["speak_permission"] = json!(speak);
        }

        let path = format!("/channels/{channel_id}");
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
    /// The deleted channel.
    pub async fn delete_channel(&self, token: &Token, channel_id: &str) -> Result<Channel> {
        debug!("Deleting channel {}", channel_id);
        let path = format!("/channels/{channel_id}");
        let response = self.http.delete(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    // Message APIs

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
        let path = format!("/channels/{channel_id}/messages/{message_id}");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a message to a channel using MessageParams.
    ///
    /// This is the new, recommended way to send channel messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `params` - Message parameters (see [`MessageParams`])
    ///
    /// # Returns
    ///
    /// The sent message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::MessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// // Simple text message
    /// let params = MessageParams::new_text("Hello world!");
    /// api.post_message_with_params(token, "channel_id", params).await?;
    ///
    /// // Message with reply
    /// let params = MessageParams::new_text("Reply!").with_reply("message_id");
    /// api.post_message_with_params(token, "channel_id", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_message_with_params(
        &self,
        token: &Token,
        channel_id: &str,
        params: MessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending message to channel {}", channel_id);

        // Handle file_image encoding if raw bytes were provided separately
        let body = serde_json::to_value(&params)?;

        let path = format!("/channels/{channel_id}/messages");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a message to a channel (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `content` - Message content
    /// * `embed` - Optional embed
    /// * `ark` - Optional ark template
    /// * `message_reference` - Optional message reference
    /// * `image` - Optional image URL
    /// * `file_image` - Optional file image data
    /// * `msg_id` - Optional message ID to reply to
    /// * `event_id` - Optional event ID
    /// * `markdown` - Optional markdown
    /// * `keyboard` - Optional keyboard
    ///
    /// # Returns
    ///
    /// The sent message response.
    #[deprecated(since = "0.1.0", note = "Use post_message_with_params instead")]
    pub async fn post_message(
        &self,
        token: &Token,
        channel_id: &str,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        image: Option<&str>,
        file_image: Option<&[u8]>,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&Keyboard>,
    ) -> Result<MessageResponse> {
        let params = MessageParams {
            content: content.map(|s| s.to_string()),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            image: image.map(|s| s.to_string()),
            file_image: file_image
                .map(|data| base64::engine::general_purpose::STANDARD.encode(data)),
            msg_id: msg_id.map(|s| s.to_string()),
            event_id: event_id.map(|s| s.to_string()),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
        };

        self.post_message_with_params(token, channel_id, params)
            .await
    }

    /// Sends a group message using GroupMessageParams.
    ///
    /// This is the new, recommended way to send group messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `group_openid` - The group OpenID
    /// * `params` - Group message parameters (see [`GroupMessageParams`])
    ///
    /// # Returns
    ///
    /// The sent group message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::GroupMessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// let params = GroupMessageParams::new_text("Hello group!");
    /// api.post_group_message_with_params(token, "group_openid", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_group_message_with_params(
        &self,
        token: &Token,
        group_openid: &str,
        params: GroupMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending group message to {}", group_openid);

        let body = serde_json::to_value(&params)?;

        let path = format!("/v2/groups/{group_openid}/messages");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a group message (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `group_openid` - The group OpenID
    /// * `msg_type` - Message type (0=text, 1=rich text, 2=markdown, 3=ark, 4=embed, 7=media)
    /// * `content` - Message content
    /// * `embed` - Optional embed
    /// * `ark` - Optional ark template
    /// * `message_reference` - Optional message reference
    /// * `media` - Optional media
    /// * `msg_id` - Optional message ID to reply to
    /// * `msg_seq` - Optional message sequence number
    /// * `event_id` - Optional event ID
    /// * `markdown` - Optional markdown
    /// * `keyboard` - Optional keyboard
    ///
    /// # Returns
    ///
    /// The sent group message response.
    #[deprecated(since = "0.1.0", note = "Use post_group_message_with_params instead")]
    pub async fn post_group_message(
        &self,
        token: &Token,
        group_openid: &str,
        msg_type: Option<u32>,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        media: Option<&Media>,
        msg_id: Option<&str>,
        msg_seq: Option<u32>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        let params = GroupMessageParams {
            msg_type: msg_type.unwrap_or(0),
            content: content.map(|s| s.to_string()),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            media: media.cloned(),
            msg_id: msg_id.map(|s| s.to_string()),
            msg_seq,
            event_id: event_id.map(|s| s.to_string()),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
        };

        self.post_group_message_with_params(token, group_openid, params)
            .await
    }

    /// Sends a C2C (client-to-client) message using C2CMessageParams.
    ///
    /// This is the new, recommended way to send C2C messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `openid` - The user's OpenID
    /// * `params` - C2C message parameters (see [`C2CMessageParams`])
    ///
    /// # Returns
    ///
    /// The sent C2C message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::C2CMessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// let params = C2CMessageParams::new_text("Hello user!");
    /// api.post_c2c_message_with_params(token, "user_openid", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_c2c_message_with_params(
        &self,
        token: &Token,
        openid: &str,
        params: C2CMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending C2C message to {}", openid);

        let body = serde_json::to_value(&params)?;

        let path = format!("/v2/users/{openid}/messages");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a C2C (client-to-client) message (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `openid` - The user's OpenID
    /// * `msg_type` - Message type (0=text, 1=rich text, 2=markdown, 3=ark, 4=embed, 7=media)
    /// * `content` - Message content
    /// * `embed` - Optional embed
    /// * `ark` - Optional ark template
    /// * `message_reference` - Optional message reference
    /// * `media` - Optional media
    /// * `msg_id` - Optional message ID to reply to
    /// * `msg_seq` - Optional message sequence number
    /// * `event_id` - Optional event ID
    /// * `markdown` - Optional markdown
    /// * `keyboard` - Optional keyboard
    ///
    /// # Returns
    ///
    /// The sent C2C message response.
    #[deprecated(since = "0.1.0", note = "Use post_c2c_message_with_params instead")]
    pub async fn post_c2c_message(
        &self,
        token: &Token,
        openid: &str,
        msg_type: Option<u32>,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        media: Option<&Media>,
        msg_id: Option<&str>,
        msg_seq: Option<u32>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&KeyboardPayload>,
    ) -> Result<MessageResponse> {
        let params = C2CMessageParams {
            msg_type: msg_type.unwrap_or(0),
            content: content.map(|s| s.to_string()),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            media: media.cloned(),
            msg_id: msg_id.map(|s| s.to_string()),
            msg_seq,
            event_id: event_id.map(|s| s.to_string()),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
        };

        self.post_c2c_message_with_params(token, openid, params)
            .await
    }

    /// Sends a direct message using DirectMessageParams.
    ///
    /// This is the new, recommended way to send direct messages. It uses a parameter struct
    /// instead of many optional arguments, making the code cleaner and more maintainable.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The DM session guild ID
    /// * `params` - Direct message parameters (see [`DirectMessageParams`])
    ///
    /// # Returns
    ///
    /// The sent direct message response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use botrs::*;
    /// # use botrs::models::message::DirectMessageParams;
    /// # async fn example(api: &BotApi, token: &Token) -> Result<()> {
    /// let params = DirectMessageParams::new_text("Hello DM!");
    /// api.post_dms_with_params(token, "guild_id", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_dms_with_params(
        &self,
        token: &Token,
        guild_id: &str,
        params: DirectMessageParams,
    ) -> Result<MessageResponse> {
        debug!("Sending direct message to guild session {}", guild_id);

        let body = serde_json::to_value(&params)?;

        let path = format!("/dms/{guild_id}/messages");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sends a direct message (legacy API for backward compatibility).
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The DM session guild ID
    /// * `content` - Message content
    /// * `embed` - Optional embed
    /// * `ark` - Optional ark template
    /// * `message_reference` - Optional message reference
    /// * `image` - Optional image URL
    /// * `file_image` - Optional file image data
    /// * `msg_id` - Optional message ID to reply to
    /// * `event_id` - Optional event ID
    /// * `markdown` - Optional markdown
    /// * `keyboard` - Optional keyboard
    ///
    /// # Returns
    ///
    /// The sent direct message response.
    #[deprecated(since = "0.1.0", note = "Use post_dms_with_params instead")]
    pub async fn post_dms(
        &self,
        token: &Token,
        guild_id: &str,
        content: Option<&str>,
        embed: Option<&Embed>,
        ark: Option<&Ark>,
        message_reference: Option<&Reference>,
        image: Option<&str>,
        file_image: Option<&[u8]>,
        msg_id: Option<&str>,
        event_id: Option<&str>,
        markdown: Option<&MarkdownPayload>,
        keyboard: Option<&Keyboard>,
    ) -> Result<MessageResponse> {
        let params = DirectMessageParams {
            content: content.map(|s| s.to_string()),
            embed: embed.cloned(),
            ark: ark.cloned(),
            message_reference: message_reference.cloned(),
            image: image.map(|s| s.to_string()),
            file_image: file_image
                .map(|data| base64::engine::general_purpose::STANDARD.encode(data)),
            msg_id: msg_id.map(|s| s.to_string()),
            event_id: event_id.map(|s| s.to_string()),
            markdown: markdown.cloned(),
            keyboard: keyboard.cloned(),
        };

        self.post_dms_with_params(token, guild_id, params).await
    }

    /// Creates a direct message session.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The source guild ID
    /// * `user_id` - The target user ID
    ///
    /// # Returns
    ///
    /// DM session information.
    pub async fn create_dms(&self, token: &Token, guild_id: &str, user_id: &str) -> Result<Value> {
        debug!(
            "Creating DM session for user {} from guild {}",
            user_id, guild_id
        );

        let body = json!({
            "recipient_id": user_id,
            "source_guild_id": guild_id
        });

        let response = self
            .http
            .post(token, "/users/@me/dms", None::<&()>, Some(&body))
            .await?;
        Ok(response)
    }

    /// Recalls (deletes) a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `hidetip` - Whether to hide the recall tip
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn recall_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Recalling message {} in channel {}", message_id, channel_id);

        let mut params = HashMap::new();
        params.insert(
            "hidetip",
            if hidetip.unwrap_or(false) {
                "true"
            } else {
                "false"
            }
            .to_string(),
        );

        let path = format!("/channels/{channel_id}/messages/{message_id}");
        self.http.delete(token, &path, Some(&params)).await?;
        Ok(())
    }

    // Audio APIs

    /// Updates audio control.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `audio_control` - Audio control data
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn update_audio(
        &self,
        token: &Token,
        channel_id: &str,
        audio_control: &AudioAction,
    ) -> Result<()> {
        debug!("Updating audio in channel {}", channel_id);
        let path = format!("/channels/{channel_id}/audio");
        let _response = self
            .http
            .post(token, &path, None::<&()>, Some(audio_control))
            .await?;
        Ok(())
    }

    /// Turn on microphone.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn on_microphone(&self, token: &Token, channel_id: &str) -> Result<()> {
        debug!("Turning on microphone in channel {}", channel_id);
        let path = format!("/channels/{channel_id}/mic");
        self.http
            .put(token, &path, None::<&()>, None::<&()>)
            .await?;
        Ok(())
    }

    /// Turn off microphone.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn off_microphone(&self, token: &Token, channel_id: &str) -> Result<()> {
        debug!("Turning off microphone in channel {}", channel_id);
        let path = format!("/channels/{channel_id}/mic");
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    // Muting APIs

    /// Mutes all members in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `mute_end_timestamp` - Optional end timestamp
    /// * `mute_seconds` - Optional duration in seconds
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn mute_all(
        &self,
        token: &Token,
        guild_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting all members in guild {}", guild_id);

        let body = json!({
            "mute_end_timestamp": mute_end_timestamp,
            "mute_seconds": mute_seconds
        });

        let path = format!("/guilds/{guild_id}/mute");
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Cancels mute for all members.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn cancel_mute_all(&self, token: &Token, guild_id: &str) -> Result<()> {
        debug!("Canceling mute for all members in guild {}", guild_id);

        let body = json!({
            "mute_end_timestamp": "0",
            "mute_seconds": "0"
        });

        let path = format!("/guilds/{guild_id}/mute");
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Mutes a specific member.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `mute_end_timestamp` - Optional end timestamp
    /// * `mute_seconds` - Optional duration in seconds
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn mute_member(
        &self,
        token: &Token,
        guild_id: &str,
        user_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        debug!("Muting member {} in guild {}", user_id, guild_id);

        let body = json!({
            "mute_end_timestamp": mute_end_timestamp,
            "mute_seconds": mute_seconds
        });

        let path = format!("/guilds/{guild_id}/members/{user_id}/mute");
        self.http
            .put(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(())
    }

    /// Gets channel permissions for a user.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Channel permissions.
    pub async fn get_channel_user_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        debug!(
            "Getting channel permissions for user {} in channel {}",
            user_id, channel_id
        );
        let path = format!("/channels/{channel_id}/members/{user_id}/permissions");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets channel permissions for a role.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Channel permissions.
    pub async fn get_channel_role_permissions(
        &self,
        token: &Token,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelPermissions> {
        debug!(
            "Getting channel permissions for role {} in channel {}",
            role_id, channel_id
        );
        let path = format!("/channels/{channel_id}/roles/{role_id}/permissions");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Adds a reaction to a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `emoji_type` - The emoji type (1=system, 2=emoji)
    /// * `emoji_id` - The emoji ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn put_reaction(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji_type: u32,
        emoji_id: &str,
    ) -> Result<()> {
        debug!(
            "Adding reaction to message {} in channel {}",
            message_id, channel_id
        );
        let path = format!(
            "/channels/{channel_id}/messages/{message_id}/reactions/{emoji_type}/{emoji_id}"
        );
        self.http
            .put(token, &path, None::<&()>, None::<&()>)
            .await?;
        Ok(())
    }

    /// Removes a reaction from a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `emoji_type` - The emoji type (1=system, 2=emoji)
    /// * `emoji_id` - The emoji ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn delete_reaction(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji_type: u32,
        emoji_id: &str,
    ) -> Result<()> {
        debug!(
            "Removing reaction from message {} in channel {}",
            message_id, channel_id
        );
        let path = format!(
            "/channels/{channel_id}/messages/{message_id}/reactions/{emoji_type}/{emoji_id}"
        );
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Pins a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn put_pin(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Value> {
        debug!("Pinning message {} in channel {}", message_id, channel_id);
        let path = format!("/channels/{channel_id}/pins/{message_id}");
        let response = self
            .http
            .put(token, &path, None::<&()>, Some(&json!({})))
            .await?;
        Ok(response)
    }

    /// Unpins a message.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn delete_pin(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
    ) -> Result<()> {
        debug!("Unpinning message {} in channel {}", message_id, channel_id);
        let path = format!("/channels/{channel_id}/pins/{message_id}");
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Gets pinned messages.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Pinned messages.
    pub async fn get_pins(&self, token: &Token, channel_id: &str) -> Result<Value> {
        debug!("Getting pinned messages in channel {}", channel_id);
        let path = format!("/channels/{channel_id}/pins");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(response)
    }

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

        let path = format!("/v2/groups/{group_openid}/files");
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

        let path = format!("/v2/users/{openid}/files");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(response)
    }

    // Announcement APIs

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

        let path = format!("/guilds/{guild_id}/announces");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
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

        let path = format!("/guilds/{guild_id}/announces");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
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

        let path = format!("/guilds/{guild_id}/announces/{message_id}");
        let response = self.http.delete(token, &path, None::<&()>).await?;
        Ok(response)
    }

    // Permission APIs

    /// Gets the list of API permissions for a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// List of API permissions.
    pub async fn get_permissions(
        &self,
        token: &Token,
        guild_id: &str,
    ) -> Result<Vec<APIPermission>> {
        debug!("Getting permissions for guild {}", guild_id);

        let path = format!("/guilds/{guild_id}/api_permission");
        let response = self.http.get(token, &path, None::<&()>).await?;

        // The response has an extra "apis" level
        if let Some(apis) = response.get("apis") {
            Ok(serde_json::from_value(apis.clone())?)
        } else {
            Ok(vec![])
        }
    }

    /// Creates an API permission demand request.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID where permission is requested
    /// * `channel_id` - The channel ID where the request will be sent
    /// * `api_identify` - The API identifier for which permission is requested
    /// * `desc` - Description explaining why the permission is needed
    ///
    /// # Returns
    ///
    /// The created permission demand.
    pub async fn post_permission_demand(
        &self,
        token: &Token,
        guild_id: &str,
        channel_id: &str,
        api_identify: APIPermissionDemandIdentify,
        desc: &str,
    ) -> Result<APIPermissionDemand> {
        debug!("Creating permission demand in guild {}", guild_id);

        let body = json!({
            "channel_id": channel_id,
            "api_identify": api_identify,
            "desc": desc
        });

        let path = format!("/guilds/{guild_id}/api_permission/demand");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    // Reaction APIs

    /// Gets the list of users who reacted with a specific emoji.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The channel ID containing the message
    /// * `message_id` - The message ID
    /// * `emoji_type` - The type of emoji (1 = system, 2 = custom)
    /// * `emoji_id` - The emoji ID
    /// * `cookie` - Optional pagination cookie from previous request
    /// * `limit` - Maximum number of users to return (1-100, default 20)
    ///
    /// # Returns
    ///
    /// List of users who reacted and pagination info.
    pub async fn get_reaction_users(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        emoji_type: EmojiType,
        emoji_id: &str,
        cookie: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ReactionUsers> {
        debug!(
            "Getting reaction users for message {} with emoji {}",
            message_id, emoji_id
        );

        let mut params = HashMap::new();
        params.insert("limit", limit.unwrap_or(20).to_string());
        if let Some(cookie) = cookie {
            params.insert("cookie", cookie.to_string());
        }

        let path = format!(
            "/channels/{channel_id}/messages/{message_id}/reactions/{emoji_type}/{emoji_id}",
            emoji_type = u8::from(emoji_type)
        );
        let response = self.http.get(token, &path, Some(&params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    // Schedule APIs

    /// Gets the list of schedules for a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `since` - Optional timestamp to get schedules after this time
    ///
    /// # Returns
    ///
    /// List of schedules.
    pub async fn get_schedules(
        &self,
        token: &Token,
        channel_id: &str,
        since: Option<&str>,
    ) -> Result<Vec<Schedule>> {
        debug!("Getting schedules for channel {}", channel_id);

        let body = if let Some(since) = since {
            json!({ "since": since })
        } else {
            json!({})
        };

        let path = format!("/channels/{channel_id}/schedules");
        let response = self
            .http
            .get(
                token,
                &path,
                if since.is_some() { Some(&body) } else { None },
            )
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets a specific schedule by ID.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `schedule_id` - The schedule ID
    ///
    /// # Returns
    ///
    /// The schedule details.
    pub async fn get_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<Schedule> {
        debug!("Getting schedule {} in channel {}", schedule_id, channel_id);

        let path = format!("/channels/{channel_id}/schedules/{schedule_id}");
        let response = self.http.get(token, &path, None::<&()>).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Creates a new schedule in a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `name` - Name of the schedule
    /// * `start_timestamp` - Start time as Unix timestamp string
    /// * `end_timestamp` - End time as Unix timestamp string
    /// * `jump_channel_id` - Channel ID to jump to when event starts
    /// * `remind_type` - Type of reminder to set
    ///
    /// # Returns
    ///
    /// The created schedule.
    pub async fn create_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        remind_type: RemindType,
    ) -> Result<Schedule> {
        debug!("Creating schedule '{}' in channel {}", name, channel_id);

        let body = json!({
            "schedule": {
                "name": name,
                "start_timestamp": start_timestamp,
                "end_timestamp": end_timestamp,
                "jump_channel_id": jump_channel_id,
                "reminder_id": u8::from(remind_type)
            }
        });

        let path = format!("/channels/{channel_id}/schedules");
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Updates an existing schedule.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `schedule_id` - The schedule ID to update
    /// * `name` - New name of the schedule
    /// * `start_timestamp` - New start time as Unix timestamp string
    /// * `end_timestamp` - New end time as Unix timestamp string
    /// * `jump_channel_id` - New channel ID to jump to when event starts
    /// * `remind_type` - New type of reminder to set
    ///
    /// # Returns
    ///
    /// The updated schedule.
    pub async fn update_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
        name: &str,
        start_timestamp: &str,
        end_timestamp: &str,
        jump_channel_id: &str,
        remind_type: RemindType,
    ) -> Result<Schedule> {
        debug!(
            "Updating schedule {} in channel {}",
            schedule_id, channel_id
        );

        let body = json!({
            "schedule": {
                "name": name,
                "start_timestamp": start_timestamp,
                "end_timestamp": end_timestamp,
                "jump_channel_id": jump_channel_id,
                "reminder_id": u8::from(remind_type)
            }
        });

        let path = format!("/channels/{channel_id}/schedules/{schedule_id}");
        let response = self
            .http
            .patch(token, &path, None::<&()>, Some(&body))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Deletes a schedule.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `channel_id` - The schedule channel ID
    /// * `schedule_id` - The schedule ID to delete
    ///
    /// # Returns
    ///
    /// Success indication.
    pub async fn delete_schedule(
        &self,
        token: &Token,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<Value> {
        debug!(
            "Deleting schedule {} in channel {}",
            schedule_id, channel_id
        );

        let path = format!("/channels/{channel_id}/schedules/{schedule_id}");
        let response = self.http.delete(token, &path, None::<&()>).await?;
        Ok(response)
    }

    /// Gets the HTTP client reference.
    pub fn http(&self) -> &HttpClient {
        &self.http
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
