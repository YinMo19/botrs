# Other Types API Reference

This module covers additional data structures and utility types used throughout the QQ Guild Bot API.

## Audio and Voice Types

### `Audio`

Represents audio controls and status in voice channels.

```rust
pub struct Audio {
    pub audio_control: Option<AudioControl>,
    pub audio_status: Option<AudioStatus>,
}
```

#### Fields

- `audio_control`: Control actions for audio playback
- `audio_status`: Current status of audio in the channel

### `AudioControl`

Audio control operations.

```rust
pub struct AudioControl {
    pub audio_url: Option<String>,
    pub text: Option<String>,
    pub status: Option<u32>,
}
```

#### Fields

- `audio_url`: URL of the audio file to play
- `text`: Text description of the audio
- `status`: Audio playback status (0: pause, 1: play)

### `AudioStatus`

Current audio playback status.

```rust
pub struct AudioStatus {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: Option<u32>,
}
```

#### Fields

- `start_time`: When audio playback started
- `end_time`: When audio playback ended
- `status`: Current playback status

### `PublicAudio`

Public audio channel information.

```rust
pub struct PublicAudio {
    pub audio_type: Option<PublicAudioType>,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
    pub user_id: Option<String>,
}
```

#### Fields

- `audio_type`: Type of audio event (enter/exit)
- `channel_id`: ID of the audio channel
- `guild_id`: ID of the guild
- `user_id`: ID of the user involved

### `PublicAudioType`

Audio channel event types.

```rust
pub enum PublicAudioType {
    Enter = 1,
    Exit = 2,
}
```

#### Variants

- `Enter`: User entered an audio channel
- `Exit`: User left an audio channel

## Forum and Thread Types

### `Thread`

Represents a forum thread.

```rust
pub struct Thread {
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    pub thread_info: ThreadInfo,
}
```

#### Fields

- `guild_id`: ID of the guild containing the thread
- `channel_id`: ID of the forum channel
- `author_id`: ID of the thread author
- `thread_info`: Detailed thread information

### `ThreadInfo`

Detailed information about a thread.

```rust
pub struct ThreadInfo {
    pub thread_id: String,
    pub title: String,
    pub content: String,
    pub date_time: String,
}
```

#### Fields

- `thread_id`: Unique thread identifier
- `title`: Thread title
- `content`: Thread content/description
- `date_time`: Thread creation timestamp

### `OpenThread`

Open forum thread with additional metadata.

```rust
pub struct OpenThread {
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    pub thread_info: ThreadInfo,
    pub task_id: Option<String>,
    pub event_id: Option<String>,
}
```

#### Fields

- `guild_id`: Guild ID
- `channel_id`: Channel ID
- `author_id`: Author ID
- `thread_info`: Thread information
- `task_id`: Associated task ID
- `event_id`: Event ID for tracking

## Permission Types

### `Permission`

Represents permissions for channels and roles.

```rust
pub struct Permission {
    pub channel_id: String,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub permissions: String,
}
```

#### Fields

- `channel_id`: Channel these permissions apply to
- `user_id`: User ID (for user-specific permissions)
- `role_id`: Role ID (for role-based permissions)
- `permissions`: Permission bit flags as string

### `ChannelPermissions`

Channel-specific permission configuration.

```rust
pub struct ChannelPermissions {
    pub channel_id: String,
    pub permissions: String,
}
```

## Schedule Types

### `Schedule`

Represents a scheduled event or task.

```rust
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_timestamp: String,
    pub end_timestamp: String,
    pub creator: Option<Member>,
    pub jump_channel_id: Option<String>,
    pub remind_type: String,
}
```

#### Fields

- `id`: Unique schedule identifier
- `name`: Schedule name/title
- `description`: Optional description
- `start_timestamp`: When the event starts
- `end_timestamp`: When the event ends
- `creator`: Member who created the schedule
- `jump_channel_id`: Channel to direct users to
- `remind_type`: Type of reminder notifications

## Interaction Types

### `Interaction`

Represents user interactions with bot components.

```rust
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user: Option<User>,
    pub member: Option<Member>,
    pub token: String,
    pub version: u32,
}
```

#### Fields

- `id`: Unique interaction identifier
- `application_id`: Bot application ID
- `interaction_type`: Type of interaction
- `data`: Interaction-specific data
- `guild_id`: Guild where interaction occurred
- `channel_id`: Channel where interaction occurred
- `user`: User who triggered the interaction
- `member`: Member information if in guild
- `token`: Interaction token for responses
- `version`: API version

### `InteractionType`

Types of user interactions.

```rust
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
}
```

#### Variants

- `Ping`: Ping interaction for verification
- `ApplicationCommand`: Slash command execution
- `MessageComponent`: Button or select menu interaction

### `InteractionData`

Data payload for interactions.

```rust
pub struct InteractionData {
    pub data_type: InteractionDataType,
    pub resolved: Option<serde_json::Value>,
}
```

#### Fields

- `data_type`: Type of interaction data
- `resolved`: Resolved entities (users, roles, channels, etc.)

## Reaction Types

### `Reaction`

Represents emoji reactions on messages.

```rust
pub struct Reaction {
    pub target: ReactionTarget,
    pub emoji_type: u32,
    pub emoji_id: String,
}
```

#### Fields

- `target`: What the reaction is applied to
- `emoji_type`: Type of emoji (1: system, 2: custom)
- `emoji_id`: Emoji identifier

### `ReactionTarget`

Target for emoji reactions.

```rust
pub struct ReactionTarget {
    pub target_type: ReactionTargetType,
    pub id: String,
}
```

#### Fields

- `target_type`: Type of target (message, etc.)
- `id`: Target identifier

### `ReactionTargetType`

Types of reaction targets.

```rust
pub enum ReactionTargetType {
    Message = 0,
}
```

### `ReactionUsers`

Users who reacted with a specific emoji.

```rust
pub struct ReactionUsers {
    pub users: Vec<User>,
    pub cookie: Option<String>,
    pub is_end: Option<bool>,
}
```

#### Fields

- `users`: List of users who reacted
- `cookie`: Pagination token
- `is_end`: Whether this is the last page

## Management Event Types

### `C2CManageEvent`

Client-to-client management events.

```rust
pub struct C2CManageEvent {
    pub event_type: ManageEventType,
    pub timestamp: u64,
    pub openid: String,
}
```

#### Fields

- `event_type`: Type of management event
- `timestamp`: Event timestamp
- `openid`: User's OpenID

### `GroupManageEvent`

Group management events.

```rust
pub struct GroupManageEvent {
    pub event_type: ManageEventType,
    pub timestamp: u64,
    pub group_openid: String,
    pub op_member_openid: String,
}
```

#### Fields

- `event_type`: Type of management event
- `timestamp`: Event timestamp
- `group_openid`: Group's OpenID
- `op_member_openid`: Operating member's OpenID

### `ManageEventType`

Types of management events.

```rust
pub enum ManageEventType {
    FriendAdd = 11001,
    FriendDel = 11002,
    C2cMsgReject = 11003,
    C2cMsgReceive = 11004,
    GroupAddRobot = 12001,
    GroupDelRobot = 12002,
    GroupMsgReject = 12003,
    GroupMsgReceive = 12004,
}
```

## Gateway Types

### `Ready`

Ready event data when bot connects.

```rust
pub struct Ready {
    pub version: u32,
    pub session_id: String,
    pub user: BotInfo,
    pub shard: Option<[u32; 2]>,
}
```

#### Fields

- `version`: Gateway version
- `session_id`: Session identifier for this connection
- `user`: Bot user information
- `shard`: Shard information [shard_id, shard_count]

### `ConnectionSession`

Session information for the gateway connection.

```rust
pub struct ConnectionSession {
    pub session_id: String,
    pub shard_id: u32,
    pub shard_count: u32,
    pub last_sequence: Option<u64>,
}
```

#### Fields

- `session_id`: Unique session identifier
- `shard_id`: Current shard ID (0-based)
- `shard_count`: Total number of shards
- `last_sequence`: Last received sequence number

### `ConnectionState`

Current state of the gateway connection.

```rust
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Closing,
}
```

#### Variants

- `Disconnected`: Not connected to gateway
- `Connecting`: Attempting to connect
- `Connected`: Successfully connected and ready
- `Reconnecting`: Attempting to reconnect after disconnect
- `Closing`: Gracefully closing connection

## Utility Types

### `HasId`

Trait for types that have an ID field.

```rust
pub trait HasId {
    fn id(&self) -> &str;
}
```

This trait is implemented by most entities (messages, users, guilds, etc.) to provide a consistent way to access their identifier.

### `Session`

Session management utilities.

```rust
pub struct Session {
    pub url: String,
    pub shards: u32,
    pub session_start_limit: SessionStartLimit,
}
```

### `SessionStartLimit`

Limits on session starts.

```rust
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u64,
    pub max_concurrency: u32,
}
```

## Common Usage Patterns

### Audio Channel Management

```rust
async fn manage_audio_channel(ctx: Context, channel_id: &str) -> Result<()> {
    // Play audio in channel
    let audio_control = AudioControl {
        audio_url: Some("https://example.com/audio.mp3".to_string()),
        text: Some("Now playing background music".to_string()),
        status: Some(1), // Play
    };
    
    ctx.update_audio(channel_id, &audio_control).await?;
    println!("Started audio playback");
    
    // Wait and then stop
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    
    let stop_control = AudioControl {
        audio_url: None,
        text: Some("Audio stopped".to_string()),
        status: Some(0), // Pause
    };
    
    ctx.update_audio(channel_id, &stop_control).await?;
    println!("Stopped audio playback");
    
    Ok(())
}
```

### Forum Thread Creation

```rust
async fn create_forum_thread(
    ctx: Context,
    channel_id: &str,
    title: &str,
    content: &str,
) -> Result<OpenThread> {
    use botrs::forum::{Title, Content, Format};
    
    let thread_title = Title {
        text: title.to_string(),
        paragraphs: vec![],
    };
    
    let thread_content = Content {
        paragraphs: vec![content.to_string()],
    };
    
    let thread = ctx.api.create_thread(
        &ctx.token,
        channel_id,
        &thread_title,
        &thread_content,
        &Format::Text,
    ).await?;
    
    println!("Created forum thread: {}", thread.thread_info.title);
    Ok(thread)
}
```

### Reaction Management

```rust
async fn manage_reactions(ctx: Context, channel_id: &str, message_id: &str) -> Result<()> {
    // Add reaction
    let reaction = Reaction {
        target: ReactionTarget {
            target_type: ReactionTargetType::Message,
            id: message_id.to_string(),
        },
        emoji_type: 1, // System emoji
        emoji_id: "128077".to_string(), // Thumbs up
    };
    
    ctx.add_reaction(channel_id, message_id, &reaction).await?;
    println!("Added thumbs up reaction");
    
    // Get users who reacted
    let reaction_users = ctx.get_reaction_users(
        channel_id,
        message_id,
        &reaction,
        None, // cookie
        Some(50), // limit
    ).await?;
    
    println!("Users who reacted:");
    for user in reaction_users.users {
        println!("  - {}", user.username.as_deref().unwrap_or("Unknown"));
    }
    
    // Remove reaction
    ctx.remove_reaction(channel_id, message_id, &reaction).await?;
    println!("Removed reaction");
    
    Ok(())
}
```

### Schedule Management

```rust
async fn create_schedule(
    ctx: Context,
    channel_id: &str,
    name: &str,
    description: &str,
    start_time: &str,
    end_time: &str,
) -> Result<()> {
    let schedule = Schedule {
        id: String::new(), // Will be assigned by server
        name: name.to_string(),
        description: Some(description.to_string()),
        start_timestamp: start_time.to_string(),
        end_timestamp: end_time.to_string(),
        creator: None,
        jump_channel_id: Some(channel_id.to_string()),
        remind_type: "1".to_string(), // Remind before start
    };
    
    // Note: Actual schedule creation would need appropriate API call
    println!("Schedule created: {}", schedule.name);
    Ok(())
}
```

### Permission Checking

```rust
async fn check_user_permissions(
    ctx: Context,
    channel_id: &str,
    user_id: &str,
) -> Result<bool> {
    let permissions = ctx.get_channel_user_permissions(channel_id, user_id).await?;
    
    // Parse permission string (this is simplified)
    let perm_value: u64 = permissions.permissions.parse().unwrap_or(0);
    
    // Check for specific permissions (bit flags)
    const SEND_MESSAGES: u64 = 1 << 11;
    const READ_MESSAGE_HISTORY: u64 = 1 << 16;
    
    let can_send = (perm_value & SEND_MESSAGES) != 0;
    let can_read_history = (perm_value & READ_MESSAGE_HISTORY) != 0;
    
    println!("User permissions in channel:");
    println!("  Can send messages: {}", can_send);
    println!("  Can read history: {}", can_read_history);
    
    Ok(can_send && can_read_history)
}
```

## See Also

- [Client API](../client.md) - Main client for bot operations
- [Messages](./messages.md) - Message types and handling
- [Guilds & Channels](./guilds-channels.md) - Guild and channel management
- [Users & Members](./users-members.md) - User and member management