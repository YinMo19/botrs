# Guilds and Channels Models API Reference

This module provides data structures for guilds (servers) and channels within the QQ Guild Bot API.

## Guild Types

### `Guild`

Represents a QQ guild (server) that the bot has access to.

```rust
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner_id: String,
    pub owner: bool,
    pub member_count: Option<u32>,
    pub max_members: Option<u32>,
    pub description: Option<String>,
    pub joined_at: Option<String>,
    pub features: Vec<String>,
    pub op_user_id: Option<String>,
}
```

#### Fields

- `id`: Unique identifier for the guild
- `name`: Guild name
- `icon`: Guild icon URL (optional)
- `owner_id`: User ID of the guild owner
- `owner`: Whether the bot is the owner of the guild
- `member_count`: Current number of members
- `max_members`: Maximum allowed members
- `description`: Guild description
- `joined_at`: Timestamp when the bot joined the guild
- `features`: List of guild features enabled
- `op_user_id`: Operator user ID

#### Example

```rust
async fn handle_guild_create(ctx: Context, guild: Guild) {
    println!("Joined guild: {} (ID: {})", guild.name, guild.id);
    println!("Member count: {}", guild.member_count.unwrap_or(0));
    
    if guild.owner {
        println!("Bot is the owner of this guild");
    }
}
```

### `GuildRole`

Represents a role within a guild.

```rust
pub struct GuildRole {
    pub id: String,
    pub name: String,
    pub color: Option<u32>,
    pub hoist: Option<bool>,
    pub number: Option<u32>,
    pub member_limit: Option<u32>,
}
```

#### Fields

- `id`: Unique identifier for the role
- `name`: Role name
- `color`: Role color (as hex value)
- `hoist`: Whether the role is displayed separately in member list
- `number`: Role position/priority
- `member_limit`: Maximum number of members that can have this role

#### Example

```rust
async fn list_guild_roles(ctx: Context, guild_id: &str) -> Result<()> {
    let roles = ctx.get_guild_roles(guild_id).await?;
    
    for role in roles.roles {
        println!("Role: {} (ID: {})", role.name, role.id);
        if let Some(color) = role.color {
            println!("  Color: #{:06X}", color);
        }
        if let Some(limit) = role.member_limit {
            println!("  Member limit: {}", limit);
        }
    }
    
    Ok(())
}
```

### `GuildRoles`

Container for guild role information.

```rust
pub struct GuildRoles {
    pub guild_id: String,
    pub roles: Vec<GuildRole>,
    pub role_num_limit: Option<u32>,
}
```

#### Fields

- `guild_id`: ID of the guild these roles belong to
- `roles`: List of roles in the guild
- `role_num_limit`: Maximum number of roles allowed in the guild

## Channel Types

### `Channel`

Represents a channel within a guild.

```rust
pub struct Channel {
    pub id: String,
    pub guild_id: String,
    pub name: String,
    pub channel_type: ChannelType,
    pub sub_type: Option<ChannelSubType>,
    pub position: Option<u32>,
    pub parent_id: Option<String>,
    pub owner_id: Option<String>,
    pub private_type: Option<u32>,
    pub speak_permission: Option<u32>,
    pub application_id: Option<String>,
    pub permissions: Option<String>,
}
```

#### Fields

- `id`: Unique identifier for the channel
- `guild_id`: ID of the guild this channel belongs to
- `name`: Channel name
- `channel_type`: Type of channel (text, voice, etc.)
- `sub_type`: Channel subtype for additional classification
- `position`: Channel position in the channel list
- `parent_id`: ID of parent channel (for channel categories)
- `owner_id`: ID of the channel owner
- `private_type`: Privacy setting for the channel
- `speak_permission`: Speaking permission requirements
- `application_id`: Associated application ID
- `permissions`: Channel-specific permissions

#### Example

```rust
async fn handle_channel_create(ctx: Context, channel: Channel) {
    println!("New channel created: {} (Type: {:?})", channel.name, channel.channel_type);
    
    match channel.channel_type {
        ChannelType::Text => {
            println!("Text channel created in guild {}", channel.guild_id);
        }
        ChannelType::Voice => {
            println!("Voice channel created in guild {}", channel.guild_id);
        }
        ChannelType::Category => {
            println!("Category created in guild {}", channel.guild_id);
        }
        _ => {
            println!("Other channel type created");
        }
    }
}
```

### `ChannelType`

Enumeration of different channel types.

```rust
pub enum ChannelType {
    Text = 0,
    Voice = 1,
    Category = 4,
    Announcement = 5,
    Forum = 10,
    Live = 11,
    Application = 12,
}
```

#### Variants

- `Text`: Text channel for messages
- `Voice`: Voice channel for audio communication
- `Category`: Category to organize channels
- `Announcement`: Announcement channel
- `Forum`: Forum channel for threaded discussions
- `Live`: Live streaming channel
- `Application`: Application-specific channel

#### Example

```rust
async fn create_text_channel(ctx: Context, guild_id: &str, name: &str) -> Result<Channel> {
    let channel = ctx.create_channel(
        guild_id,
        name,
        ChannelType::Text,
        None, // sub_type
        None, // position
        None, // parent_id
        None, // private_type
        None, // speak_permission
        None, // application_id
    ).await?;
    
    println!("Created text channel: {}", channel.name);
    Ok(channel)
}
```

### `ChannelSubType`

Enumeration of channel subtypes for additional classification.

```rust
pub enum ChannelSubType {
    Chat = 0,
    Announcement = 1,
    Guide = 2,
    Game = 3,
}
```

#### Variants

- `Chat`: General chat channel
- `Announcement`: Announcement-specific channel
- `Guide`: Guide or help channel
- `Game`: Gaming-related channel

## Member Types

### `Member`

Represents a member of a guild.

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

- `user`: User information for this member
- `nick`: Member's nickname in the guild
- `roles`: List of role IDs assigned to the member
- `joined_at`: Timestamp when the member joined the guild
- `deaf`: Whether the member is deafened in voice channels
- `mute`: Whether the member is muted in voice channels

#### Example

```rust
async fn handle_member_update(ctx: Context, member: Member) {
    if let Some(user) = &member.user {
        println!("Member updated: {}", user.username.as_deref().unwrap_or("Unknown"));
        
        if let Some(nick) = &member.nick {
            println!("Nickname: {}", nick);
        }
        
        println!("Roles: {:?}", member.roles);
    }
}
```

### `User`

Represents user information.

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
- `bot`: Whether this user is a bot
- `union_openid`: Union OpenID for cross-platform identification
- `union_user_account`: Union user account identifier

## Channel Management

### Creating Channels

```rust
async fn setup_guild_channels(ctx: Context, guild_id: &str) -> Result<()> {
    // Create a category
    let category = ctx.create_channel(
        guild_id,
        "General",
        ChannelType::Category,
        None,
        Some(0), // position at top
        None,
        None,
        None,
        None,
    ).await?;
    
    // Create text channels under the category
    let general_chat = ctx.create_channel(
        guild_id,
        "general",
        ChannelType::Text,
        Some(ChannelSubType::Chat),
        Some(1),
        Some(&category.id), // parent category
        None,
        None,
        None,
    ).await?;
    
    let announcements = ctx.create_channel(
        guild_id,
        "announcements",
        ChannelType::Text,
        Some(ChannelSubType::Announcement),
        Some(2),
        Some(&category.id),
        None,
        None,
        None,
    ).await?;
    
    println!("Created category '{}' with channels:", category.name);
    println!("  - {}", general_chat.name);
    println!("  - {}", announcements.name);
    
    Ok(())
}
```

### Channel Permissions

```rust
async fn manage_channel_permissions(ctx: Context, channel_id: &str, user_id: &str) -> Result<()> {
    // Get current permissions for a user
    let permissions = ctx.get_channel_user_permissions(channel_id, user_id).await?;
    println!("User permissions: {}", permissions.permissions);
    
    // Get permissions for a role
    let role_id = "role_id_here";
    let role_permissions = ctx.get_channel_role_permissions(channel_id, role_id).await?;
    println!("Role permissions: {}", role_permissions.permissions);
    
    Ok(())
}
```

### Guild Member Management

```rust
async fn manage_guild_members(ctx: Context, guild_id: &str) -> Result<()> {
    // Get guild members
    let members = ctx.get_guild_members(guild_id, Some(100), None).await?;
    println!("Guild has {} members", members.len());
    
    for member in &members {
        if let Some(user) = &member.user {
            println!("Member: {}", user.username.as_deref().unwrap_or("Unknown"));
            println!("  Roles: {:?}", member.roles);
            
            if let Some(joined) = &member.joined_at {
                println!("  Joined: {}", joined);
            }
        }
    }
    
    // Get specific member
    let user_id = "specific_user_id";
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    if let Some(user) = &member.user {
        println!("Found member: {}", user.username.as_deref().unwrap_or("Unknown"));
    }
    
    Ok(())
}
```

### Role Management

```rust
async fn manage_roles(ctx: Context, guild_id: &str) -> Result<()> {
    // Create a new role
    let new_role = ctx.create_guild_role(
        guild_id,
        "Moderator",
        Some(0x0099ff), // Blue color
        Some(true),     // Hoist (display separately)
        Some(100),      // Member limit
    ).await?;
    
    println!("Created role: {} (ID: {})", new_role.name, new_role.id);
    
    // Assign role to a user
    let user_id = "user_id_here";
    ctx.add_guild_role_member(guild_id, &new_role.id, user_id, None).await?;
    println!("Assigned role to user");
    
    // Update role
    let updated_role = ctx.update_guild_role(
        guild_id,
        &new_role.id,
        "Senior Moderator",
        Some(0xff9900), // Orange color
        Some(true),
        Some(50), // Reduced member limit
    ).await?;
    
    println!("Updated role: {}", updated_role.name);
    
    // Remove role from user
    ctx.remove_guild_role_member(guild_id, &new_role.id, user_id, None).await?;
    println!("Removed role from user");
    
    // Delete role
    ctx.delete_guild_role(guild_id, &new_role.id).await?;
    println!("Deleted role");
    
    Ok(())
}
```

## Common Usage Patterns

### Guild Discovery

```rust
async fn explore_guilds(ctx: Context) -> Result<()> {
    let guilds = ctx.get_guilds(None, None).await?;
    
    for guild in guilds {
        println!("Guild: {} (ID: {})", guild.name, guild.id);
        
        // Get channels for this guild
        let channels = ctx.get_channels(&guild.id).await?;
        println!("  Channels ({}):", channels.len());
        
        for channel in channels {
            let type_name = match channel.channel_type {
                ChannelType::Text => "Text",
                ChannelType::Voice => "Voice",
                ChannelType::Category => "Category",
                ChannelType::Announcement => "Announcement",
                ChannelType::Forum => "Forum",
                ChannelType::Live => "Live",
                ChannelType::Application => "Application",
            };
            
            println!("    {} - {} ({})", channel.name, type_name, channel.id);
        }
    }
    
    Ok(())
}
```

### Channel Organization

```rust
async fn organize_channels(ctx: Context, guild_id: &str) -> Result<()> {
    let channels = ctx.get_channels(guild_id).await?;
    
    // Group channels by category
    let mut categories = std::collections::HashMap::new();
    let mut orphaned_channels = Vec::new();
    
    for channel in channels {
        match (channel.channel_type, &channel.parent_id) {
            (ChannelType::Category, _) => {
                categories.insert(channel.id.clone(), (channel, Vec::new()));
            }
            (_, Some(parent_id)) => {
                if let Some((_, children)) = categories.get_mut(parent_id) {
                    children.push(channel);
                }
            }
            (_, None) => {
                orphaned_channels.push(channel);
            }
        }
    }
    
    // Print organized structure
    for (_, (category, children)) in categories {
        println!("Category: {}", category.name);
        for child in children {
            println!("  └─ {}", child.name);
        }
    }
    
    if !orphaned_channels.is_empty() {
        println!("Uncategorized channels:");
        for channel in orphaned_channels {
            println!("  - {}", channel.name);
        }
    }
    
    Ok(())
}
```

## See Also

- [Client API](../client.md) - Main client for guild and channel operations
- [Context API](../context.md) - Context object for API access
- [Messages](./messages.md) - Message types and handling
- [Users & Members](./users-members.md) - User and member management