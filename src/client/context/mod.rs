use super::prelude::*;

mod announce;
mod audio;
mod channel;
mod direct_message;
mod files;
mod forum;
mod guild;
mod interaction;
mod me;
mod message;
mod pins;
mod reaction;
mod schedule;
mod webhook;

/// Context passed to event handlers containing API access and bot information.
#[derive(Clone)]
pub struct Context {
    /// API client for making requests
    pub api: Arc<BotApi>,
    /// Authentication token
    pub token: Token,
    /// Bot information
    pub bot_info: Option<BotInfo>,
}

impl Context {
    pub fn new(api: Arc<BotApi>, token: Token) -> Self {
        Self {
            api,
            token,
            bot_info: None,
        }
    }

    /// Sets the bot information.

    pub fn with_bot_info(mut self, bot_info: BotInfo) -> Self {
        self.bot_info = Some(bot_info);
        self
    }
}
