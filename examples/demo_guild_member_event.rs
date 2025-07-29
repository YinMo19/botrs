//! Demo: Guild Member Event
//!
//! This example demonstrates how to create a bot that handles guild member events.
//! It's equivalent to the Python demo_guild_member_event.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Member, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to guild member events.
struct GuildMemberEventHandler;

#[async_trait::async_trait]
impl EventHandler for GuildMemberEventHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a guild member is added.
    async fn guild_member_add(&self, ctx: Context, member: Member) {
        // Get member nickname for logging
        let nick = member.nick.as_deref().unwrap_or("Unknown");
        info!("{} 加入频道", nick);

        // Get user ID from the member (Member has an optional User)
        let user_id = match &member.user {
            Some(user) => &user.id,
            None => {
                warn!("Guild member has no user information");
                return;
            }
        };

        // Note: guild_id needs to be obtained from context or event data
        // For this demo, we'll use a placeholder since it's not available in Member
        let guild_id = "placeholder_guild_id"; // This should come from event context

        // Create DMS (equivalent to self.api.create_dms)
        match ctx.api.create_dms(&ctx.token, guild_id, user_id).await {
            Ok(_dms_payload) => {
                info!("发送私信");

                // Send welcome DM (equivalent to self.api.post_dms)
                let params = botrs::models::message::DirectMessageParams {
                    content: Some("welcome join guild".to_string()),
                    msg_id: None, // Member doesn't have event_id, this should come from event context
                    ..Default::default()
                };

                match ctx
                    .api
                    .post_dms_with_params(&ctx.token, guild_id, params)
                    .await
                {
                    Ok(_) => info!("Successfully sent welcome DM"),
                    Err(e) => warn!("Failed to send welcome DM: {}", e),
                }
            }
            Err(e) => warn!("Failed to create DMS: {}", e),
        }
    }

    /// Called when a guild member is updated.
    async fn guild_member_update(&self, _ctx: Context, member: Member) {
        let nick = member.nick.as_deref().unwrap_or("Unknown");
        info!("{} 更新了资料", nick);
    }

    /// Called when a guild member is removed.
    async fn guild_member_remove(&self, _ctx: Context, member: Member) {
        let nick = member.nick.as_deref().unwrap_or("Unknown");
        info!("{} 退出了频道", nick);
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

    info!("Starting guild member event demo...");

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

    // Set up intents - we want to receive guild member events
    // This is equivalent to: intents = botpy.Intents(guild_members=True)
    let intents = Intents::default().with_guild_members();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = GuildMemberEventHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}
