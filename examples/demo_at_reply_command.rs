//! Demo: AT Reply with Commands
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with a simple command system. It's equivalent to the Python demo_at_reply_command.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// A simple command handler function type.
type CommandHandlerFn = fn(&str) -> Option<String>;

/// Simple command registry structure.
struct CommandRegistry {
    commands: Vec<(Vec<String>, CommandHandlerFn)>,
}

impl CommandRegistry {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Register a command with multiple aliases.
    fn register(&mut self, aliases: Vec<&str>, handler: CommandHandlerFn) {
        let aliases: Vec<String> = aliases.iter().map(|s| s.to_string()).collect();
        self.commands.push((aliases, handler));
    }

    /// Try to execute a command based on the message content.
    fn try_execute(&self, content: &str) -> Option<String> {
        let trimmed = content.trim();

        for (aliases, handler) in &self.commands {
            for alias in aliases {
                if trimmed.starts_with(alias) {
                    // Extract parameters after the command
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

/// Command handler for hello/你好
fn hello_command(params: &str) -> Option<String> {
    info!("Hello command executed with params: {}", params);
    Some(if params.is_empty() {
        "Hello! 你好！".to_string()
    } else {
        format!("Hello! 你好！参数: {}", params)
    })
}

/// Command handler for good night/晚安
fn good_night_command(params: &str) -> Option<String> {
    info!("Good night command executed with params: {}", params);
    Some(if params.is_empty() {
        "Good night! 晚安！".to_string()
    } else {
        format!("Good night! 晚安！参数: {}", params)
    })
}

/// Event handler that responds to @ mentions with command processing.
struct AtReplyCommandHandler {
    registry: CommandRegistry,
}

impl AtReplyCommandHandler {
    fn new() -> Self {
        let mut registry = CommandRegistry::new();

        // Register commands with their aliases (similar to @Commands decorator)
        registry.register(vec!["你好", "hello"], hello_command);
        registry.register(vec!["晚安"], good_night_command);

        Self { registry }
    }
}

#[async_trait::async_trait]
impl EventHandler for AtReplyCommandHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, ctx: Context, message: Message) {
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received message: {}", content);

        // Try to execute commands
        if let Some(response) = self.registry.try_execute(content) {
            // Send reply using message.reply (equivalent to first method in Python)
            match message.reply(&ctx.api, &ctx.token, &response).await {
                Ok(_) => info!("Successfully sent reply via message.reply"),
                Err(e) => warn!("Failed to send reply via message.reply: {}", e),
            }

            // Also send using api.post_message (equivalent to second method in Python)
            let params = botrs::models::message::MessageParams {
                content: Some(response),
                msg_id: message.id.clone(),
                ..Default::default()
            };

            match ctx
                .api
                .post_message_with_params(
                    &ctx.token,
                    &message.channel_id.as_ref().unwrap_or(&String::new()),
                    params,
                )
                .await
            {
                Ok(_) => info!("Successfully sent message via api.post_message"),
                Err(e) => warn!("Failed to send message via api.post_message: {}", e),
            }
        } else {
            // No command matched, send a default response
            let default_response = "收到消息，但没有匹配的命令。可用命令: 你好/hello, 晚安";

            match message.reply(&ctx.api, &ctx.token, default_response).await {
                Ok(_) => info!("Successfully sent default reply"),
                Err(e) => warn!("Failed to send default reply: {}", e),
            }
        }
    }

    /// Called when an error occurs during event processing.
    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init_logging();

    info!("Starting AT reply command demo...");

    // Load configuration with multiple fallback options
    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1), // app_id from command line
        env::args().nth(2), // secret from command line
    )?;

    info!("Configuration loaded successfully");

    // Create token
    let token = Token::new(config.bot.app_id, config.bot.secret);

    // Validate token
    if let Err(e) = token.validate() {
        panic!("Invalid token: {}", e);
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive public guild messages (@ mentions)
    // This is equivalent to: intents = botpy.Intents(public_guild_messages=True)
    let intents = Intents::default().with_public_guild_messages();

    info!("Configured intents: {}", intents);

    // Create event handler with command registry
    let handler = AtReplyCommandHandler::new();

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}
