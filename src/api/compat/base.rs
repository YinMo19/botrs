use crate::api::{APIVersion, BotApi};
use crate::error::Result;
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use std::time::Duration;

impl BotApi {
    /// Setup constructor.
    #[allow(non_snake_case)]
    pub fn Setup(
        bot_app_id: impl Into<String>,
        secret: impl Into<String>,
        in_sandbox: bool,
    ) -> Result<(Self, Token)> {
        Self::setup(bot_app_id, secret, in_sandbox)
    }

    /// OpenAPI version method.
    #[allow(non_snake_case)]
    pub const fn Version(&self) -> APIVersion {
        self.version()
    }

    /// Timeout configuration method.
    #[allow(non_snake_case)]
    pub fn WithTimeout(&self, duration: Duration) -> Result<Self> {
        self.with_timeout(duration)
    }

    /// Debug configuration method.
    #[allow(non_snake_case)]
    pub fn SetDebug(&self, debug: bool) -> Self {
        self.set_debug(debug)
    }

    /// App ID accessor for the v1 OpenAPI implementation.
    #[allow(non_snake_case)]
    pub fn GetAppID(&self) -> &str {
        self.get_app_id()
    }

    /// Transport passthrough.
    #[allow(non_snake_case)]
    pub async fn Transport<B>(&self, method: Method, url: &str, body: Option<&B>) -> Result<Vec<u8>>
    where
        B: Serialize + ?Sized,
    {
        self.transport(self.token_required()?, method, url, body)
            .await
    }

    /// Trace ID accessor.
    #[allow(non_snake_case)]
    pub fn TraceID(&self) -> String {
        self.trace_id()
    }
}
