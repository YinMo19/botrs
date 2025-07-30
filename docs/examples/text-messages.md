# Text Messages Examples

This guide demonstrates how to handle and send text messages using BotRS. Text messages are the most common type of interaction in QQ Guild bots, supporting various scenarios from simple replies to complex command systems.

## Basic Text Handling

### Simple Echo Bot

The most basic text message handling involves echoing user input back to them.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct EchoHandler;

#[async_trait::async_trait]
impl EventHandler for EchoHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        // Ignore bot messages to prevent loops
        if message.is_from_bot() {
            return;
        }

        // Check if message has content
        if let Some(content) = &message.content {
            // Simple echo - reply with the same content
            if let Err(e) = message.reply(&ctx.api, &ctx.token, content).await {
                eprintln!("Failed to send reply: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_guild_messages();
    let handler = EchoHandler;
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### @ Mention Replies

Responding specifically to messages that mention the bot.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MentionHandler;

#[async_trait::async_trait]
impl EventHandler for MentionHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot {} is ready for mentions!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // Get bot name for personalized responses
            let bot_name = ctx
                .bot_info
                .as_ref()
                .map(|info| info.username.as_str())
                .unwrap_or("Bot");

            let reply_content = format!(
                "Hello! Bot {} received your mention: {}",
                bot_name, content
            );

            if let Err(e) = message.reply(&ctx.api, &ctx.token, &reply_content).await {
                eprintln!("Failed to reply to mention: {}", e);
            }
        }
    }
}
```

## Command Systems

### Simple Command Handler

Building a basic command system that responds to prefixed messages.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use std::collections::HashMap;

type CommandHandler = fn(&str) -> String;

struct CommandBot {
    commands: HashMap<String, CommandHandler>,
}

impl CommandBot {
    fn new() -> Self {
        let mut commands = HashMap::new();
        
        // Register commands
        commands.insert("ping".to_string(), ping_command as CommandHandler);
        commands.insert("hello".to_string(), hello_command as CommandHandler);
        commands.insert("echo".to_string(), echo_command as CommandHandler);
        commands.insert("help".to_string(), help_command as CommandHandler);
        
        Self { commands }
    }

    fn handle_command(&self, content: &str) -> Option<String> {
        // Commands start with !
        if !content.starts_with('!') {
            return None;
        }

        let content = &content[1..]; // Remove !
        let parts: Vec<&str> = content.splitn(2, ' ').collect();
        let command = parts[0];
        let args = parts.get(1).unwrap_or(&"");

        self.commands.get(command).map(|handler| handler(args))
    }
}

// Command implementations
fn ping_command(_args: &str) -> String {
    "Pong!".to_string()
}

fn hello_command(args: &str) -> String {
    if args.is_empty() {
        "Hello there!".to_string()
    } else {
        format!("Hello, {}!", args)
    }
}

fn echo_command(args: &str) -> String {
    if args.is_empty() {
        "Nothing to echo!".to_string()
    } else {
        format!("Echo: {}", args)
    }
}

fn help_command(_args: &str) -> String {
    "Available commands:\n!ping - Responds with Pong!\n!hello [name] - Greets you\n!echo <text> - Echoes your text\n!help - Shows this help".to_string()
}

#[async_trait::async_trait]
impl EventHandler for CommandBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Command bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if let Some(response) = self.handle_command(content) {
                if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                    eprintln!("Failed to send command response: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_guild_messages();
    let handler = CommandBot::new();
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### Advanced Command System with Aliases

More sophisticated command handling with command aliases and argument parsing.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct CommandRegistry {
    commands: Vec<(Vec<String>, fn(&str) -> Option<String>)>,
}

impl CommandRegistry {
    fn new() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
        };

        // Register commands with aliases
        registry.register(vec!["hello", "hi", "hey"], hello_handler);
        registry.register(vec!["goodbye", "bye", "cya"], goodbye_handler);
        registry.register(vec!["time", "clock"], time_handler);
        registry.register(vec!["random", "rand"], random_handler);

        registry
    }

    fn register(&mut self, aliases: Vec<&str>, handler: fn(&str) -> Option<String>) {
        let aliases: Vec<String> = aliases.iter().map(|s| s.to_string()).collect();
        self.commands.push((aliases, handler));
    }

    fn execute(&self, content: &str) -> Option<String> {
        let trimmed = content.trim();
        
        for (aliases, handler) in &self.commands {
            for alias in aliases {
                if trimmed.starts_with(alias) {
                    let params = if trimmed.len() > alias.len() {
                        trimmed[alias.len()..].trim()
                    } else {
                        ""
                    };
                    return handler(params);
                }
            }
        }
        
        None
    }
}

// Command handlers
fn hello_handler(params: &str) -> Option<String> {
    Some(if params.is_empty() {
        "Hello! How are you today?".to_string()
    } else {
        format!("Hello, {}! Nice to meet you!", params)
    })
}

fn goodbye_handler(params: &str) -> Option<String> {
    Some(if params.is_empty() {
        "Goodbye! Have a great day!".to_string()
    } else {
        format!("Goodbye, {}! See you later!", params)
    })
}

fn time_handler(_params: &str) -> Option<String> {
    use chrono::Utc;
    let now = Utc::now();
    Some(format!("Current UTC time: {}", now.format("%Y-%m-%d %H:%M:%S")))
}

fn random_handler(params: &str) -> Option<String> {
    use rand::Rng;
    
    if params.is_empty() {
        let num = rand::thread_rng().gen_range(1..=100);
        Some(format!("Random number: {}", num))
    } else if let Ok(max) = params.parse::<u32>() {
        let num = rand::thread_rng().gen_range(1..=max);
        Some(format!("Random number (1-{}): {}", max, num))
    } else {
        Some("Invalid number format. Usage: random [max_number]".to_string())
    }
}

struct AdvancedCommandHandler {
    registry: CommandRegistry,
}

impl AdvancedCommandHandler {
    fn new() -> Self {
        Self {
            registry: CommandRegistry::new(),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for AdvancedCommandHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Advanced command bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if let Some(response) = self.registry.execute(content) {
                if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                    eprintln!("Failed to send response: {}", e);
                }
            }
        }
    }
}
```

## Different Message Types

### Group Messages

Handling text messages in QQ groups.

```rust
use botrs::{Client, Context, EventHandler, GroupMessage, Intents, Ready, Token};

struct GroupTextHandler;

#[async_trait::async_trait]
impl EventHandler for GroupTextHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Group bot {} is ready!", ready.user.username);
    }

    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        if let Some(content) = &message.content {
            println!("Received group message: {}", content);

            // Handle specific group commands
            let response = if content.contains("hello") {
                Some("Hello everyone in the group!")
            } else if content.contains("help") {
                Some("Group commands: hello, help, info")
            } else if content.contains("info") {
                Some("This is a QQ group bot built with BotRS")
            } else {
                None
            };

            if let Some(reply_text) = response {
                // Use the convenience reply method
                if let Err(e) = message.reply(&ctx.api, &ctx.token, reply_text).await {
                    eprintln!("Failed to reply to group message: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_messages();
    let handler = GroupTextHandler;
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### C2C Messages

Handling client-to-client text messages.

```rust
use botrs::{C2CMessage, Client, Context, EventHandler, Intents, Ready, Token};

struct C2CTextHandler;

#[async_trait::async_trait]
impl EventHandler for C2CTextHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("C2C bot {} is ready!", ready.user.username);
    }

    async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
        if let Some(content) = &message.content {
            println!("Received C2C message: {}", content);

            // Create a personalized response
            let reply_content = format!("I received your private message: {}", content);

            // Reply to the C2C message
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &reply_content).await {
                eprintln!("Failed to reply to C2C message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_messages();
    let handler = C2CTextHandler;
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### Direct Messages

Handling direct messages in guild contexts.

```rust
use botrs::{Client, Context, DirectMessage, EventHandler, Intents, Ready, Token};

struct DirectMessageHandler;

#[async_trait::async_trait]
impl EventHandler for DirectMessageHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("DM bot {} is ready!", ready.user.username);
    }

    async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
        if let Some(content) = &message.content {
            println!("Received direct message: {}", content);

            // Handle DM-specific commands
            let response = match content.to_lowercase().as_str() {
                "help" => "DM Commands: help, status, info",
                "status" => "Bot is running normally",
                "info" => "This is a private conversation with the bot",
                _ => "Thanks for your message! Type 'help' for available commands.",
            };

            if let Err(e) = message.reply(&ctx.api, &ctx.token, response).await {
                eprintln!("Failed to reply to direct message: {}", e);
            }
        }
    }
}
```

## Advanced Text Processing

### Text Analysis and Response

Analyzing message content for intelligent responses.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct AnalysisHandler;

impl AnalysisHandler {
    fn analyze_sentiment(&self, text: &str) -> &'static str {
        let positive_words = ["good", "great", "awesome", "love", "happy", "excellent"];
        let negative_words = ["bad", "terrible", "hate", "sad", "awful", "horrible"];
        
        let text_lower = text.to_lowercase();
        let positive_count = positive_words.iter()
            .map(|word| text_lower.matches(word).count())
            .sum::<usize>();
        let negative_count = negative_words.iter()
            .map(|word| text_lower.matches(word).count())
            .sum::<usize>();
        
        if positive_count > negative_count {
            "positive"
        } else if negative_count > positive_count {
            "negative"
        } else {
            "neutral"
        }
    }

    fn generate_response(&self, content: &str) -> String {
        let sentiment = self.analyze_sentiment(content);
        let word_count = content.split_whitespace().count();
        
        match sentiment {
            "positive" => format!("I'm glad to hear positive things! Your message ({} words) sounds great!", word_count),
            "negative" => format!("I hope things get better! Thanks for sharing your {} words with me.", word_count),
            _ => format!("Thanks for your message! I received {} words from you.", word_count),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for AnalysisHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Analysis bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // Skip short messages
            if content.len() < 10 {
                return;
            }

            let response = self.generate_response(content);
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                eprintln!("Failed to send analysis response: {}", e);
            }
        }
    }
}
```

### Rate Limiting and Spam Protection

Implementing basic rate limiting for text responses.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct RateLimitedHandler {
    last_response: HashMap<String, Instant>,
    cooldown: Duration,
}

impl RateLimitedHandler {
    fn new(cooldown_seconds: u64) -> Self {
        Self {
            last_response: HashMap::new(),
            cooldown: Duration::from_secs(cooldown_seconds),
        }
    }

    fn can_respond(&mut self, user_id: &str) -> bool {
        let now = Instant::now();
        
        if let Some(&last_time) = self.last_response.get(user_id) {
            if now.duration_since(last_time) < self.cooldown {
                return false;
            }
        }
        
        self.last_response.insert(user_id.to_string(), now);
        true
    }
}

#[async_trait::async_trait]
impl EventHandler for RateLimitedHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Rate-limited bot {} is ready!", ready.user.username);
    }

    async fn message_create(&mut self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let (Some(content), Some(author)) = (&message.content, &message.author) {
            // Check rate limit
            if !self.can_respond(&author.id) {
                return; // Silently ignore if user is rate limited
            }

            // Simple echo with rate limiting
            let response = format!("Echo (rate limited): {}", content);
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                eprintln!("Failed to send rate-limited response: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_guild_messages();
    let handler = RateLimitedHandler::new(5); // 5 second cooldown
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

## Error Handling

### Robust Error Handling

Implementing comprehensive error handling for text message operations.

```rust
use botrs::{BotError, Client, Context, EventHandler, Intents, Message, Ready, Token};

struct RobustHandler;

impl RobustHandler {
    async fn safe_reply(&self, ctx: &Context, message: &Message, content: &str) -> Result<(), BotError> {
        match message.reply(&ctx.api, &ctx.token, content).await {
            Ok(_) => {
                println!("Successfully sent reply");
                Ok(())
            }
            Err(BotError::Http(status)) => {
                eprintln!("HTTP error {}: Failed to send message", status);
                Err(BotError::Http(status))
            }
            Err(BotError::RateLimit(retry_after)) => {
                eprintln!("Rate limited, retry after {} seconds", retry_after);
                // Could implement retry logic here
                Err(BotError::RateLimit(retry_after))
            }
            Err(e) => {
                eprintln!("Other error: {}", e);
                Err(e)
            }
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for RobustHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Robust bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // Validate message content
            if content.len() > 2000 {
                let _ = self.safe_reply(&ctx, &message, "Message too long! Please keep it under 2000 characters.").await;
                return;
            }

            if content.trim().is_empty() {
                return; // Ignore empty messages
            }

            // Process the message
            let response = format!("Processed: {}", content);
            let _ = self.safe_reply(&ctx, &message, &response).await;
        }
    }

    async fn error(&self, error: BotError) {
        eprintln!("Handler error: {}", error);
    }
}
```

## Best Practices

### Performance Considerations

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tokio::sync::Semaphore;
use std::sync::Arc;

struct PerformantHandler {
    // Limit concurrent message processing
    semaphore: Arc<Semaphore>,
}

impl PerformantHandler {
    fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    async fn process_message(&self, ctx: Context, message: Message) {
        // Acquire permit for processing
        let _permit = self.semaphore.acquire().await.unwrap();

        if let Some(content) = &message.content {
            // Simulate processing time
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            let response = format!("Processed: {}", content);
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                eprintln!("Failed to reply: {}", e);
            }
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for PerformantHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Performant bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // Spawn task for concurrent processing
        let handler = Arc::new(self.clone());
        tokio::spawn(async move {
            handler.process_message(ctx, message).await;
        });
    }
}
```

This comprehensive guide covers the essential patterns for handling text messages in BotRS, from basic echo bots to sophisticated command systems with error handling and performance optimizations.

## See Also

- [Rich Messages](./rich-messages.md) - Working with embeds, attachments, and interactive content
- [Message Models](../api/models/messages.md) - Detailed API reference for message types
- [Event Handling](./event-handling.md) - Complete guide to event handling patterns
- [Error Recovery](./error-recovery.md) - Advanced error handling strategies