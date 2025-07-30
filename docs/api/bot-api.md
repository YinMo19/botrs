# BotApi API Reference

The `BotApi` struct provides direct access to QQ Guild's REST API endpoints. It handles HTTP requests, authentication, and response parsing for all bot operations.

## Overview

```rust
use botrs::{BotApi, Token, Result};

// Create API client
let api = BotApi::new(http_client);

// Use with token
let token = Token::new("app_id", "secret");
let bot_info = api.get_bot_info(&token).await?;
```

The `BotApi` is the core interface for making HTTP requests to QQ Guild's API. It provides methods for:

- Bot information and authentication
- Guild and channel management
- Message operations
- Member management
- Audio and voice controls
- File uploads and media
- Permissions and roles

## Constructor

### `new`

Creates a new BotApi instance.

```rust
pub fn new(http: HttpClient) -> Self
```

#### Parameters

- `http`: The HTTP client to use for requests

#### Example

```rust
use botrs::{BotApi, HttpClient};

let http = HttpClient::new(30, false)?;
let api = BotApi::new(http);
```

## Bot Information

### `get_bot_info`

Gets information about the current bot.

```rust
pub async fn get_bot_info(&self, token: &Token) -> Result<BotInfo>
```

#### Parameters

- `token`: Authentication token

#### Returns

Bot information including username, ID, and other details.

#### Example

```rust
let bot_info = api.get_bot_info(&token).await?;
println!("Bot: {} (ID: {})", bot_info.username, bot_info.id);
```

### `get_gateway`

Gets the WebSocket gateway URL for real-time events.

```rust
pub async fn get_gateway(&self, token: &Token) -> Result<GatewayResponse>
```

#### Parameters

- `token`: Authentication token

#### Returns

Gateway information including WebSocket URL and connection details.

#### Example

```rust
let gateway = api.get_gateway(&token).await?;
println!("Gateway URL: {}", gateway.url);
```

## Guild Operations

### `get_guild`

Gets detailed information about a specific guild.

```rust
pub async fn get_guild(&self, token: &Token, guild_id: &str) -> Result<Guild>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild ID to retrieve

#### Returns

Complete guild information including channels, roles, and settings.

#### Example

```rust
let guild = api.get_guild(&token, "guild_123").await?;
println!("Guild: {} with {} members", guild.name, guild.member_count);
```

### `get_guilds`

Gets a list of guilds the bot is in.

```rust
pub async fn get_guilds(&self, token: &Token) -> Result<Vec<Guild>>
```

#### Parameters

- `token`: Authentication token

#### Returns

List of guilds the bot has access to.

#### Example

```rust
let guilds = api.get_guilds(&token).await?;
for guild in guilds {
    println!("Guild: {} ({})", guild.name, guild.id);
}
```

## Channel Operations

### `get_channel`

Gets information about a specific channel.

```rust
pub async fn get_channel(&self, token: &Token, channel_id: &str) -> Result<Channel>
```

#### Parameters

- `token`: Authentication token
- `channel_id`: The channel ID to retrieve

#### Returns

Channel information including name, type, and permissions.

#### Example

```rust
let channel = api.get_channel(&token, "channel_123").await?;
println!("Channel: {} (Type: {:?})", channel.name, channel.type_);
```

### `get_channels`

Gets all channels in a guild.

```rust
pub async fn get_channels(&self, token: &Token, guild_id: &str) -> Result<Vec<Channel>>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild ID to get channels from

#### Returns

List of channels in the guild.

#### Example

```rust
let channels = api.get_channels(&token, "guild_123").await?;
for channel in channels {
    println!("Channel: {} ({})", channel.name, channel.id);
}
```

### `create_channel`

Creates a new channel in a guild.

```rust
pub async fn create_channel(
    &self,
    token: &Token,
    guild_id: &str,
    channel: CreateChannel,
) -> Result<Channel>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild to create the channel in
- `channel`: Channel creation parameters

#### Returns

The created channel information.

#### Example

```rust
use botrs::{CreateChannel, ChannelType, ChannelSubType};

let new_channel = CreateChannel {
    name: "new-text-channel".to_string(),
    type_: ChannelType::Text,
    sub_type: ChannelSubType::Chat,
    position: None,
    parent_id: None,
    private_type: None,
    private_user_ids: None,
    speak_permission: None,
    application_id: None,
};

let channel = api.create_channel(&token, "guild_123", new_channel).await?;
println!("Created channel: {}", channel.name);
```

### `update_channel`

Updates an existing channel.

```rust
pub async fn update_channel(
    &self,
    token: &Token,
    channel_id: &str,
    update: UpdateChannel,
) -> Result<Channel>
```

#### Parameters

- `token`: Authentication token
- `channel_id`: The channel to update
- `update`: Channel update parameters

#### Returns

The updated channel information.

### `delete_channel`

Deletes a channel.

```rust
pub async fn delete_channel(&self, token: &Token, channel_id: &str) -> Result<()>
```

#### Parameters

- `token`: Authentication token
- `channel_id`: The channel to delete

#### Example

```rust
api.delete_channel(&token, "channel_123").await?;
println!("Channel deleted");
```

## Message Operations

### `get_message`

Gets a specific message by ID.

```rust
pub async fn get_message(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
) -> Result<Message>
```

#### Parameters

- `token`: Authentication token
- `channel_id`: The channel containing the message
- `message_id`: The message ID to retrieve

#### Returns

The message data.

#### Example

```rust
let message = api.get_message(&token, "channel_123", "msg_456").await?;
println!("Message: {}", message.content.unwrap_or_default());
```

### `post_message_with_params`

Sends a message using structured parameters.

```rust
pub async fn post_message_with_params(
    &self,
    token: &Token,
    channel_id: &str,
    params: MessageParams,
) -> Result<Message>
```

#### Parameters

- `token`: Authentication token
- `channel_id`: The channel to send the message to
- `params`: Message parameters including content, embeds, files, etc.

#### Returns

The sent message data.

#### Example

```rust
use botrs::MessageParams;

let params = MessageParams::new_text("Hello, world!")
    .with_reply("original_msg_id")
    .with_markdown(true);

let message = api.post_message_with_params(&token, "channel_123", params).await?;
println!("Message sent: {}", message.id);
```

### `post_message`

Sends a simple text message (legacy method).

```rust
pub async fn post_message(
    &self,
    token: &Token,
    channel_id: &str,
    content: Option<&str>,
    embed: Option<MessageEmbed>,
    ark: Option<MessageArk>,
    reference: Option<MessageReference>,
    image: Option<&str>,
    msg_id: Option<&str>,
    event_id: Option<&str>,
    markdown: Option<MarkdownParams>,
    keyboard: Option<MessageKeyboard>,
    media: Option<FileInfo>,
) -> Result<Message>
```

**Note**: This method is deprecated. Use `post_message_with_params` instead.

### `recall_message`

Recalls (deletes) a message.

```rust
pub async fn recall_message(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    hidetip: bool,
) -> Result<()>
```

#### Parameters

- `token`: Authentication token
- `channel_id`: The channel containing the message
- `message_id`: The message to recall
- `hidetip`: Whether to hide the deletion notification

#### Example

```rust
api.recall_message(&token, "channel_123", "msg_456", false).await?;
println!("Message recalled");
```

## Direct Messages

### `create_dms`

Creates a direct message session.

```rust
pub async fn create_dms(
    &self,
    token: &Token,
    create_dms: CreateDirectMessageGuild,
) -> Result<DirectMessageGuild>
```

#### Parameters

- `token`: Authentication token
- `create_dms`: Parameters for creating the DM session

#### Returns

Direct message guild information.

#### Example

```rust
use botrs::CreateDirectMessageGuild;

let create_dm = CreateDirectMessageGuild {
    recipient_id: "user_123".to_string(),
    source_guild_id: "guild_456".to_string(),
};

let dm_guild = api.create_dms(&token, create_dm).await?;
println!("DM session created: {}", dm_guild.guild_id);
```

### `post_dms_with_params`

Sends a direct message using structured parameters.

```rust
pub async fn post_dms_with_params(
    &self,
    token: &Token,
    guild_id: &str,
    params: MessageParams,
) -> Result<Message>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The DM guild ID
- `params`: Message parameters

#### Returns

The sent direct message.

## Group and C2C Messages

### `post_group_message_with_params`

Sends a group message using structured parameters.

```rust
pub async fn post_group_message_with_params(
    &self,
    token: &Token,
    group_openid: &str,
    params: GroupMessageParams,
) -> Result<Message>
```

#### Parameters

- `token`: Authentication token
- `group_openid`: The group identifier
- `params`: Group message parameters

#### Returns

The sent group message.

### `post_c2c_message_with_params`

Sends a C2C (client-to-client) message using structured parameters.

```rust
pub async fn post_c2c_message_with_params(
    &self,
    token: &Token,
    openid: &str,
    params: C2CMessageParams,
) -> Result<Message>
```

#### Parameters

- `token`: Authentication token
- `openid`: The user identifier
- `params`: C2C message parameters

#### Returns

The sent C2C message.

## Member Management

### `get_guild_member`

Gets information about a specific guild member.

```rust
pub async fn get_guild_member(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
) -> Result<Member>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild ID
- `user_id`: The user ID

#### Returns

Member information including roles and permissions.

#### Example

```rust
let member = api.get_guild_member(&token, "guild_123", "user_456").await?;
println!("Member: {}", member.nick.unwrap_or_default());
```

### `get_guild_members`

Gets a list of guild members with pagination.

```rust
pub async fn get_guild_members(
    &self,
    token: &Token,
    guild_id: &str,
    query: Option<&MemberQuery>,
) -> Result<Vec<Member>>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild ID
- `query`: Optional query parameters for pagination and filtering

#### Returns

List of guild members.

#### Example

```rust
use botrs::MemberQuery;

let query = MemberQuery {
    after: None,
    limit: Some(100),
};

let members = api.get_guild_members(&token, "guild_123", Some(&query)).await?;
println!("Found {} members", members.len());
```

### `delete_member`

Removes a member from the guild.

```rust
pub async fn delete_member(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    add_blacklist: bool,
    delete_history_msg_days: Option<u8>,
    reason: Option<&str>,
) -> Result<()>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild ID
- `user_id`: The user to remove
- `add_blacklist`: Whether to add to blacklist
- `delete_history_msg_days`: Days of message history to delete
- `reason`: Reason for removal

#### Example

```rust
api.delete_member(
    &token,
    "guild_123",
    "user_456",
    false,
    Some(7),
    Some("Violated community guidelines")
).await?;
```

## Role Management

### `get_guild_roles`

Gets all roles in a guild.

```rust
pub async fn get_guild_roles(&self, token: &Token, guild_id: &str) -> Result<GuildRoles>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild ID

#### Returns

Guild roles information.

#### Example

```rust
let roles = api.get_guild_roles(&token, "guild_123").await?;
for role in &roles.roles {
    println!("Role: {} (ID: {})", role.name, role.id);
}
```

### `create_guild_role`

Creates a new role in a guild.

```rust
pub async fn create_guild_role(
    &self,
    token: &Token,
    guild_id: &str,
    role: CreateRole,
) -> Result<CreateRoleResponse>
```

#### Parameters

- `token`: Authentication token
- `guild_id`: The guild ID
- `role`: Role creation parameters

#### Returns

The created role information.

#### Example

```rust
use botrs::CreateRole;

let new_role = CreateRole {
    name: "Moderator".to_string(),
    color: Some(0x9932cc),
    hoist: Some(true),
    mentionable: Some(true),
};

let role = api.create_guild_role(&token, "guild_123", new_role).await?;
println!("Created role: {}", role.role.name);
```

### `update_guild_role`

Updates an existing guild role.

```rust
pub async fn update_guild_role(
    &self,
    token: &Token,
    guild_id: &str,
    role_id: &str,
    role: UpdateRole,
) -> Result<UpdateRoleResponse>
```

### `delete_guild_role`

Deletes a guild role.

```rust
pub async fn delete_guild_role(
    &self,
    token: &Token,
    guild_id: &str,
    role_id: &str,
) -> Result<()>
```

### `create_guild_role_member`

Assigns a role to a guild member.

```rust
pub async fn create_guild_role_member(
    &self,
    token: &Token,
    guild_id: &str,
    role_id: &str,
    user_id: &str,
    channel: Option<MemberAddRoleChannel>,
) -> Result<()>
```

### `delete_guild_role_member`

Removes a role from a guild member.

```rust
pub async fn delete_guild_role_member(
    &self,
    token: &Token,
    guild_id: &str,
    role_id: &str,
    user_id: &str,
    channel: Option<MemberAddRoleChannel>,
) -> Result<()>
```

## Audio and Voice

### `update_audio`

Updates audio playback in a voice channel.

```rust
pub async fn update_audio(
    &self,
    token: &Token,
    channel_id: &str,
    audio_control: AudioControl,
) -> Result<()>
```

#### Parameters

- `token`: Authentication token
- `channel_id`: The voice channel ID
- `audio_control`: Audio control parameters

#### Example

```rust
use botrs::{AudioControl, AudioStatus};

let audio_control = AudioControl {
    audio_url: "https://example.com/audio.mp3".to_string(),
    text: "Now playing music".to_string(),
    status: AudioStatus::Start,
};

api.update_audio(&token, "channel_123", audio_control).await?;
```

### `on_microphone`

Enables microphone for a user in voice channel.

```rust
pub async fn on_microphone(
    &self,
    token: &Token,
    channel_id: &str,
    user_id: &str,
) -> Result<()>
```

### `off_microphone`

Disables microphone for a user in voice channel.

```rust
pub async fn off_microphone(
    &self,
    token: &Token,
    channel_id: &str,
    user_id: &str,
) -> Result<()>
```

### `mute_all`

Mutes all users in a voice channel.

```rust
pub async fn mute_all(&self, token: &Token, channel_id: &str) -> Result<()>
```

### `cancel_mute_all`

Unmutes all users in a voice channel.

```rust
pub async fn cancel_mute_all(&self, token: &Token, channel_id: &str) -> Result<()>
```

### `mute_member`

Mutes a specific member in a voice channel.

```rust
pub async fn mute_member(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    mute_end_timestamp: Option<&str>,
    mute_seconds: Option<&str>,
) -> Result<()>
```

## File Operations

### `post_group_file`

Uploads a file to a group.

```rust
pub async fn post_group_file(
    &self,
    token: &Token,
    group_openid: &str,
    file_type: u8,
    file_data: &[u8],
) -> Result<FileInfo>
```

#### Parameters

- `token`: Authentication token
- `group_openid`: The group identifier
- `file_type`: Type of file being uploaded
- `file_data`: The file content as bytes

#### Returns

Information about the uploaded file.

### `post_c2c_file`

Uploads a file for C2C messaging.

```rust
pub async fn post_c2c_file(
    &self,
    token: &Token,
    openid: &str,
    file_type: u8,
    file_data: &[u8],
) -> Result<FileInfo>
```

## Permissions

### `get_permissions`

Gets API permissions for the bot.

```rust
pub async fn get_permissions(&self, token: &Token, guild_id: &str) -> Result<ApiPermissions>
```

### `post_permission_demand`

Requests additional API permissions.

```rust
pub async fn post_permission_demand(
    &self,
    token: &Token,
    guild_id: &str,
    demand: PermissionDemandRequest,
) -> Result<()>
```

### `get_channel_user_permissions`

Gets user permissions for a specific channel.

```rust
pub async fn get_channel_user_permissions(
    &self,
    token: &Token,
    channel_id: &str,
    user_id: &str,
) -> Result<ChannelPermissions>
```

### `get_channel_role_permissions`

Gets role permissions for a specific channel.

```rust
pub async fn get_channel_role_permissions(
    &self,
    token: &Token,
    channel_id: &str,
    role_id: &str,
) -> Result<ChannelPermissions>
```

## Reactions and Pins

### `put_reaction`

Adds a reaction to a message.

```rust
pub async fn put_reaction(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: ReactionEmoji,
) -> Result<()>
```

### `delete_reaction`

Removes a reaction from a message.

```rust
pub async fn delete_reaction(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: ReactionEmoji,
) -> Result<()>
```

### `get_reaction_users`

Gets users who reacted with a specific emoji.

```rust
pub async fn get_reaction_users(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: ReactionEmoji,
    query: Option<ReactionUsersQuery>,
) -> Result<ReactionUsers>
```

### `put_pin`

Pins a message in a channel.

```rust
pub async fn put_pin(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
) -> Result<PinMessage>
```

### `delete_pin`

Unpins a message in a channel.

```rust
pub async fn delete_pin(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
) -> Result<()>
```

### `get_pins`

Gets all pinned messages in a channel.

```rust
pub async fn get_pins(&self, token: &Token, channel_id: &str) -> Result<PinMessages>
```

## Utility Methods

### `http`

Gets a reference to the underlying HTTP client.

```rust
pub fn http(&self) -> &HttpClient
```

### `close`

Closes the API client and releases resources.

```rust
pub async fn close(&self)
```

## Error Handling

All API methods return `Result<T, BotError>`. Common error scenarios include:

- **Authentication errors**: Invalid token or insufficient permissions
- **Rate limiting**: Too many requests in a short time
- **Not found errors**: Resource doesn't exist
- **Network errors**: Connection problems or timeouts

### Example Error Handling

```rust
use botrs::BotError;

match api.get_guild(&token, "invalid_guild").await {
    Ok(guild) => println!("Guild: {}", guild.name),
    Err(BotError::NotFound(msg)) => println!("Guild not found: {}", msg),
    Err(BotError::RateLimit { retry_after }) => {
        println!("Rate limited, retry after {} seconds", retry_after);
    }
    Err(e) => eprintln!("API error: {}", e),
}
```

## Best Practices

### Performance

1. **Reuse API instances**: Create one `BotApi` instance and reuse it
2. **Handle rate limits**: Implement proper backoff for rate-limited requests
3. **Use structured parameters**: Prefer `*_with_params` methods over legacy variants
4. **Batch operations**: Group related API calls when possible

### Error Handling

1. **Check permissions**: Verify bot has necessary permissions before API calls
2. **Validate inputs**: Check parameters before making requests
3. **Implement retries**: Retry transient failures with exponential backoff
4. **Log errors**: Record API errors for debugging and monitoring

### Security

1. **Protect tokens**: Never log or expose authentication tokens
2. **Validate user input**: Sanitize user-provided data before API calls
3. **Respect permissions**: Don't attempt operations the bot isn't authorized for
4. **Rate limit protection**: Implement client-side rate limiting

## See Also

- [`Client`](./client.md) - High-level bot client
- [`Context`](./context.md) - API access in event handlers
- [`Message Types`](./models/messages.md) - Message data structures
- [`Error Types`](./error-types.md) - Error handling and types