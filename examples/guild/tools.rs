//! Guild Tools
//!
//! Demonstrates message lookup/editing, setting guides, channel announcements,
//! and guarded destructive operations from a normal bot event handler.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::prelude::*;
use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct GuildToolsHandler;

#[async_trait::async_trait]
impl EventHandler for GuildToolsHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        let content = message.content.trim();
        if !content.starts_with("/tools") {
            return;
        }

        let channel_id = message.channel_id.as_str();

        let parts = content.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["/tools", "recent"] => {
                let pager = MessagesPager::new(Some(5));
                match session.list_messages(channel_id, &pager).await {
                    Ok(messages) => {
                        let ids = messages
                            .iter()
                            .map(|message| message.id.as_str())
                            .collect::<Vec<_>>()
                            .join("\n");
                        let reply = if ids.is_empty() {
                            "No recent message IDs returned".to_string()
                        } else {
                            format!("Recent message IDs:\n{ids}")
                        };
                        let _ = session.reply(reply).await;
                    }
                    Err(err) => warn!("list messages failed: {}", err),
                }
            }
            ["/tools", "get", message_id] => {
                match session.get_message(channel_id, message_id).await {
                    Ok(found) => {
                        let text = if found.content.is_empty() {
                            "<empty>"
                        } else {
                            found.content.as_str()
                        };
                        let _ = session.reply(text).await;
                    }
                    Err(err) => warn!("get message failed: {}", err),
                }
            }
            ["/tools", "edit", message_id, rest @ ..] if !rest.is_empty() => {
                let params = MessageParams::new_text(rest.join(" "));
                match session.update_message(channel_id, message_id, params).await {
                    Ok(updated) => info!("updated message {:?}", updated.id),
                    Err(err) => warn!("update message failed: {}", err),
                }
            }
            ["/tools", "guide", user_id] => {
                let params = SettingGuideParams::for_users([*user_id]);
                if let Err(err) = session.send_setting_guide(channel_id, params).await {
                    warn!("send setting guide failed: {}", err);
                }
            }
            ["/tools", "announce", message_id] if destructive_examples_enabled() => {
                if let Err(err) = session
                    .create_channel_announce(channel_id, message_id)
                    .await
                {
                    warn!("create channel announce failed: {}", err);
                }
            }
            ["/tools", "recall", message_id] if destructive_examples_enabled() => {
                if let Err(err) = session
                    .recall_message(channel_id, message_id, Some(true))
                    .await
                {
                    warn!("recall message failed: {}", err);
                }
            }
            ["/tools", "clean_pins"] if destructive_examples_enabled() => {
                if let Err(err) = session.clean_pins(channel_id).await {
                    warn!("clean pins failed: {}", err);
                }
            }
            _ => {
                let help = "Commands: /tools recent | /tools get <message_id> | /tools edit <message_id> <text> | /tools guide <user_id>";
                let _ = session.reply(help).await;
            }
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

fn destructive_examples_enabled() -> bool {
    env::var("BOTRS_ALLOW_DESTRUCTIVE_EXAMPLES").is_ok_and(|value| value == "1")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;

    let token = Token::new(config.bot.app_id, config.bot.secret);
    let intents = Intents::new().with_public_guild_messages();
    let mut client = Client::new(token, intents, GuildToolsHandler, config.bot.sandbox)?;

    client.start().await?;
    Ok(())
}
