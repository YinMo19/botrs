//! Main client implementation for the QQ Guild Bot API.
//!
//! This module provides the main `Client` struct that serves as the entry point
//! for bot applications, handling connections, events, and API interactions.

use crate::api::BotApi;
use crate::error::{BotError, Result};
use crate::gateway::Gateway;
use crate::http::HttpClient;
use crate::intents::Intents;
use crate::models::channel::{ChannelSubType, ChannelType};
use crate::models::gateway::GatewayEvent;
use crate::models::guild::{GuildRole, GuildRoles, Member as GuildMember};
use crate::models::*;
use crate::token::Token;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Event handler trait for processing gateway events.
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, _ready: Ready) {}

    /// Called when a message is created (@ mentions).
    async fn message_create(&self, _ctx: Context, _message: Message) {}

    /// Called when a direct message is created.
    async fn direct_message_create(&self, _ctx: Context, _message: DirectMessage) {}

    /// Called when a group message is created.
    async fn group_message_create(&self, _ctx: Context, _message: GroupMessage) {}

    /// Called when a C2C message is created.
    async fn c2c_message_create(&self, _ctx: Context, _message: C2CMessage) {}

    /// Called when a message is deleted.
    async fn message_delete(&self, _ctx: Context, _message: Message) {}

    /// Called when a guild is created (bot joins).
    async fn guild_create(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a guild is updated.
    async fn guild_update(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a guild is deleted (bot leaves).
    async fn guild_delete(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a channel is created.
    async fn channel_create(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is updated.
    async fn channel_update(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is deleted.
    async fn channel_delete(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a guild member is added.
    async fn guild_member_add(&self, _ctx: Context, _member: Member) {}

    /// Called when a guild member is updated.
    async fn guild_member_update(&self, _ctx: Context, _member: Member) {}

    /// Called when a guild member is removed.
    async fn guild_member_remove(&self, _ctx: Context, _member: Member) {}

    /// Called when a message audit passes.
    async fn message_audit_pass(&self, _ctx: Context, _audit: MessageAudit) {}

    /// Called when a message audit is rejected.
    async fn message_audit_reject(&self, _ctx: Context, _audit: MessageAudit) {}

    /// Called for any unhandled events.
    async fn unknown_event(&self, _ctx: Context, _event: GatewayEvent) {}

    /// Called when an error occurs during event processing.
    async fn error(&self, _error: BotError) {
        error!("Event handler error: {}", _error);
    }
}

/// Context passed to event handlers containing API access and bot information.
#[derive(Clone)]
pub struct Context {
    /// API client for making requests
    pub api: Arc<BotApi>,
    /// Authentication token
    pub token: Token,
    /// Bot information
    pub bot_info: Option<BotInfo>,
}

impl Context {
    /// Creates a new context.
    pub fn new(api: Arc<BotApi>, token: Token) -> Self {
        Self {
            api,
            token,
            bot_info: None,
        }
    }

    /// Sets the bot information.
    pub fn with_bot_info(mut self, bot_info: BotInfo) -> Self {
        self.bot_info = Some(bot_info);
        self
    }

    /// Sends a message to a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID to send the message to
    /// * `content` - Message content
    ///
    /// # Returns
    ///
    /// The sent message response.
    pub async fn send_message(&self, channel_id: &str, content: &str) -> Result<MessageResponse> {
        let params = crate::models::message::MessageParams::new_text(content);
        self.api
            .post_message_with_params(&self.token, channel_id, params)
            .await
    }

    /// Sends a message with embed to a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID to send the message to
    /// * `content` - Optional message content
    /// * `embed` - Embed to send
    ///
    /// # Returns
    ///
    /// The sent message response.
    pub async fn send_message_with_embed(
        &self,
        channel_id: &str,
        content: Option<&str>,
        embed: &Embed,
    ) -> Result<MessageResponse> {
        let params = crate::models::message::MessageParams {
            content: content.map(|s| s.to_string()),
            embed: Some(embed.clone()),
            ..Default::default()
        };
        self.api
            .post_message_with_params(&self.token, channel_id, params)
            .await
    }

    /// Sends a reply to a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID to send the reply to
    /// * `content` - Reply content
    /// * `message_id` - The message ID to reply to
    ///
    /// # Returns
    ///
    /// The sent message response.
    pub async fn reply_message(
        &self,
        channel_id: &str,
        content: &str,
        message_id: &str,
    ) -> Result<MessageResponse> {
        let reference = Reference {
            message_id: Some(message_id.to_string()),
            ignore_get_message_error: Some(true),
        };

        let params = crate::models::message::MessageParams {
            content: Some(content.to_string()),
            message_reference: Some(reference),
            ..Default::default()
        };
        self.api
            .post_message_with_params(&self.token, channel_id, params)
            .await
    }

    /// Sends a group message.
    ///
    /// # Arguments
    ///
    /// * `group_openid` - The group OpenID
    /// * `content` - Message content
    ///
    /// # Returns
    ///
    /// The sent group message response.
    pub async fn send_group_message(
        &self,
        group_openid: &str,
        content: &str,
    ) -> Result<MessageResponse> {
        let params = crate::models::message::GroupMessageParams::new_text(content);
        self.api
            .post_group_message_with_params(&self.token, group_openid, params)
            .await
    }

    /// Sends a C2C (client-to-client) message.
    ///
    /// # Arguments
    ///
    /// * `openid` - The user's OpenID
    /// * `content` - Message content
    ///
    /// # Returns
    ///
    /// The sent C2C message response.
    pub async fn send_c2c_message(&self, openid: &str, content: &str) -> Result<MessageResponse> {
        let params = crate::models::message::C2CMessageParams::new_text(content);
        self.api
            .post_c2c_message_with_params(&self.token, openid, params)
            .await
    }

    /// Gets guild information.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Guild information.
    pub async fn get_guild(&self, guild_id: &str) -> Result<Guild> {
        self.api.get_guild(&self.token, guild_id).await
    }

    /// Gets channel information.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Channel information.
    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel> {
        self.api.get_channel(&self.token, channel_id).await
    }

    /// Gets message information.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    ///
    /// # Returns
    ///
    /// The message.
    pub async fn get_message(&self, channel_id: &str, message_id: &str) -> Result<Message> {
        self.api
            .get_message(&self.token, channel_id, message_id)
            .await
    }

    /// Recalls (deletes) a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID to recall
    /// * `hide_tip` - Whether to hide the recall tip
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn recall_message(
        &self,
        channel_id: &str,
        message_id: &str,
        hide_tip: bool,
    ) -> Result<()> {
        self.api
            .recall_message(&self.token, channel_id, message_id, Some(hide_tip))
            .await
    }

    /// Adds a reaction to a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `emoji_type` - The emoji type (1 for system emoji, 2 for custom emoji)
    /// * `emoji_id` - The emoji ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn add_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: u32,
        emoji_id: &str,
    ) -> Result<()> {
        self.api
            .put_reaction(&self.token, channel_id, message_id, emoji_type, emoji_id)
            .await
    }

    /// Removes a reaction from a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID
    /// * `emoji_type` - The emoji type (1 for system emoji, 2 for custom emoji)
    /// * `emoji_id` - The emoji ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn remove_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji_type: u32,
        emoji_id: &str,
    ) -> Result<()> {
        self.api
            .delete_reaction(&self.token, channel_id, message_id, emoji_type, emoji_id)
            .await
    }

    /// Gets the current user's guilds.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - Optional starting guild ID
    /// * `limit` - Number of guilds to return (1-100, default 100)
    /// * `desc` - Whether to return results in descending order
    ///
    /// # Returns
    ///
    /// List of guilds.
    pub async fn get_guilds(
        &self,
        guild_id: Option<&str>,
        limit: Option<u32>,
        desc: Option<bool>,
    ) -> Result<Vec<Guild>> {
        self.api
            .get_guilds(&self.token, guild_id, limit, desc)
            .await
    }

    /// Gets channels in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// List of channels.
    pub async fn get_channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        self.api.get_channels(&self.token, guild_id).await
    }

    /// Creates a new channel in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `name` - Channel name
    /// * `channel_type` - Channel type
    /// * `sub_type` - Channel sub type
    /// * `position` - Channel position
    /// * `parent_id` - Parent channel ID for category channels
    /// * `private_type` - Private type (0=public, 1=private, 2=voice private)
    /// * `private_user_ids` - List of user IDs for private channels
    /// * `speak_permission` - Speak permission (0=invalid, 1=all members, 2=members with role)
    /// * `application_id` - Application ID for application channels
    ///
    /// # Returns
    ///
    /// The created channel.
    pub async fn create_channel(
        &self,
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
        self.api
            .create_channel(
                &self.token,
                guild_id,
                name,
                channel_type,
                sub_type,
                position,
                parent_id,
                private_type,
                private_user_ids,
                speak_permission,
                application_id,
            )
            .await
    }

    /// Gets guild roles.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// List of guild roles.
    pub async fn get_guild_roles(&self, guild_id: &str) -> Result<GuildRoles> {
        self.api.get_guild_roles(&self.token, guild_id).await
    }

    /// Creates a new guild role.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `name` - Role name
    /// * `color` - Role color (ARGB hex value converted to decimal)
    /// * `hoist` - Whether to display separately in member list (0=no, 1=yes)
    ///
    /// # Returns
    ///
    /// The created guild role.
    pub async fn create_guild_role(
        &self,
        guild_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<GuildRole> {
        self.api
            .create_guild_role(&self.token, guild_id, name, color, hoist)
            .await
    }

    /// Updates a guild role.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    /// * `name` - Role name
    /// * `color` - Role color (ARGB hex value converted to decimal)
    /// * `hoist` - Whether to display separately in member list (0=no, 1=yes)
    ///
    /// # Returns
    ///
    /// The updated guild role.
    pub async fn update_guild_role(
        &self,
        guild_id: &str,
        role_id: &str,
        name: Option<&str>,
        color: Option<u32>,
        hoist: Option<bool>,
    ) -> Result<GuildRole> {
        self.api
            .update_guild_role(&self.token, guild_id, role_id, name, color, hoist)
            .await
    }

    /// Deletes a guild role.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn delete_guild_role(&self, guild_id: &str, role_id: &str) -> Result<()> {
        self.api
            .delete_guild_role(&self.token, guild_id, role_id)
            .await
    }

    /// Adds a role to a guild member.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `role_id` - The role ID
    /// * `channel_id` - Optional channel ID for channel-specific roles
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn add_guild_role_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        self.api
            .create_guild_role_member(&self.token, guild_id, role_id, user_id, channel_id)
            .await
    }

    /// Removes a role from a guild member.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    /// * `role_id` - The role ID
    /// * `channel_id` - Optional channel ID for channel-specific roles
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn remove_guild_role_member(
        &self,
        guild_id: &str,
        user_id: &str,
        role_id: &str,
        channel_id: Option<&str>,
    ) -> Result<()> {
        self.api
            .delete_guild_role_member(&self.token, guild_id, role_id, user_id, channel_id)
            .await
    }

    /// Gets guild member information.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Member information.
    pub async fn get_guild_member(&self, guild_id: &str, user_id: &str) -> Result<GuildMember> {
        self.api
            .get_guild_member(&self.token, guild_id, user_id)
            .await
    }

    /// Gets guild members list.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `after` - Optional user ID to get members after
    /// * `limit` - Number of members to return (1-400, default 1)
    ///
    /// # Returns
    ///
    /// List of members.
    pub async fn get_guild_members(
        &self,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<GuildMember>> {
        self.api
            .get_guild_members(&self.token, guild_id, after, limit)
            .await
    }

    /// Kicks a member from the guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID to kick
    /// * `add_blacklist` - Whether to add user to blacklist
    /// * `delete_history_msg_days` - Days of message history to delete (3, 7, 15, 30)
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn kick_member(
        &self,
        guild_id: &str,
        user_id: &str,
        add_blacklist: Option<bool>,
        delete_history_msg_days: Option<i32>,
    ) -> Result<()> {
        self.api
            .delete_member(
                &self.token,
                guild_id,
                user_id,
                add_blacklist,
                delete_history_msg_days,
            )
            .await
    }

    /// Updates audio control in a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `audio_control` - Audio control data
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn update_audio(&self, channel_id: &str, audio_control: &AudioAction) -> Result<()> {
        self.api
            .update_audio(&self.token, channel_id, audio_control)
            .await
    }

    /// Turns on microphone in a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn on_microphone(&self, channel_id: &str) -> Result<()> {
        self.api.on_microphone(&self.token, channel_id).await
    }

    /// Turns off microphone in a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn off_microphone(&self, channel_id: &str) -> Result<()> {
        self.api.off_microphone(&self.token, channel_id).await
    }

    /// Mutes all members in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `mute_end_timestamp` - Optional end timestamp
    /// * `mute_seconds` - Optional duration in seconds
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn mute_all(
        &self,
        guild_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        self.api
            .mute_all(&self.token, guild_id, mute_end_timestamp, mute_seconds)
            .await
    }

    /// Cancels mute for all members in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn cancel_mute_all(&self, guild_id: &str) -> Result<()> {
        self.api.cancel_mute_all(&self.token, guild_id).await
    }

    /// Mutes a specific member in a guild.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The guild ID
    /// * `user_id` - The user ID to mute
    /// * `mute_end_timestamp` - Optional end timestamp
    /// * `mute_seconds` - Optional duration in seconds
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn mute_member(
        &self,
        guild_id: &str,
        user_id: &str,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Result<()> {
        self.api
            .mute_member(
                &self.token,
                guild_id,
                user_id,
                mute_end_timestamp,
                mute_seconds,
            )
            .await
    }

    /// Pins a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID to pin
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn pin_message(&self, channel_id: &str, message_id: &str) -> Result<()> {
        let _ = self
            .api
            .put_pin(&self.token, channel_id, message_id)
            .await?;
        Ok(())
    }

    /// Unpins a message.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `message_id` - The message ID to unpin
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    pub async fn unpin_message(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.api
            .delete_pin(&self.token, channel_id, message_id)
            .await
    }

    /// Gets pinned messages in a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// The pinned messages response.
    pub async fn get_pins(&self, channel_id: &str) -> Result<serde_json::Value> {
        self.api.get_pins(&self.token, channel_id).await
    }

    /// Gets channel permissions for a user.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Channel permissions for the user.
    pub async fn get_channel_user_permissions(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        self.api
            .get_channel_user_permissions(&self.token, channel_id, user_id)
            .await
    }

    /// Gets channel permissions for a role.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `role_id` - The role ID
    ///
    /// # Returns
    ///
    /// Channel permissions for the role.
    pub async fn get_channel_role_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelPermissions> {
        self.api
            .get_channel_role_permissions(&self.token, channel_id, role_id)
            .await
    }

    /// Updates a channel.
    ///
    /// # Arguments
    ///
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
        channel_id: &str,
        name: Option<&str>,
        position: Option<u32>,
        parent_id: Option<&str>,
        private_type: Option<u32>,
        speak_permission: Option<u32>,
    ) -> Result<Channel> {
        self.api
            .update_channel(
                &self.token,
                channel_id,
                name,
                position,
                parent_id,
                private_type,
                speak_permission,
            )
            .await
    }

    /// Deletes a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// The deleted channel.
    pub async fn delete_channel(&self, channel_id: &str) -> Result<Channel> {
        self.api.delete_channel(&self.token, channel_id).await
    }

    /// Creates a DMS session.
    ///
    /// # Arguments
    ///
    /// * `recipient_id` - The recipient user ID
    /// * `source_guild_id` - The source guild ID
    ///
    /// # Returns
    ///
    /// The created DMS session.
    pub async fn create_dms(
        &self,
        recipient_id: &str,
        source_guild_id: &str,
    ) -> Result<serde_json::Value> {
        self.api
            .create_dms(&self.token, recipient_id, source_guild_id)
            .await
    }

    /// Sends a file to a group.
    ///
    /// # Arguments
    ///
    /// * `group_openid` - The group OpenID
    /// * `file_type` - The file type (1=image, 2=video, 3=audio, 4=file)
    /// * `url` - The file URL
    /// * `srv_send_msg` - Whether to send as message
    ///
    /// # Returns
    ///
    /// The file upload response.
    pub async fn post_group_file(
        &self,
        group_openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<serde_json::Value> {
        self.api
            .post_group_file(&self.token, group_openid, file_type, url, srv_send_msg)
            .await
    }

    /// Sends a file to a C2C chat.
    ///
    /// # Arguments
    ///
    /// * `openid` - The user's OpenID
    /// * `file_type` - The file type (1=image, 2=video, 3=audio, 4=file)
    /// * `url` - The file URL
    /// * `srv_send_msg` - Whether to send as message
    ///
    /// # Returns
    ///
    /// The file upload response.
    pub async fn post_c2c_file(
        &self,
        openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<serde_json::Value> {
        self.api
            .post_c2c_file(&self.token, openid, file_type, url, srv_send_msg)
            .await
    }
}

/// Main client for the QQ Guild Bot API.
pub struct Client<H: EventHandler> {
    /// Authentication token
    token: Token,
    /// Intent flags
    intents: Intents,
    /// HTTP client
    http: HttpClient,
    /// API client
    api: Arc<BotApi>,
    /// Event handler
    handler: Arc<H>,
    /// Whether to use sandbox environment
    is_sandbox: bool,
    /// Request timeout in seconds
    timeout: u64,
}

impl<H: EventHandler + 'static> Client<H> {
    /// Creates a new client.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `intents` - Intent flags for events to receive
    /// * `handler` - Event handler implementation
    /// * `is_sandbox` - Whether to use sandbox environment
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{Client, Token, Intents, EventHandler, Context};
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl EventHandler for MyHandler {
    ///     async fn message_create(&self, ctx: Context, message: botrs::Message) {
    ///         println!("Received message: {:?}", message.content);
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let token = Token::new("app_id", "secret");
    ///     let intents = Intents::default();
    ///     let handler = MyHandler;
    ///     let client = Client::new(token, intents, handler, false)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(token: Token, intents: Intents, handler: H, is_sandbox: bool) -> Result<Self> {
        let timeout = crate::DEFAULT_TIMEOUT;

        let http = HttpClient::new(timeout, is_sandbox)?;
        let api = Arc::new(BotApi::new(http.clone()));

        Ok(Self {
            token,
            intents,
            http,
            api,
            handler: Arc::new(handler),
            is_sandbox,
            timeout,
        })
    }

    /// Creates a new client with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `intents` - Intent flags for events to receive
    /// * `handler` - Event handler implementation
    /// * `timeout` - Request timeout in seconds
    /// * `is_sandbox` - Whether to use sandbox environment
    ///
    /// # Returns
    ///
    /// A new client instance.
    pub fn with_config(
        token: Token,
        intents: Intents,
        handler: H,
        timeout: u64,
        is_sandbox: bool,
    ) -> Result<Self> {
        let http = HttpClient::new(timeout, is_sandbox)?;
        let api = Arc::new(BotApi::new(http.clone()));

        Ok(Self {
            token,
            intents,
            http,
            api,
            handler: Arc::new(handler),
            is_sandbox,
            timeout,
        })
    }

    /// Starts the bot and connects to the gateway.
    ///
    /// This method will block until the bot is stopped or an error occurs.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{Client, Token, Intents, EventHandler};
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl EventHandler for MyHandler {}
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let token = Token::new("app_id", "secret");
    ///     let intents = Intents::default();
    ///     let handler = MyHandler;
    ///     let mut client = Client::new(token, intents, handler, false)?;
    ///     client.start().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting bot client");

        // Validate token
        self.token.validate()?;

        // Get bot information
        let bot_info = self.api.get_bot_info(&self.token).await?;
        info!("Bot info: {} ({})", bot_info.username, bot_info.id);

        // Get gateway information
        let gateway_info = self.api.get_gateway(&self.token).await?;
        info!("Gateway URL: {}", gateway_info.url);

        // Create context
        let ctx = Context::new(self.api.clone(), self.token.clone()).with_bot_info(bot_info);

        // Set up event channel
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel();

        // Create and connect gateway
        let gateway = Gateway::new(
            gateway_info.url,
            self.token.clone(),
            self.intents,
            None, // TODO: Implement sharding
        );

        // Start gateway connection in a separate task with auto-reconnect
        let gateway_task = {
            let mut gateway_clone = gateway;
            async move {
                // Gateway now handles auto-reconnect internally
                if let Err(e) = gateway_clone.connect(event_sender).await {
                    error!("Gateway connection failed permanently: {}", e);
                }
            }
        };

        tokio::spawn(gateway_task);

        // Main event processing loop - continue running even if gateway disconnects
        info!("Bot client started, waiting for events...");
        while let Some(event) = event_receiver.recv().await {
            if let Err(e) = self.handle_event(ctx.clone(), event).await {
                self.handler.error(e).await;
            }
        }

        info!("Bot client stopped");
        Ok(())
    }

    /// Handles a gateway event by dispatching it to the appropriate handler method.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Event context
    /// * `event` - Gateway event to handle
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    async fn handle_event(&self, ctx: Context, event: GatewayEvent) -> Result<()> {
        debug!("Handling event: {:?}", event.event_type);

        match event.event_type.as_deref() {
            Some("READY") => {
                if let Some(data) = event.data {
                    if let Ok(ready) = serde_json::from_value::<Ready>(data) {
                        info!("Bot is ready! Session ID: {}", ready.session_id);
                        self.handler.ready(ctx, ready).await;
                    }
                }
            }
            Some("AT_MESSAGE_CREATE") => {
                if let Some(data) = event.data {
                    if let Ok(message) = serde_json::from_value::<Message>(data) {
                        self.handler.message_create(ctx, message).await;
                    }
                }
            }
            Some("DIRECT_MESSAGE_CREATE") => {
                if let Some(data) = event.data {
                    if let Ok(message) = serde_json::from_value::<DirectMessage>(data) {
                        self.handler.direct_message_create(ctx, message).await;
                    }
                }
            }
            Some("GROUP_AT_MESSAGE_CREATE") => {
                if let Some(data) = event.data {
                    debug!(
                        "Attempting to parse GROUP_AT_MESSAGE_CREATE data: {:?}",
                        data
                    );
                    match serde_json::from_value::<GroupMessage>(data.clone()) {
                        Ok(message) => {
                            debug!("Successfully parsed GroupMessage: {:?}", message);
                            self.handler.group_message_create(ctx, message).await;
                        }
                        Err(e) => {
                            error!("Failed to parse GROUP_AT_MESSAGE_CREATE: {}", e);
                            debug!(
                                "Raw event data: {}",
                                serde_json::to_string_pretty(&data).unwrap_or_default()
                            );
                        }
                    }
                }
            }
            Some("C2C_MESSAGE_CREATE") => {
                if let Some(data) = event.data {
                    if let Ok(message) = serde_json::from_value::<C2CMessage>(data) {
                        self.handler.c2c_message_create(ctx, message).await;
                    }
                }
            }
            Some("PUBLIC_MESSAGE_DELETE") => {
                if let Some(data) = event.data {
                    if let Ok(message) = serde_json::from_value::<Message>(data) {
                        self.handler.message_delete(ctx, message).await;
                    }
                }
            }
            Some("GUILD_CREATE") => {
                if let Some(data) = event.data {
                    if let Ok(guild) = serde_json::from_value::<Guild>(data) {
                        self.handler.guild_create(ctx, guild).await;
                    }
                }
            }
            Some("GUILD_UPDATE") => {
                if let Some(data) = event.data {
                    if let Ok(guild) = serde_json::from_value::<Guild>(data) {
                        self.handler.guild_update(ctx, guild).await;
                    }
                }
            }
            Some("GUILD_DELETE") => {
                if let Some(data) = event.data {
                    if let Ok(guild) = serde_json::from_value::<Guild>(data) {
                        self.handler.guild_delete(ctx, guild).await;
                    }
                }
            }
            Some("CHANNEL_CREATE") => {
                if let Some(data) = event.data {
                    if let Ok(channel) = serde_json::from_value::<Channel>(data) {
                        self.handler.channel_create(ctx, channel).await;
                    }
                }
            }
            Some("CHANNEL_UPDATE") => {
                if let Some(data) = event.data {
                    if let Ok(channel) = serde_json::from_value::<Channel>(data) {
                        self.handler.channel_update(ctx, channel).await;
                    }
                }
            }
            Some("CHANNEL_DELETE") => {
                if let Some(data) = event.data {
                    if let Ok(channel) = serde_json::from_value::<Channel>(data) {
                        self.handler.channel_delete(ctx, channel).await;
                    }
                }
            }
            Some("GUILD_MEMBER_ADD") => {
                if let Some(data) = event.data {
                    if let Ok(member) = serde_json::from_value::<Member>(data) {
                        self.handler.guild_member_add(ctx, member).await;
                    }
                }
            }
            Some("GUILD_MEMBER_UPDATE") => {
                if let Some(data) = event.data {
                    if let Ok(member) = serde_json::from_value::<Member>(data) {
                        self.handler.guild_member_update(ctx, member).await;
                    }
                }
            }
            Some("GUILD_MEMBER_REMOVE") => {
                if let Some(data) = event.data {
                    if let Ok(member) = serde_json::from_value::<Member>(data) {
                        self.handler.guild_member_remove(ctx, member).await;
                    }
                }
            }
            Some("MESSAGE_AUDIT_PASS") => {
                if let Some(data) = event.data {
                    if let Ok(audit) = serde_json::from_value::<MessageAudit>(data) {
                        self.handler.message_audit_pass(ctx, audit).await;
                    }
                }
            }
            Some("MESSAGE_AUDIT_REJECT") => {
                if let Some(data) = event.data {
                    if let Ok(audit) = serde_json::from_value::<MessageAudit>(data) {
                        self.handler.message_audit_reject(ctx, audit).await;
                    }
                }
            }
            _ => {
                debug!("Unknown event type: {:?}", event.event_type);
                self.handler.unknown_event(ctx, event).await;
            }
        }

        Ok(())
    }

    /// Gets a reference to the API client.
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Gets a reference to the HTTP client.
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Gets the intents being used.
    pub fn intents(&self) -> Intents {
        self.intents
    }

    /// Returns true if using sandbox environment.
    pub fn is_sandbox(&self) -> bool {
        self.is_sandbox
    }

    /// Shuts down the client and cleans up resources.
    pub async fn shutdown(&self) {
        info!("Shutting down bot client");
        self.api.close().await;
    }
}

impl<H: EventHandler> std::fmt::Debug for Client<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("intents", &self.intents)
            .field("is_sandbox", &self.is_sandbox)
            .field("timeout", &self.timeout)
            .finish()
    }
}
