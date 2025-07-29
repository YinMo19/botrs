//! Demo: Open Forum Event
//!
//! This example demonstrates how to create a bot that handles open forum events.
//! It's equivalent to the Python demo_open_forum_event.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, OpenThread, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to open forum events.
struct OpenForumEventHandler;

#[async_trait::async_trait]
impl EventHandler for OpenForumEventHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when an open forum thread is created.
    async fn open_forum_thread_create(&self, _ctx: Context, open_forum_thread: OpenThread) {
        let author_id = open_forum_thread.author_id.as_deref().unwrap_or("Unknown");
        info!("{} 创建了主题", author_id);
    }

    /// Called when an open forum thread is updated.
    async fn open_forum_thread_update(&self, _ctx: Context, open_forum_thread: OpenThread) {
        let author_id = open_forum_thread.author_id.as_deref().unwrap_or("Unknown");
        info!("{} 更新了主题", author_id);
    }

    /// Called when an open forum thread is deleted.
    async fn open_forum_thread_delete(&self, _ctx: Context, open_forum_thread: OpenThread) {
        let author_id = open_forum_thread.author_id.as_deref().unwrap_or("Unknown");
        info!("{} 删除了主题", author_id);
    }

    /// Called when an open forum post is created.
    async fn open_forum_post_create(&self, _ctx: Context, open_forum_thread: OpenThread) {
        let author_id = open_forum_thread.author_id.as_deref().unwrap_or("Unknown");
        info!("{} 创建了帖子", author_id);
    }

    /// Called when an open forum post is deleted.
    async fn open_forum_post_delete(&self, _ctx: Context, open_forum_thread: OpenThread) {
        let author_id = open_forum_thread.author_id.as_deref().unwrap_or("Unknown");
        info!("{} 删除了帖子", author_id);
    }

    /// Called when an open forum reply is created.
    async fn open_forum_reply_create(&self, _ctx: Context, open_forum_thread: OpenThread) {
        let author_id = open_forum_thread.author_id.as_deref().unwrap_or("Unknown");
        info!("{} 发表了评论", author_id);
    }

    /// Called when an open forum reply is deleted.
    async fn open_forum_reply_delete(&self, _ctx: Context, open_forum_thread: OpenThread) {
        let author_id = open_forum_thread.author_id.as_deref().unwrap_or("Unknown");
        info!("{} 删除了评论", author_id);
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

    info!("Starting open forum event demo...");

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

    // Set up intents - we want to receive open forum events
    // This is equivalent to: intents = botpy.Intents(open_forum_event=True)
    let intents = Intents::default().with_open_forum_event();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = OpenForumEventHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}
