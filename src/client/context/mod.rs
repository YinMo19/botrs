use super::prelude::*;
use std::ops::Deref;

/// Context passed to event handlers containing API access and bot information.
#[derive(Clone)]
pub struct Context {
    /// API client for making requests
    api: Arc<BotApi>,
    /// Bot information
    pub bot_info: Option<BotInfo>,
}

impl Context {
    pub fn new(api: Arc<BotApi>) -> Self {
        Self {
            api,
            bot_info: None,
        }
    }

    /// Returns the shared API client.
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    pub(crate) fn api_clone(&self) -> BotApi {
        self.api.as_ref().clone()
    }

    /// Sets the bot information.
    pub fn with_bot_info(mut self, bot_info: BotInfo) -> Self {
        self.bot_info = Some(bot_info);
        self
    }
}

impl Deref for Context {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
