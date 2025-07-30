# Context API Reference

The `Context` struct provides access to the bot's API client and authentication token within event handlers. It acts as the primary interface for making API calls and accessing bot information during event processing.

## Overview

```rust
use botrs::{Context, BotApi, Token, BotInfo};

pub struct Context {
    pub api: Arc<BotApi>,
    pub token: Token,
    pub bot_info: Option<BotInfo>,
}
```

The `Context` is automatically provided to all event handler methods and contains everything needed to interact with the QQ Guild API.

## Fields

### `api`

```rust
pub api: Arc<BotApi>
```

The API client for making HTTP requests to QQ Guild's REST API. This is shared between all contexts and event handlers.

#### Example

```rust
// Access the API client directly
let guild = ctx.api.get_guild(&ctx.token, "guild_123").await?;
```

### `token`

```rust
pub token: Token
```

The authentication token used for API requests. This is automatically included in all API calls made through the context.

#### Example

```rust
// Token is automatically used in context methods
let message = ctx.send_message("channel_123", "Hello!").await?;

// Or used explicitly with the API client
let params = MessageParams::new_text("Hello!");
let message = ctx.api.post_message_with_params(&ctx.token, "channel_123", params).await?;
```

### `bot_info`

```rust
pub bot_info: Option<BotInfo>
```

Information about the bot user, including username, ID, and other details. This is populated after the bot connects and receives the READY event.

#### Example

```rust
if let Some(bot_info) = &ctx.bot_info {
    println!("Bot: {} (ID: {})", bot_info.username, bot_info.id);
}
```

## Constructor Methods

### `new`

Creates a new context with API client and token.

```rust
pub fn new(api: Arc<BotApi>, token: Token) -> Self
```

#### Parameters

- `api`: The API client instance
- `token`: Authentication token

#### Example

```rust
use std::sync::Arc;

let api = Arc::new(BotApi::new(http_client));
let token = Token::new("app_id", "secret");
let ctx = Context::new(api, token);
```

### `with_bot_info`

Adds bot information to the context.

```rust
pub fn with_bot_info(mut self, bot_info: BotInfo) -> Self
```

#### Parameters

- `bot_info`: Bot user information

#### Returns

The context with bot information added.

#### Example

```rust
let ctx = Context::new(api, token)
    .with_bot_info(bot_info);
```

## Message Methods

### `send_message`

Sends a simple text message to a channel.

```rust
pub async fn send_message(&self, channel_id: &str, content: &str) -> Result<Message>
```

#### Parameters

- `channel_id`: The channel to send the message to
- `content`: The message content

#### Returns

The sent message data.

#### Example

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            if content == "!ping" {
                let _ = ctx.send_message(&msg.channel_id, "Pong!").await;
            }
        }
    }
}
```

### `send_message_with_embed`

Sends a message with an embed to a channel.

```rust
pub async fn send_message_with_embed(
    &self,
    channel_id: &str,
    content: Option<&str>,
    embed: &MessageEmbed,
) -> Result<Message>
```

#### Parameters

- `channel_id`: The channel to send the message to
- `content`: Optional text content
- `embed`: The embed to include

#### Returns

The sent message data.

#### Example

```rust
use botrs::MessageEmbed;

let embed = MessageEmbed::new()
    .title("Bot Status")
    .description("All systems operational")
    .color(0x00ff00);

let _ = ctx.send_message_with_embed(
    &channel_id, 
    Some("Status Update"), 
    &embed
).await;
```

### `reply_message`

Replies to a specific message.

```rust
pub async fn reply_message(
    &self,
    channel_id: &str,
    message_id: &str,
    content: &str,
) -> Result<Message>
```

#### Parameters

- `channel_id`: The channel containing the original message
- `message_id`: The message to reply to
- `content`: The reply content

#### Returns

The sent reply message.

#### Example

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            if content.starts_with("!echo ") {
                let echo_text = &content[6..];
                let _ = ctx.reply_message(&msg.channel_id, &msg.id, echo_text).await;
            }
        }
    }
}
```

### `send_group_message`

Sends a message to a group.

```rust
pub async fn send_group_message(
    &self,
    group_openid: &str,
    content: &str,
) -> Result<Message>
```

#### Parameters

- `group_openid`: The group identifier
- `content`: The message content

#### Returns

The sent group message.

### `send_c2c_message`

Sends a C2C (client-to-client) message.

```rust
pub async fn send_c2c_message(
    &self,
    openid: &str,
    content: &str,
) -> Result<Message>
```

#### Parameters

- `openid`: The user identifier
- `content`: The message content

#### Returns

The sent C2C message.

## Guild Methods

### `get_guild`

Gets information about a guild.

```rust
pub async fn get_guild(&self, guild_id: &str) -> Result<Guild>
```

#### Parameters

- `guild_id`: The guild ID to retrieve

#### Returns

Guild information.

#### Example

```rust
let guild = ctx.get_guild("guild_123").await?;
println!("Guild: {} with {} members", guild.name, guild.member_count);
```

### `get_guilds`

Gets all guilds the bot is in.

```rust
pub async fn get_guilds(&self) -> Result<Vec<Guild>>
```

#### Returns

List of guilds.

#### Example

```rust
let guilds = ctx.get_guilds().await?;
for guild in guilds {
    println!("Guild: {} ({})", guild.name, guild.id);
}
```

## Channel Methods

### `get_channel`

Gets information about a channel.

```rust
pub async fn get_channel(&self, channel_id: &str) -> Result<Channel>
```

#### Parameters

- `channel_id`: The channel ID to retrieve

#### Returns

Channel information.

### `get_channels`

Gets all channels in a guild.

```rust
pub async fn get_channels(&self, guild_id: &str) -> Result<Vec<Channel>>
```

#### Parameters

- `guild_id`: The guild ID

#### Returns

List of channels in the guild.

### `create_channel`

Creates a new channel in a guild.

```rust
pub async fn create_channel(
    &self,
    guild_id: &str,
    name: &str,
    channel_type: ChannelType,
    sub_type: ChannelSubType,
) -> Result<Channel>
```

#### Parameters

- `guild_id`: The guild to create the channel in
- `name`: The channel name
- `channel_type`: The type of channel
- `sub_type`: The channel sub-type

#### Returns

The created channel.

#### Example

```rust
use botrs::{ChannelType, ChannelSubType};

let channel = ctx.create_channel(
    "guild_123",
    "new-channel",
    ChannelType::Text,
    ChannelSubType::Chat,
).await?;

println!("Created channel: {}", channel.name);
```

### `update_channel`

Updates an existing channel.

```rust
pub async fn update_channel(
    &self,
    channel_id: &str,
    name: Option<&str>,
    position: Option<u32>,
) -> Result<Channel>
```

#### Parameters

- `channel_id`: The channel to update
- `name`: Optional new name
- `position`: Optional new position

#### Returns

The updated channel.

### `delete_channel`

Deletes a channel.

```rust
pub async fn delete_channel(&self, channel_id: &str) -> Result<()>
```

#### Parameters

- `channel_id`: The channel to delete

## Member Management

### `get_guild_member`

Gets information about a guild member.

```rust
pub async fn get_guild_member(
    &self,
    guild_id: &str,
    user_id: &str,
) -> Result<Member>
```

#### Parameters

- `guild_id`: The guild ID
- `user_id`: The user ID

#### Returns

Member information.

### `get_guild_members`

Gets guild members with optional pagination.

```rust
pub async fn get_guild_members(
    &self,
    guild_id: &str,
    after: Option<&str>,
    limit: Option<u32>,
) -> Result<Vec<Member>>
```

#### Parameters

- `guild_id`: The guild ID
- `after`: Pagination cursor
- `limit`: Maximum number of members to return

#### Returns

List of guild members.

### `kick_member`

Removes a member from the guild.

```rust
pub async fn kick_member(
    &self,
    guild_id: &str,
    user_id: &str,
    add_blacklist: bool,
    delete_history_msg_days: Option<u8>,
    reason: Option<&str>,
) -> Result<()>
```

#### Parameters

- `guild_id`: The guild ID
- `user_id`: The user to kick
- `add_blacklist`: Whether to add to blacklist
- `delete_history_msg_days`: Days of message history to delete
- `reason`: Reason for kicking

#### Example

```rust
ctx.kick_member(
    "guild_123",
    "user_456",
    false,
    Some(1),
    Some("Spam violation")
).await?;
```

## Role Management

### `get_guild_roles`

Gets all roles in a guild.

```rust
pub async fn get_guild_roles(&self, guild_id: &str) -> Result<GuildRoles>
```

### `create_guild_role`

Creates a new role in a guild.

```rust
pub async fn create_guild_role(
    &self,
    guild_id: &str,
    name: &str,
    color: Option<u32>,
    hoist: Option<bool>,
    mentionable: Option<bool>,
) -> Result<Role>
```

#### Parameters

- `guild_id`: The guild ID
- `name`: Role name
- `color`: Role color (hex)
- `hoist`: Whether role is displayed separately
- `mentionable`: Whether role can be mentioned

#### Returns

The created role.

### `update_guild_role`

Updates an existing guild role.

```rust
pub async fn update_guild_role(
    &self,
    guild_id: &str,
    role_id: &str,
    name: Option<&str>,
    color: Option<u32>,
    hoist: Option<bool>,
    mentionable: Option<bool>,
) -> Result<Role>
```

### `delete_guild_role`

Deletes a guild role.

```rust
pub async fn delete_guild_role(
    &self,
    guild_id: &str,
    role_id: &str,
) -> Result<()>
```

### `add_guild_role_member`

Assigns a role to a member.

```rust
pub async fn add_guild_role_member(
    &self,
    guild_id: &str,
    role_id: &str,
    user_id: &str,
    reason: Option<&str>,
) -> Result<()>
```

### `remove_guild_role_member`

Removes a role from a member.

```rust
pub async fn remove_guild_role_member(
    &self,
    guild_id: &str,
    role_id: &str,
    user_id: &str,
    reason: Option<&str>,
) -> Result<()>
```

## Audio and Voice

### `update_audio`

Updates audio playback in a voice channel.

```rust
pub async fn update_audio(
    &self,
    channel_id: &str,
    audio_control: AudioControl,
) -> Result<()>
```

#### Parameters

- `channel_id`: The voice channel ID
- `audio_control`: Audio control parameters

### `on_microphone`

Enables microphone for a user.

```rust
pub async fn on_microphone(
    &self,
    channel_id: &str,
    user_id: &str,
) -> Result<()>
```

### `off_microphone`

Disables microphone for a user.

```rust
pub async fn off_microphone(
    &self,
    channel_id: &str,
    user_id: &str,
) -> Result<()>
```

### `mute_all`

Mutes all users in a voice channel.

```rust
pub async fn mute_all(&self, channel_id: &str) -> Result<()>
```

### `cancel_mute_all`

Unmutes all users in a voice channel.

```rust
pub async fn cancel_mute_all(&self, channel_id: &str) -> Result<()>
```

### `mute_member`

Mutes a specific member.

```rust
pub async fn mute_member(
    &self,
    guild_id: &str,
    user_id: &str,
    mute_end_timestamp: Option<&str>,
    mute_seconds: Option<&str>,
) -> Result<()>
```

## Message Management

### `get_message`

Gets a specific message.

```rust
pub async fn get_message(
    &self,
    channel_id: &str,
    message_id: &str,
) -> Result<Message>
```

### `recall_message`

Recalls (deletes) a message.

```rust
pub async fn recall_message(
    &self,
    channel_id: &str,
    message_id: &str,
    hidetip: bool,
) -> Result<()>
```

#### Parameters

- `channel_id`: The channel containing the message
- `message_id`: The message to recall
- `hidetip`: Whether to hide the deletion notification

## Reactions and Pins

### `add_reaction`

Adds a reaction to a message.

```rust
pub async fn add_reaction(
    &self,
    channel_id: &str,
    message_id: &str,
    emoji: ReactionEmoji,
) -> Result<()>
```

### `remove_reaction`

Removes a reaction from a message.

```rust
pub async fn remove_reaction(
    &self,
    channel_id: &str,
    message_id: &str,
    emoji: ReactionEmoji,
) -> Result<()>
```

### `pin_message`

Pins a message in a channel.

```rust
pub async fn pin_message(
    &self,
    channel_id: &str,
    message_id: &str,
) -> Result<PinMessage>
```

### `unpin_message`

Unpins a message in a channel.

```rust
pub async fn unpin_message(
    &self,
    channel_id: &str,
    message_id: &str,
) -> Result<()>
```

### `get_pins`

Gets all pinned messages in a channel.

```rust
pub async fn get_pins(&self, channel_id: &str) -> Result<PinMessages>
```

## Permissions

### `get_channel_user_permissions`

Gets user permissions for a channel.

```rust
pub async fn get_channel_user_permissions(
    &self,
    channel_id: &str,
    user_id: &str,
) -> Result<ChannelPermissions>
```

### `get_channel_role_permissions`

Gets role permissions for a channel.

```rust
pub async fn get_channel_role_permissions(
    &self,
    channel_id: &str,
    role_id: &str,
) -> Result<ChannelPermissions>
```

## File Operations

### `create_dms`

Creates a direct message session.

```rust
pub async fn create_dms(
    &self,
    recipient_id: &str,
    source_guild_id: &str,
) -> Result<DirectMessageGuild>
```

### `post_group_file`

Uploads a file to a group.

```rust
pub async fn post_group_file(
    &self,
    group_openid: &str,
    file_type: u8,
    file_data: &[u8],
) -> Result<FileInfo>
```

### `post_c2c_file`

Uploads a file for C2C messaging.

```rust
pub async fn post_c2c_file(
    &self,
    openid: &str,
    file_type: u8,
    file_data: &[u8],
) -> Result<FileInfo>
```

## Usage Examples

### Basic Message Handling

```rust
use botrs::{EventHandler, Context, Message};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            match content.as_str() {
                "!ping" => {
                    let _ = ctx.send_message(&msg.channel_id, "Pong!").await;
                }
                "!guild" => {
                    if let Some(guild_id) = &msg.guild_id {
                        match ctx.get_guild(guild_id).await {
                            Ok(guild) => {
                                let info = format!("Guild: {} ({} members)", 
                                                  guild.name, guild.member_count);
                                let _ = ctx.send_message(&msg.channel_id, &info).await;
                            }
                            Err(e) => {
                                eprintln!("Failed to get guild info: {}", e);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
```

### Advanced Bot with Rich Features

```rust
use botrs::{EventHandler, Context, Message, MessageEmbed, AudioControl, AudioStatus};

struct AdvancedBot;

#[async_trait::async_trait]
impl EventHandler for AdvancedBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            if content.starts_with("!") {
                let command = &content[1..];
                self.handle_command(&ctx, &msg, command).await;
            }
        }
    }
}

impl AdvancedBot {
    async fn handle_command(&self, ctx: &Context, msg: &Message, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        match parts.get(0) {
            Some(&"status") => {
                let embed = MessageEmbed::new()
                    .title("Bot Status")
                    .description("All systems operational")
                    .color(0x00ff00)
                    .field("Uptime", "24 hours", true)
                    .field("Memory", "512 MB", true);
                
                let _ = ctx.send_message_with_embed(
                    &msg.channel_id,
                    None,
                    &embed
                ).await;
            }
            Some(&"play") => {
                if let Some(&url) = parts.get(1) {
                    let audio_control = AudioControl {
                        audio_url: url.to_string(),
                        text: format!("Now playing: {}", url),
                        status: AudioStatus::Start,
                    };
                    
                    let _ = ctx.update_audio(&msg.channel_id, audio_control).await;
                }
            }
            Some(&"kick") => {
                if let Some(&user_id) = parts.get(1) {
                    if let Some(guild_id) = &msg.guild_id {
                        let _ = ctx.kick_member(
                            guild_id,
                            user_id,
                            false,
                            None,
                            Some("Kicked by moderator")
                        ).await;
                    }
                }
            }
            _ => {
                let _ = ctx.send_message(
                    &msg.channel_id,
                    "Unknown command. Available: !status, !play <url>, !kick <user>"
                ).await;
            }
        }
    }
}
```

## Best Practices

### Error Handling

Always handle errors appropriately in your event handlers:

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    match ctx.send_message(&msg.channel_id, "Hello!").await {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => eprintln!("Failed to send message: {}", e),
    }
}
```

### Performance

- Use the context methods for common operations instead of calling the API directly
- Cache frequently accessed data like guild information
- Avoid blocking operations in event handlers

### Security

- Validate user input before using it in API calls
- Check permissions before performing moderation actions
- Don't expose sensitive information in error messages

## See Also

- [`BotApi`](./bot-api.md) - Direct API access
- [`Client`](./client.md) - Bot client setup
- [`EventHandler`](./event-handler.md) - Event handling
- [`Message Types`](./models/messages.md) - Message structures