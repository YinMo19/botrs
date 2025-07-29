//! Demo: Schedule
//!
//! This example demonstrates how to create a bot that manages schedules.
//! It's equivalent to the Python demo_schedule.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that manages schedules.
struct ScheduleHandler;

// Channel schedule ID - modify this to your own channel's schedule sub-channel ID
const _CHANNEL_SCHEDULE_ID: &str = "12333";

#[async_trait::async_trait]
impl EventHandler for ScheduleHandler {
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

        // Schedule ID - can be filled in or obtained after sending "/创建日程" command
        let schedule_id = String::new(); // 日程ID，可以填写或者发送/创建日程 命令后获取

        info!("receive message {}", content);

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}收到你的@消息了: {content}");

        // Reply to the message first
        match message.reply(&ctx.api, &ctx.token, &reply_content).await {
            Ok(_) => info!("Successfully replied to message"),
            Err(e) => warn!("Failed to reply to message: {}", e),
        }

        // Calculate time delays (equivalent to Python time calculations)
        let delay = 1000 * 60; // 1 minute in milliseconds
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let start_time = now + delay;
        let end_time = start_time + delay;

        // Handle different schedule commands
        if content.contains("/创建日程") {
            // Create schedule (equivalent to self.api.create_schedule)
            match ctx
                .api
                .create_schedule(
                    &ctx.token,
                    _CHANNEL_SCHEDULE_ID,
                    "test",
                    &start_time.to_string(),
                    &end_time.to_string(),
                    _CHANNEL_SCHEDULE_ID,
                    botrs::models::schedule::RemindType::None,
                )
                .await
            {
                Ok(schedule) => {
                    info!("Successfully created schedule: {:?}", schedule);
                    // if let Some(id) = &schedule.id {
                    //     schedule_id = id.clone();
                    // }
                }
                Err(e) => {
                    warn!("Failed to create schedule: {}", e);
                }
            }
        } else if content.contains("/查询日程") {
            // Get schedule (equivalent to self.api.get_schedule)
            if !schedule_id.is_empty() {
                match ctx
                    .api
                    .get_schedule(&ctx.token, _CHANNEL_SCHEDULE_ID, &schedule_id)
                    .await
                {
                    Ok(schedule) => {
                        info!("Schedule details: {:?}", schedule);
                    }
                    Err(e) => {
                        warn!("Failed to get schedule: {}", e);
                    }
                }
            } else {
                warn!("No schedule_id available for query");
            }
        } else if content.contains("/更新日程") {
            // Update schedule (equivalent to self.api.update_schedule)
            if !schedule_id.is_empty() {
                match ctx
                    .api
                    .update_schedule(
                        &ctx.token,
                        _CHANNEL_SCHEDULE_ID,
                        &schedule_id,
                        "update",
                        &start_time.to_string(),
                        &end_time.to_string(),
                        _CHANNEL_SCHEDULE_ID,
                        botrs::models::schedule::RemindType::None,
                    )
                    .await
                {
                    Ok(result) => {
                        info!("Successfully updated schedule: {:?}", result);
                    }
                    Err(e) => {
                        warn!("Failed to update schedule: {}", e);
                    }
                }
            } else {
                warn!("No schedule_id available for update");
            }
        } else if content.contains("/删除日程") {
            // Delete schedule (equivalent to self.api.delete_schedule)
            if !schedule_id.is_empty() {
                match ctx
                    .api
                    .delete_schedule(&ctx.token, _CHANNEL_SCHEDULE_ID, &schedule_id)
                    .await
                {
                    Ok(result) => {
                        info!("Successfully deleted schedule: {:?}", result);
                    }
                    Err(e) => {
                        warn!("Failed to delete schedule: {}", e);
                    }
                }
            } else {
                warn!("No schedule_id available for deletion");
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

    info!("Starting schedule demo...");

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
        panic!("Invalid token: {e}");
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive public guild messages (@ mentions)
    // This is equivalent to: intents = botpy.Intents(public_guild_messages=True)
    let intents = Intents::default().with_public_guild_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = ScheduleHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}
