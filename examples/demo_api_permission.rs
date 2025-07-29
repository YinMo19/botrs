//! Demo: API Permission
//!
//! This example demonstrates how to create a bot that manages API permissions.
//! It's equivalent to the Python demo_api_permission.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that manages API permissions.
struct ApiPermissionHandler;

#[async_trait::async_trait]
impl EventHandler for ApiPermissionHandler {
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

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}创建日程{content}");

        // Reply to the message first
        match message.reply(&ctx.api, &ctx.token, &reply_content).await {
            Ok(_) => info!("Successfully replied to message"),
            Err(e) => warn!("Failed to reply to message: {}", e),
        }

        // Get guild ID for permission operations
        let _guild_id = match &message.guild_id {
            Some(guild_id) => guild_id,
            None => {
                warn!("Message has no guild_id");
                return;
            }
        };

        // Get channel ID for permission operations
        let _channel_id = match &message.channel_id {
            Some(channel_id) => channel_id,
            None => {
                warn!("Message has no channel_id");
                return;
            }
        };

        // Handle different permission commands
        if content.contains("/权限列表") {
            // Get permissions list (equivalent to self.api.get_permissions)
            match ctx.api.get_permissions(&ctx.token, _guild_id).await {
                Ok(apis) => {
                    for api in apis {
                        let desc = api.desc.as_deref().unwrap_or("Unknown");
                        let auth_status = api.auth_status.unwrap_or(0);
                        info!("api: {}, status: {}", desc, auth_status);
                    }
                }
                Err(e) => {
                    warn!("Failed to get permissions: {}", e);
                }
            }
        }

        if content.contains("/请求权限") {
            // Create permission demand (equivalent to self.api.post_permission_demand)
            let demand_identity =
                botrs::models::permission::APIPermissionDemandIdentify::guild_members();

            match ctx
                .api
                .post_permission_demand(
                    &ctx.token,
                    _guild_id,
                    _channel_id,
                    demand_identity,
                    "获取当前频道成员信息",
                )
                .await
            {
                Ok(demand) => {
                    let title = demand.title.as_deref().unwrap_or("Unknown");
                    let desc = &demand.desc;
                    info!("api title: {}, desc: {}", title, desc);
                }
                Err(e) => {
                    warn!("Failed to post permission demand: {}", e);
                }
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

    info!("Starting API permission demo...");

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
    let handler = ApiPermissionHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}
