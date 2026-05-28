use super::prelude::*;
use std::ops::Deref;

/// Internal runtime context used to construct event-scoped sessions.
#[derive(Clone)]
pub(crate) struct Context {
    /// API client for making requests
    api: Arc<BotApi>,
    /// Bot information
    pub(crate) bot_info: Option<BotInfo>,
}

impl Context {
    pub(crate) fn new(api: Arc<BotApi>) -> Self {
        Self {
            api,
            bot_info: None,
        }
    }

    /// Returns the shared API client.
    pub(crate) fn api(&self) -> &BotApi {
        &self.api
    }

    /// Returns an owned handle to the shared API client.
    pub(crate) fn api_handle(&self) -> Arc<BotApi> {
        Arc::clone(&self.api)
    }

    /// Sets the bot information.
    pub(crate) fn with_bot_info(mut self, bot_info: BotInfo) -> Self {
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
