//! Demo: Audio or Live Channel Member
//!
//! This example demonstrates how to create a bot that handles audio or live channel member events.
//! It's equivalent to the Python demo_audio_or_live_channel_member.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, PublicAudio, PublicAudioType, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to audio or live channel member events.
struct AudioOrLiveChannelMemberHandler;

#[async_trait::async_trait]
impl EventHandler for AudioOrLiveChannelMemberHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a user enters an audio or live channel.
    async fn audio_or_live_channel_member_enter(&self, _ctx: Context, public_audio: PublicAudio) {
        // Get user ID for logging
        let user_id = public_audio.user_id.as_deref().unwrap_or("Unknown");

        // Check channel type and log accordingly (equivalent to Python version)
        match public_audio.channel_type {
            Some(PublicAudioType::Voice) => {
                info!("{} 加入了音视频子频道", user_id);
            }
            Some(PublicAudioType::Live) => {
                info!("{} 加入了直播子频道", user_id);
            }
            None => {
                warn!("Unknown channel type for user: {}", user_id);
            }
        }
    }

    /// Called when a user exits an audio or live channel.
    async fn audio_or_live_channel_member_exit(&self, _ctx: Context, public_audio: PublicAudio) {
        // Get user ID for logging
        let user_id = public_audio.user_id.as_deref().unwrap_or("Unknown");

        // Check channel type and log accordingly (equivalent to Python version)
        match public_audio.channel_type {
            Some(PublicAudioType::Voice) => {
                info!("{} 退出了音视频子频道", user_id);
            }
            Some(PublicAudioType::Live) => {
                info!("{} 退出了直播子频道", user_id);
            }
            None => {
                warn!("Unknown channel type for user: {}", user_id);
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

    info!("Starting audio or live channel member demo...");

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

    // Set up intents - we want to receive audio or live channel member events
    // This is equivalent to: intents = botpy.Intents(audio_or_live_channel_member=True)
    let intents = Intents::default().with_audio_or_live_channel_member();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = AudioOrLiveChannelMemberHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}
