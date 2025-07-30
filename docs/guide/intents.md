# Intents System

The Intents system in BotRS controls which events your bot receives from the QQ Guild Gateway. Intents act as a subscription mechanism that determines what data your bot has access to, providing both performance benefits and privacy controls.

## Understanding Intents

Intents are bitflags that specify which events your bot wants to receive. By only subscribing to the events you need, you can:

- **Reduce bandwidth usage** by filtering out unnecessary events
- **Improve performance** by processing fewer events
- **Comply with privacy requirements** by limiting access to sensitive data
- **Optimize memory usage** by avoiding storage of unused event data

## Intent Categories

### Public Intents

These intents can be used without special approval and cover most common bot use cases:

```rust
use botrs::Intents;

let intents = Intents::GUILD_MESSAGES 
    | Intents::GUILD_MESSAGE_REACTIONS
    | Intents::GUILDS;
```

**Available Public Intents:**
- `GUILDS` - Guild create, update, delete events
- `GUILD_MEMBERS` - Member join, update, remove events  
- `GUILD_MESSAGES` - Message create, update, delete in guilds
- `GUILD_MESSAGE_REACTIONS` - Message reaction add/remove
- `DIRECT_MESSAGE` - Private messages to the bot
- `GROUP_AND_C2C_EVENT` - Group and C2C message events
- `INTERACTION` - Slash commands and button interactions
- `MESSAGE_AUDIT` - Message audit events
- `FORUMS_EVENT` - Forum thread and post events
- `AUDIO_ACTION` - Voice channel events

### Privileged Intents

Some intents require special approval from QQ due to their access to sensitive user data:

**Privileged Intent Requirements:**
- `GUILD_MEMBERS` requires verification for bots in 100+ guilds
- Message content access may require additional permissions

## Basic Configuration

### Default Intent Setup

For most bots, you'll want to start with basic message and guild intents:

```rust
use botrs::{Client, Intents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let intents = Intents::GUILD_MESSAGES 
        | Intents::GUILDS
        | Intents::DIRECT_MESSAGE;
    
    let client = Client::builder("your_app_id")
        .intents(intents)
        .build()
        .await?;
    
    client.start().await?;
    Ok(())
}
```

### All Non-Privileged Intents

To subscribe to all available public events:

```rust
let intents = Intents::non_privileged();
```

### Custom Intent Combinations

Build specific intent combinations for your use case:

```rust
// Bot that only handles direct messages and reactions
let dm_bot_intents = Intents::DIRECT_MESSAGE 
    | Intents::GUILD_MESSAGE_REACTIONS;

// Moderation bot with full guild access
let mod_bot_intents = Intents::GUILDS 
    | Intents::GUILD_MEMBERS 
    | Intents::GUILD_MESSAGES
    | Intents::MESSAGE_AUDIT;

// Voice/audio bot
let voice_bot_intents = Intents::GUILDS 
    | Intents::AUDIO_ACTION;
```

## Event-Intent Mapping

Understanding which intents enable which events helps you configure exactly what your bot needs:

### Guild Events
```rust
// Requires: Intents::GUILDS
async fn guild_create(&self, ctx: Context, guild: Guild) {
    // Handle new guild
}

async fn guild_update(&self, ctx: Context, guild: Guild) {
    // Handle guild updates
}

async fn guild_delete(&self, ctx: Context, guild: UnavailableGuild) {
    // Handle guild removal
}
```

### Message Events
```rust
// Requires: Intents::GUILD_MESSAGES
async fn message_create(&self, ctx: Context, msg: Message) {
    // Handle guild messages
}

// Requires: Intents::DIRECT_MESSAGE
async fn direct_message_create(&self, ctx: Context, msg: Message) {
    // Handle private messages
}

// Requires: Intents::GROUP_AND_C2C_EVENT
async fn group_message_create(&self, ctx: Context, msg: Message) {
    // Handle group messages
}
```

### Member Events
```rust
// Requires: Intents::GUILD_MEMBERS
async fn guild_member_add(&self, ctx: Context, member: Member) {
    // Handle new member
}

async fn guild_member_update(&self, ctx: Context, member: Member) {
    // Handle member updates
}

async fn guild_member_remove(&self, ctx: Context, member: Member) {
    // Handle member removal
}
```

## Advanced Intent Management

### Runtime Intent Checking

You can check which intents are enabled at runtime:

```rust
use botrs::{Client, Intents};

impl EventHandler for MyBot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let client_intents = ctx.client.intents();
        
        if client_intents.contains(Intents::GUILD_MEMBERS) {
            println!("Member events enabled");
        }
        
        if client_intents.contains(Intents::GUILD_MESSAGES) {
            println!("Message events enabled");
        }
    }
}
```

### Conditional Event Handling

Design your event handlers to gracefully handle missing intents:

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Always available with GUILD_MESSAGES intent
        if let Some(content) = &msg.content {
            self.process_message_content(content).await;
        }
        
        // Only attempt member operations if we have the intent
        if ctx.client.intents().contains(Intents::GUILD_MEMBERS) {
            if let Some(member) = &msg.member {
                self.process_member_info(member).await;
            }
        }
    }
}
```

### Intent Validation

BotRS provides utilities to validate your intent configuration:

```rust
use botrs::Intents;

fn validate_bot_config(intents: Intents) -> Result<(), String> {
    // Ensure message handling bots have message intents
    if intents.contains(Intents::GUILD_MESSAGES) {
        if !intents.contains(Intents::GUILDS) {
            return Err("GUILD_MESSAGES requires GUILDS intent".to_string());
        }
    }
    
    // Warn about privileged intents
    if intents.contains(Intents::GUILD_MEMBERS) {
        println!("Warning: GUILD_MEMBERS is a privileged intent");
    }
    
    Ok(())
}
```

## Performance Considerations

### Intent Optimization

Choose the minimal set of intents for your bot's functionality:

```rust
// ❌ Avoid: Requesting all intents unnecessarily
let wasteful_intents = Intents::all();

// ✅ Better: Only request what you need
let efficient_intents = Intents::GUILD_MESSAGES | Intents::GUILDS;
```

### Event Filtering

Use intent-based event filtering to improve performance:

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Quick exit for bots without DM intent
        if msg.guild_id.is_none() && 
           !ctx.client.intents().contains(Intents::DIRECT_MESSAGE) {
            return;
        }
        
        // Process message
        self.handle_message(&ctx, &msg).await;
    }
}
```

## Troubleshooting

### Missing Events

If your bot isn't receiving expected events:

1. **Check Intent Configuration**: Verify you have the required intents enabled
2. **Verify Privileged Intents**: Ensure privileged intents are approved in Developer Portal
3. **Validate Event Handlers**: Confirm your event handler methods are properly implemented

### Common Intent Issues

```rust
// ❌ Problem: Missing guild context
let intents = Intents::GUILD_MESSAGES; // Missing GUILDS

// ✅ Solution: Include required dependent intents
let intents = Intents::GUILD_MESSAGES | Intents::GUILDS;
```

### Debug Intent Problems

Enable debug logging to troubleshoot intent issues:

```rust
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    
    let intents = Intents::GUILD_MESSAGES | Intents::GUILDS;
    tracing::info!("Starting bot with intents: {:?}", intents);
    
    // Bot setup...
    Ok(())
}
```

## Best Practices

### Intent Selection Guidelines

1. **Start Minimal**: Begin with only the intents you immediately need
2. **Add Incrementally**: Add new intents as you implement new features
3. **Document Requirements**: Comment why each intent is needed
4. **Test Thoroughly**: Verify all functionality works with your intent configuration

### Security Considerations

- **Principle of Least Privilege**: Only request intents your bot actually uses
- **Regular Audits**: Periodically review and remove unused intents
- **Privacy Compliance**: Be aware of what user data each intent provides access to

### Production Deployment

Before deploying to production:

1. **Validate Intent Approval**: Ensure all required privileged intents are approved
2. **Test Edge Cases**: Test behavior when intents are missing or revoked
3. **Monitor Performance**: Track the impact of intent configuration on bot performance
4. **Document Dependencies**: Clearly document which features require which intents

The Intent system provides fine-grained control over your bot's event subscription. By carefully selecting the appropriate intents, you can build efficient, privacy-conscious bots that scale well with your user base.