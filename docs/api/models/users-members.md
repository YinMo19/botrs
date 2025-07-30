# Users and Members Models API Reference

This module provides data structures for user information, guild members, and related entities within the QQ Guild Bot API.

## Core Types

### `User`

Represents a QQ user in the system.

```rust
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub union_openid: Option<String>,
    pub union_user_account: Option<String>,
}
```

#### Fields

- `id`: Unique user identifier
- `username`: User's display name
- `avatar`: Avatar image URL
- `bot`: Whether this user is a bot account
- `union_openid`: Cross-platform identifier for the user
- `union_user_account`: Union account identifier

#### Example

```rust
async fn handle_user_info(user: User) {
    println!("User: {}", user.username.as_deref().unwrap_or("Unknown"));
    
    if user.bot.unwrap_or(false) {
        println!("This is a bot account");
    }
    
    if let Some(avatar) = &user.avatar {
        println!("Avatar URL: {}", avatar);
    }
}
```

### `Member`

Represents a guild member, containing user information plus guild-specific data.

```rust
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
}
```

#### Fields

- `user`: Base user information
- `nick`: Member's nickname in the guild (overrides username)
- `roles`: List of role IDs assigned to this member
- `joined_at`: ISO 8601 timestamp when the member joined the guild
- `deaf`: Whether the member is server-deafened in voice channels
- `mute`: Whether the member is server-muted in voice channels

#### Methods

The `Member` struct provides convenience methods for common operations:

##### Display Name

```rust
impl Member {
    pub fn display_name(&self) -> &str {
        self.nick.as_deref()
            .or_else(|| self.user.as_ref()?.username.as_deref())
            .unwrap_or("Unknown")
    }
}
```

#### Example

```rust
async fn handle_member_join(ctx: Context, member: Member) {
    let display_name = member.display_name();
    println!("New member joined: {}", display_name);
    
    if let Some(joined_at) = &member.joined_at {
        println!("Joined at: {}", joined_at);
    }
    
    // Check if member has any roles
    if !member.roles.is_empty() {
        println!("Member has {} roles", member.roles.len());
        for role_id in &member.roles {
            println!("  Role ID: {}", role_id);
        }
    }
}
```

### `BotInfo`

Contains information about the bot user itself.

```rust
pub struct BotInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub bot: bool,
    pub application_id: Option<String>,
}
```

#### Fields

- `id`: Bot's user ID
- `username`: Bot's username
- `avatar`: Bot's avatar URL
- `bot`: Always `true` for bot accounts
- `application_id`: Associated application ID

#### Example

```rust
async fn handle_ready(ctx: Context, ready: Ready) {
    if let Some(bot_info) = ctx.bot_info {
        println!("Bot logged in as: {}", bot_info.username);
        println!("Bot ID: {}", bot_info.id);
        
        if let Some(app_id) = &bot_info.application_id {
            println!("Application ID: {}", app_id);
        }
    }
}
```

## Message-Specific User Types

### `MessageUser`

User information in the context of a guild message.

```rust
pub struct MessageUser {
    pub id: String,
    pub username: Option<String>,
    pub bot: Option<bool>,
    pub avatar: Option<String>,
}
```

#### Example

```rust
async fn handle_message(ctx: Context, message: Message) {
    if let Some(author) = &message.author {
        println!("Message from: {}", author.username.as_deref().unwrap_or("Unknown"));
        
        if author.bot.unwrap_or(false) {
            // Ignore bot messages
            return;
        }
    }
}
```

### `DirectMessageUser`

User information in direct messages.

```rust
pub struct DirectMessageUser {
    pub id: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
}
```

### `GroupMessageUser`

User information in group messages, using OpenID for identification.

```rust
pub struct GroupMessageUser {
    pub id: Option<String>,
    pub member_openid: Option<String>,
    pub union_openid: Option<String>,
}
```

#### Fields

- `id`: User ID (may not be available in all contexts)
- `member_openid`: Group-specific member identifier
- `union_openid`: Cross-platform user identifier

### `C2CMessageUser`

User information in client-to-client messages.

```rust
pub struct C2CMessageUser {
    pub user_openid: Option<String>,
}
```

#### Fields

- `user_openid`: User identifier for C2C conversations

## Member Management Operations

### Getting Member Information

```rust
async fn get_member_details(ctx: Context, guild_id: &str, user_id: &str) -> Result<()> {
    // Get specific member
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    
    if let Some(user) = &member.user {
        println!("Member: {}", user.username.as_deref().unwrap_or("Unknown"));
        println!("User ID: {}", user.id);
        
        if let Some(nick) = &member.nick {
            println!("Nickname: {}", nick);
        }
        
        if let Some(joined_at) = &member.joined_at {
            println!("Joined: {}", joined_at);
        }
        
        // Check voice status
        if member.mute.unwrap_or(false) {
            println!("Member is server muted");
        }
        
        if member.deaf.unwrap_or(false) {
            println!("Member is server deafened");
        }
    }
    
    Ok(())
}
```

### Listing Guild Members

```rust
async fn list_guild_members(ctx: Context, guild_id: &str) -> Result<()> {
    let limit = 100;
    let mut after: Option<String> = None;
    let mut total_members = 0;
    
    loop {
        let members = ctx.get_guild_members(guild_id, Some(limit), after.as_deref()).await?;
        
        if members.is_empty() {
            break;
        }
        
        for member in &members {
            total_members += 1;
            
            if let Some(user) = &member.user {
                let display_name = member.nick.as_deref()
                    .unwrap_or(user.username.as_deref().unwrap_or("Unknown"));
                
                println!("{}. {} (ID: {})", total_members, display_name, user.id);
                
                if !member.roles.is_empty() {
                    println!("   Roles: {:?}", member.roles);
                }
            }
        }
        
        // Get the last member's ID for pagination
        if let Some(last_member) = members.last() {
            if let Some(user) = &last_member.user {
                after = Some(user.id.clone());
            }
        }
        
        // If we got fewer than the limit, we've reached the end
        if members.len() < limit as usize {
            break;
        }
    }
    
    println!("Total members: {}", total_members);
    Ok(())
}
```

### Member Role Management

```rust
async fn manage_member_roles(ctx: Context, guild_id: &str, user_id: &str) -> Result<()> {
    // Get current member info
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    println!("Current roles: {:?}", member.roles);
    
    // Add a role
    let role_id = "role_id_to_add";
    ctx.add_guild_role_member(guild_id, role_id, user_id, None).await?;
    println!("Added role {} to user", role_id);
    
    // Wait a moment and then remove the role
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    
    ctx.remove_guild_role_member(guild_id, role_id, user_id, None).await?;
    println!("Removed role {} from user", role_id);
    
    Ok(())
}
```

### Kicking Members

```rust
async fn moderate_member(ctx: Context, guild_id: &str, user_id: &str, reason: &str) -> Result<()> {
    // Get member info before kicking
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    
    if let Some(user) = &member.user {
        println!("Preparing to kick: {}", user.username.as_deref().unwrap_or("Unknown"));
        
        // Kick member with reason
        ctx.kick_member(
            guild_id,
            user_id,
            Some(1), // Add to blacklist for 1 day
            Some(reason),
        ).await?;
        
        println!("Member kicked successfully");
    }
    
    Ok(())
}
```

## Voice Channel Management

### Managing Voice States

```rust
async fn manage_voice_channel(ctx: Context, channel_id: &str) -> Result<()> {
    // Mute all members in voice channel
    ctx.mute_all(channel_id).await?;
    println!("Muted all members in voice channel");
    
    // Wait and then unmute all
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    
    ctx.cancel_mute_all(channel_id).await?;
    println!("Unmuted all members in voice channel");
    
    Ok(())
}
```

### Individual Voice Controls

```rust
async fn control_member_voice(ctx: Context, channel_id: &str, user_id: &str) -> Result<()> {
    // Mute specific member
    ctx.mute_member(
        channel_id,
        user_id,
        Some(300), // Mute for 5 minutes
        Some("Temporary mute for disruption"),
    ).await?;
    
    println!("Muted member for 5 minutes");
    
    // Control microphone access
    ctx.off_microphone(channel_id, user_id).await?;
    println!("Disabled microphone for member");
    
    // Re-enable after some time
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    
    ctx.on_microphone(channel_id, user_id).await?;
    println!("Re-enabled microphone for member");
    
    Ok(())
}
```

## User Identification Patterns

### Handling Different User Types

```rust
fn identify_user_type(message: &Message) -> String {
    if let Some(author) = &message.author {
        if author.bot.unwrap_or(false) {
            return "Bot".to_string();
        }
        
        if author.username.is_some() {
            return "Regular User".to_string();
        }
    }
    
    "Unknown User Type".to_string()
}

async fn handle_different_message_types(ctx: Context) {
    // Guild message
    let handle_guild_msg = |msg: Message| {
        if let Some(author) = &msg.author {
            println!("Guild message from: {}", author.username.as_deref().unwrap_or("Unknown"));
        }
    };
    
    // Group message
    let handle_group_msg = |msg: GroupMessage| {
        if let Some(author) = &msg.author {
            if let Some(openid) = &author.member_openid {
                println!("Group message from OpenID: {}", openid);
            }
        }
    };
    
    // C2C message
    let handle_c2c_msg = |msg: C2CMessage| {
        if let Some(author) = &msg.author {
            if let Some(openid) = &author.user_openid {
                println!("C2C message from OpenID: {}", openid);
            }
        }
    };
}
```

### Cross-Platform User Tracking

```rust
use std::collections::HashMap;

struct UserTracker {
    // Map union_openid to user information
    users_by_openid: HashMap<String, User>,
    // Map guild_id + user_id to member information
    members_by_guild: HashMap<String, HashMap<String, Member>>,
}

impl UserTracker {
    fn new() -> Self {
        Self {
            users_by_openid: HashMap::new(),
            members_by_guild: HashMap::new(),
        }
    }
    
    fn track_user(&mut self, user: User) {
        if let Some(openid) = &user.union_openid {
            self.users_by_openid.insert(openid.clone(), user);
        }
    }
    
    fn track_member(&mut self, guild_id: &str, member: Member) {
        if let Some(user) = &member.user {
            let guild_members = self.members_by_guild
                .entry(guild_id.to_string())
                .or_insert_with(HashMap::new);
            
            guild_members.insert(user.id.clone(), member);
            
            // Also track the user globally
            if let Some(user) = member.user.clone() {
                self.track_user(user);
            }
        }
    }
    
    fn find_user_by_openid(&self, openid: &str) -> Option<&User> {
        self.users_by_openid.get(openid)
    }
    
    fn find_member(&self, guild_id: &str, user_id: &str) -> Option<&Member> {
        self.members_by_guild
            .get(guild_id)?
            .get(user_id)
    }
}
```

## Common Usage Patterns

### Member Verification System

```rust
async fn verify_new_member(ctx: Context, guild_id: &str, member: Member) -> Result<()> {
    if let Some(user) = &member.user {
        println!("Verifying new member: {}", user.username.as_deref().unwrap_or("Unknown"));
        
        // Check if it's a bot
        if user.bot.unwrap_or(false) {
            println!("Bot account detected - applying bot role");
            
            let bot_role_id = "bot_role_id";
            ctx.add_guild_role_member(guild_id, bot_role_id, &user.id, None).await?;
        } else {
            // Apply default member role
            let member_role_id = "member_role_id";
            ctx.add_guild_role_member(guild_id, member_role_id, &user.id, None).await?;
            
            println!("Applied default member role");
        }
        
        // Log join information
        if let Some(joined_at) = &member.joined_at {
            println!("Member joined at: {}", joined_at);
        }
    }
    
    Ok(())
}
```

### Permission-Based Actions

```rust
async fn execute_moderation_action(
    ctx: Context,
    guild_id: &str,
    moderator_id: &str,
    target_id: &str,
    action: &str,
) -> Result<()> {
    // Get moderator member info
    let moderator = ctx.get_guild_member(guild_id, moderator_id).await?;
    
    // Check if moderator has required role
    let mod_role_id = "moderator_role_id";
    if !moderator.roles.contains(&mod_role_id.to_string()) {
        println!("User lacks moderation permissions");
        return Ok(());
    }
    
    // Get target member info
    let target = ctx.get_guild_member(guild_id, target_id).await?;
    
    match action {
        "kick" => {
            ctx.kick_member(guild_id, target_id, Some(1), Some("Moderation action")).await?;
            println!("Member kicked by moderator");
        }
        "mute" => {
            // Find a voice channel the target is in
            // This would require additional voice state tracking
            println!("Mute action requested");
        }
        _ => {
            println!("Unknown moderation action: {}", action);
        }
    }
    
    Ok(())
}
```

## See Also

- [Client API](../client.md) - Main client for user and member operations
- [Context API](../context.md) - Context object for API access  
- [Messages](./messages.md) - Message types and user context
- [Guilds & Channels](./guilds-channels.md) - Guild and channel management