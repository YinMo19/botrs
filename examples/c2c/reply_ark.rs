//! C2C Reply ARK
//!
//! Replies to C2C messages with an ARK payload.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{Ark, ArkKv, C2CMessageParams};
use botrs::{C2CReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct C2CReplyArkHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyArkHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn c2c_message_create(&self, mut session: C2CReplySession) {
        let ark = Ark {
            template_id: Some(37),
            kv: Some(vec![
                ArkKv {
                    key: Some("#METATITLE#".to_string()),
                    value: Some("通知提醒".to_string()),
                    obj: None,
                },
                ArkKv {
                    key: Some("#METACOVER#".to_string()),
                    value: Some(
                        "https://vfiles.gtimg.cn/vupload/20211029/bf0ed01635493790634.jpg"
                            .to_string(),
                    ),
                    obj: None,
                },
            ]),
        };

        let params = C2CMessageParams {
            msg_type: 3,
            ark: Some(ark),
            ..Default::default()
        };

        match session.send_message(params).await {
            Ok(response) => info!("Successfully sent C2C ARK message: {:?}", response),
            Err(e) => warn!("Failed to send C2C ARK message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting C2C reply ARK example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_messages();
    let mut client = Client::new(token, intents, C2CReplyArkHandler, true)?;
    client.start().await?;
    Ok(())
}
