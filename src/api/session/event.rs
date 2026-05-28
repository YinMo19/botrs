use std::{ops::Deref, sync::Arc};

use crate::api_impl::BotApi;
use crate::client::Context;
use crate::models::api::BotInfo;

/// Stateful API surface for handling one non-reply gateway event.
#[derive(Clone)]
pub struct EventSession<T> {
    ctx: Context,
    event: T,
}

impl<T> EventSession<T> {
    pub(crate) fn new(ctx: Context, event: T) -> Self {
        Self { ctx, event }
    }

    /// Returns the API client backing this session.
    pub fn api(&self) -> &BotApi {
        self.ctx.api()
    }

    /// Returns an owned handle to the shared API client.
    pub fn api_handle(&self) -> Arc<BotApi> {
        self.ctx.api_handle()
    }

    /// Returns bot information captured when the event was dispatched.
    pub fn bot_info(&self) -> Option<&BotInfo> {
        self.ctx.bot_info.as_ref()
    }

    /// Returns the event payload this session is bound to.
    pub fn event(&self) -> &T {
        &self.event
    }

    /// Consumes this session and returns the event payload.
    pub fn into_event(self) -> T {
        self.event
    }
}

impl<T> Deref for EventSession<T> {
    type Target = BotApi;

    fn deref(&self) -> &Self::Target {
        self.api()
    }
}
