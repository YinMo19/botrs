# Command Handler Example

This example demonstrates how to build a robust command handling system for your QQ Guild bot using BotRS.

## Overview

A command handler provides a structured way to process user commands, validate permissions, and organize bot functionality. This example shows how to create a flexible command system that can be easily extended.

## Basic Command Structure

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token, BotError};
use std::collections::HashMap;
use async_trait::async_trait;

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub min_args: usize,
    pub max_args: Option<usize>,
    pub requires_permission: Option<String>,
}

pub struct CommandHandler {
    pub commands: HashMap<String, Command>,
    pub prefix: String,
}

impl CommandHandler {
    pub fn new(prefix: &str) -> Self {
        let mut handler = Self {
            commands: HashMap::new(),
            prefix: prefix.to_string(),
        };
        
        // Register built-in commands
        handler.register_default_commands();
        handler
    }
    
    fn register_default_commands(&mut self) {
        self.register_command(Command {
            name: "ping".to_string(),
            description: "Test bot responsiveness".to_string(),
            usage: "!ping".to_string(),
            min_args: 0,
            max_args: Some(0),
            requires_permission: None,
        });
        
        self.register_command(Command {
            name: "help".to_string(),
            description: "Show available commands".to_string(),
            usage: "!help [command]".to_string(),
            min_args: 0,
            max_args: Some(1),
            requires_permission: None,
        });
        
        self.register_command(Command {
            name: "echo".to_string(),
            description: "Echo back the provided text".to_string(),
            usage: "!echo <text>".to_string(),
            min_args: 1,
            max_args: None,
            requires_permission: None,
        });
        
        self.register_command(Command {
            name: "kick".to_string(),
            description: "Kick a member from the guild".to_string(),
            usage: "!kick <@user> [reason]".to_string(),
            min_args: 1,
            max_args: None,
            requires_permission: Some("kick_members".to_string()),
        });
        
        self.register_command(Command {
            name: "mute".to_string(),
            description: "Mute a member in voice channels".to_string(),
            usage: "!mute <@user> [duration_seconds]".to_string(),
            min_args: 1,
            max_args: Some(2),
            requires_permission: Some("manage_channels".to_string()),
        });
    }
    
    pub fn register_command(&mut self, command: Command) {
        self.commands.insert(command.name.clone(), command);
    }
    
    pub fn parse_command(&self, content: &str) -> Option<ParsedCommand> {
        if !content.starts_with(&self.prefix) {
            return None;
        }
        
        let without_prefix = &content[self.prefix.len()..];
        let parts: Vec<&str> = without_prefix.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }
        
        Some(ParsedCommand {
            name: parts[0].to_lowercase(),
            args: parts[1..].iter().map(|s| s.to_string()).collect(),
            raw_args: if parts.len() > 1 {
                without_prefix[parts[0].len()..].trim().to_string()
            } else {
                String::new()
            },
        })
    }
}

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
    pub raw_args: String,
}
```

## Bot Implementation

```rust
pub struct CommandBot {
    command_handler: CommandHandler,
}

impl CommandBot {
    pub fn new() -> Self {
        Self {
            command_handler: CommandHandler::new("!"),
        }
    }
}

#[async_trait]
impl EventHandler for CommandBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Command bot is ready! Logged in as: {}", ready.user.username);
        println!("Commands available with prefix: {}", self.command_handler.prefix);
    }
    
    async fn message_create(&self, ctx: Context, message: Message) {
        // Ignore bot messages
        if message.is_from_bot() {
            return;
        }
        
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };
        
        // Parse command
        let parsed_command = match self.command_handler.parse_command(content) {
            Some(cmd) => cmd,
            None => return, // Not a command
        };
        
        // Get command definition
        let command = match self.command_handler.commands.get(&parsed_command.name) {
            Some(cmd) => cmd,
            None => {
                let _ = message.reply(
                    &ctx.api,
                    &ctx.token,
                    &format!("Unknown command: `{}`. Use `{}help` for available commands.", 
                            parsed_command.name, self.command_handler.prefix)
                ).await;
                return;
            }
        };
        
        // Validate arguments
        if let Err(error_msg) = self.validate_command_args(command, &parsed_command) {
            let _ = message.reply(&ctx.api, &ctx.token, &error_msg).await;
            return;
        }
        
        // Check permissions if required
        if let Some(required_perm) = &command.requires_permission {
            if let Err(error_msg) = self.check_permission(&ctx, &message, required_perm).await {
                let _ = message.reply(&ctx.api, &ctx.token, &error_msg).await;
                return;
            }
        }
        
        // Execute command
        if let Err(e) = self.execute_command(&ctx, &message, command, &parsed_command).await {
            eprintln!("Command execution failed: {}", e);
            let _ = message.reply(
                &ctx.api,
                &ctx.token,
                "An error occurred while executing the command."
            ).await;
        }
    }
    
    async fn error(&self, error: BotError) {
        eprintln!("Bot error: {}", error);
    }
}

impl CommandBot {
    fn validate_command_args(&self, command: &Command, parsed: &ParsedCommand) -> Result<(), String> {
        let arg_count = parsed.args.len();
        
        if arg_count < command.min_args {
            return Err(format!(
                "Not enough arguments. Usage: `{}`",
                command.usage
            ));
        }
        
        if let Some(max_args) = command.max_args {
            if arg_count > max_args {
                return Err(format!(
                    "Too many arguments. Usage: `{}`",
                    command.usage
                ));
            }
        }
        
        Ok(())
    }
    
    async fn check_permission(
        &self,
        ctx: &Context,
        message: &Message,
        required_permission: &str,
    ) -> Result<(), String> {
        // Get the user ID from the message author
        let user_id = match &message.author {
            Some(author) => &author.id,
            None => return Err("Cannot determine user permissions".to_string()),
        };
        
        // Check user permissions (simplified)
        match ctx.get_channel_user_permissions(&message.channel_id, user_id).await {
            Ok(permissions) => {
                if self.has_permission(&permissions.permissions, required_permission) {
                    Ok(())
                } else {
                    Err(format!("You don't have the required permission: {}", required_permission))
                }
            }
            Err(_) => Err("Failed to check permissions".to_string()),
        }
    }
    
    fn has_permission(&self, permissions: &str, required: &str) -> bool {
        // Simplified permission checking
        let perm_value: u64 = permissions.parse().unwrap_or(0);
        let required_bit = match required {
            "kick_members" => 1 << 1,
            "manage_channels" => 1 << 4,
            "manage_messages" => 1 << 13,
            _ => return false,
        };
        (perm_value & required_bit) != 0
    }
    
    async fn execute_command(
        &self,
        ctx: &Context,
        message: &Message,
        command: &Command,
        parsed: &ParsedCommand,
    ) -> Result<(), BotError> {
        match command.name.as_str() {
            "ping" => self.handle_ping(ctx, message).await,
            "help" => self.handle_help(ctx, message, parsed).await,
            "echo" => self.handle_echo(ctx, message, parsed).await,
            "kick" => self.handle_kick(ctx, message, parsed).await,
            "mute" => self.handle_mute(ctx, message, parsed).await,
            _ => {
                message.reply(ctx.api, ctx.token, "Command not implemented yet.").await?;
                Ok(())
            }
        }
    }
    
    async fn handle_ping(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let start_time = std::time::Instant::now();
        let response = message.reply(&ctx.api, &ctx.token, "Pong! 🏓").await?;
        let latency = start_time.elapsed();
        
        // Edit the response to include latency (if API supports editing)
        let latency_msg = format!("Pong! 🏓 Latency: {:?}", latency);
        message.reply(&ctx.api, &ctx.token, &latency_msg).await?;
        
        Ok(())
    }
    
    async fn handle_help(&self, ctx: &Context, message: &Message, parsed: &ParsedCommand) -> Result<(), BotError> {
        if parsed.args.is_empty() {
            // Show all commands
            let mut help_text = format!("**Available Commands** (prefix: `{}`)\n\n", self.command_handler.prefix);
            
            for (_, command) in &self.command_handler.commands {
                help_text.push_str(&format!(
                    "**{}** - {}\n",
                    command.name,
                    command.description
                ));
            }
            
            help_text.push_str(&format!("\nUse `{}help <command>` for detailed usage information.", self.command_handler.prefix));
            message.reply(&ctx.api, &ctx.token, &help_text).await?;
        } else {
            // Show specific command help
            let command_name = &parsed.args[0];
            
            match self.command_handler.commands.get(command_name) {
                Some(command) => {
                    let help_text = format!(
                        "**{}**\n\n**Description:** {}\n**Usage:** `{}`{}",
                        command.name,
                        command.description,
                        command.usage,
                        if let Some(perm) = &command.requires_permission {
                            format!("\n**Required Permission:** {}", perm)
                        } else {
                            String::new()
                        }
                    );
                    message.reply(&ctx.api, &ctx.token, &help_text).await?;
                }
                None => {
                    message.reply(
                        &ctx.api,
                        &ctx.token,
                        &format!("Command `{}` not found.", command_name)
                    ).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_echo(&self, ctx: &Context, message: &Message, parsed: &ParsedCommand) -> Result<(), BotError> {
        let echo_text = if parsed.raw_args.is_empty() {
            "Echo! (no text provided)".to_string()
        } else {
            parsed.raw_args.clone()
        };
        
        message.reply(&ctx.api, &ctx.token, &echo_text).await?;
        Ok(())
    }
    
    async fn handle_kick(&self, ctx: &Context, message: &Message, parsed: &ParsedCommand) -> Result<(), BotError> {
        // Extract user ID from mention (simplified)
        let user_mention = &parsed.args[0];
        let user_id = self.extract_user_id_from_mention(user_mention)?;
        
        let reason = if parsed.args.len() > 1 {
            parsed.args[1..].join(" ")
        } else {
            "No reason provided".to_string()
        };
        
        // Kick the member
        match ctx.kick_member(&message.guild_id, &user_id, Some(1), Some(&reason)).await {
            Ok(_) => {
                message.reply(
                    &ctx.api,
                    &ctx.token,
                    &format!("User {} has been kicked. Reason: {}", user_mention, reason)
                ).await?;
            }
            Err(e) => {
                message.reply(
                    &ctx.api,
                    &ctx.token,
                    &format!("Failed to kick user: {}", e)
                ).await?;
            }
        }
        
        Ok(())
    }
    
    async fn handle_mute(&self, ctx: &Context, message: &Message, parsed: &ParsedCommand) -> Result<(), BotError> {
        let user_mention = &parsed.args[0];
        let user_id = self.extract_user_id_from_mention(user_mention)?;
        
        let duration = if parsed.args.len() > 1 {
            parsed.args[1].parse::<u64>().unwrap_or(300) // Default 5 minutes
        } else {
            300
        };
        
        // Find a voice channel (simplified - in practice you'd track voice states)
        // For this example, we'll assume the message channel is voice-capable
        match ctx.mute_member(&message.channel_id, &user_id, Some(duration), Some("Muted by command")).await {
            Ok(_) => {
                message.reply(
                    &ctx.api,
                    &ctx.token,
                    &format!("User {} has been muted for {} seconds.", user_mention, duration)
                ).await?;
            }
            Err(e) => {
                message.reply(
                    &ctx.api,
                    &ctx.token,
                    &format!("Failed to mute user: {}", e)
                ).await?;
            }
        }
        
        Ok(())
    }
    
    fn extract_user_id_from_mention(&self, mention: &str) -> Result<String, BotError> {
        // Parse user mentions like <@123456789> or <@!123456789>
        if mention.starts_with("<@") && mention.ends_with('>') {
            let id_part = mention.trim_start_matches("<@").trim_start_matches('!').trim_end_matches('>');
            if id_part.chars().all(|c| c.is_ascii_digit()) {
                Ok(id_part.to_string())
            } else {
                Err(BotError::InvalidInput("Invalid user mention format".to_string()))
            }
        } else {
            Err(BotError::InvalidInput("Please mention a user with @".to_string()))
        }
    }
}
```

## Main Function

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,command_handler=info")
        .init();

    println!("Starting command handler bot...");

    // Get credentials
    let app_id = std::env::var("QQ_BOT_APP_ID")
        .expect("QQ_BOT_APP_ID environment variable required");
    let secret = std::env::var("QQ_BOT_SECRET")
        .expect("QQ_BOT_SECRET environment variable required");

    // Create token and validate
    let token = Token::new(app_id, secret);
    token.validate()?;

    // Set up intents
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_guilds()
        .with_guild_members();

    // Create bot with command handler
    let handler = CommandBot::new();
    let mut client = Client::new(token, intents, handler, false)?;

    println!("Command bot starting...");
    client.start().await?;

    Ok(())
}
```

## Advanced Features

### Custom Command Registration

```rust
impl CommandBot {
    pub fn register_custom_command(&mut self, command: Command) {
        self.command_handler.register_command(command);
    }
    
    pub fn create_custom_commands(&mut self) {
        // Server info command
        self.register_custom_command(Command {
            name: "serverinfo".to_string(),
            description: "Display server information".to_string(),
            usage: "!serverinfo".to_string(),
            min_args: 0,
            max_args: Some(0),
            requires_permission: None,
        });
        
        // User info command
        self.register_custom_command(Command {
            name: "userinfo".to_string(),
            description: "Display user information".to_string(),
            usage: "!userinfo [@user]".to_string(),
            min_args: 0,
            max_args: Some(1),
            requires_permission: None,
        });
        
        // Clear messages command
        self.register_custom_command(Command {
            name: "clear".to_string(),
            description: "Clear recent messages".to_string(),
            usage: "!clear <count>".to_string(),
            min_args: 1,
            max_args: Some(1),
            requires_permission: Some("manage_messages".to_string()),
        });
    }
}
```

### Cooldown System

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CooldownManager {
    cooldowns: HashMap<String, Instant>,
}

impl CooldownManager {
    pub fn new() -> Self {
        Self {
            cooldowns: HashMap::new(),
        }
    }
    
    pub fn check_cooldown(&mut self, key: &str, duration: Duration) -> bool {
        let now = Instant::now();
        
        if let Some(&last_used) = self.cooldowns.get(key) {
            if now.duration_since(last_used) < duration {
                return false; // Still on cooldown
            }
        }
        
        self.cooldowns.insert(key.to_string(), now);
        true
    }
    
    pub fn get_remaining_cooldown(&self, key: &str, duration: Duration) -> Option<Duration> {
        if let Some(&last_used) = self.cooldowns.get(key) {
            let elapsed = Instant::now().duration_since(last_used);
            if elapsed < duration {
                return Some(duration - elapsed);
            }
        }
        None
    }
}
```

### Command Middleware

```rust
pub trait CommandMiddleware {
    async fn before_command(
        &self,
        ctx: &Context,
        message: &Message,
        command: &Command,
    ) -> Result<bool, BotError>; // Return false to stop execution
    
    async fn after_command(
        &self,
        ctx: &Context,
        message: &Message,
        command: &Command,
        result: &Result<(), BotError>,
    );
}

pub struct LoggingMiddleware;

#[async_trait]
impl CommandMiddleware for LoggingMiddleware {
    async fn before_command(
        &self,
        _ctx: &Context,
        message: &Message,
        command: &Command,
    ) -> Result<bool, BotError> {
        println!(
            "Executing command '{}' from user {} in channel {}",
            command.name,
            message.author.as_ref().map(|a| &a.id).unwrap_or(&"unknown".to_string()),
            message.channel_id
        );
        Ok(true)
    }
    
    async fn after_command(
        &self,
        _ctx: &Context,
        _message: &Message,
        command: &Command,
        result: &Result<(), BotError>,
    ) {
        match result {
            Ok(_) => println!("Command '{}' executed successfully", command.name),
            Err(e) => println!("Command '{}' failed: {}", command.name, e),
        }
    }
}
```

## Usage Examples

### Basic Commands

```
# Test bot responsiveness
!ping

# Get help
!help
!help ping

# Echo text
!echo Hello, world!

# Server moderation (requires permissions)
!kick @user Spamming
!mute @user 300
```

### Error Handling

The command handler includes comprehensive error handling:

- **Invalid syntax**: Shows usage information
- **Missing permissions**: Informs user about required permissions
- **Invalid arguments**: Validates argument count and format
- **API failures**: Gracefully handles and reports errors

## Best Practices

1. **Command Validation**: Always validate arguments before execution
2. **Permission Checks**: Verify user permissions for moderation commands
3. **Error Handling**: Provide user-friendly error messages
4. **Logging**: Log command execution for debugging and audit purposes
5. **Rate Limiting**: Implement cooldowns to prevent spam
6. **Extensibility**: Design the system to easily add new commands

## See Also

- [Getting Started](./getting-started.md) - Basic bot setup
- [Rich Messages](./rich-messages.md) - Advanced message formatting
- [Event Handling](./event-handling.md) - Comprehensive event processing
- [Error Recovery](./error-recovery.md) - Advanced error handling patterns