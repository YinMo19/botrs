use crate::error::{BotError, Result};
use reqwest::Client;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tracing::debug;

/// HTTP client for the QQ Guild Bot API.
#[derive(Clone)]
pub struct HttpClient {
    /// The underlying reqwest client
    pub(crate) client: Client,
    /// The base URL for API requests
    pub(crate) base_url: String,
    /// Whether to use sandbox environment
    pub(crate) is_sandbox: bool,
    /// Request timeout
    pub(crate) timeout: Duration,
    /// Last trace ID returned by OpenAPI.
    pub(crate) last_trace_id: Arc<RwLock<Option<String>>>,
    /// Whether verbose HTTP debug logging is enabled.
    pub(crate) debug: bool,
    /// OpenAPI instance app ID used by the X-Union-Appid header.
    pub(crate) union_app_id: Option<String>,
}

impl HttpClient {
    /// Creates a new HTTP client.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Request timeout in seconds
    /// * `is_sandbox` - Whether to use sandbox environment
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::http::HttpClient;
    ///
    /// let client = HttpClient::new(30, false).unwrap();
    /// ```
    pub fn new(timeout: u64, is_sandbox: bool) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .user_agent(format!("BotRS/{}", crate::VERSION))
            .build()
            .map_err(BotError::Http)?;

        let base_url = if is_sandbox {
            crate::SANDBOX_API_URL.to_string()
        } else {
            crate::DEFAULT_API_URL.to_string()
        };

        Ok(Self {
            client,
            base_url,
            is_sandbox,
            timeout: Duration::from_secs(timeout),
            last_trace_id: Arc::new(RwLock::new(None)),
            debug: false,
            union_app_id: None,
        })
    }

    pub(crate) fn clone_with_client(&self, client: Client, timeout: Duration, debug: bool) -> Self {
        Self {
            client,
            base_url: self.base_url.clone(),
            is_sandbox: self.is_sandbox,
            timeout,
            last_trace_id: Arc::clone(&self.last_trace_id),
            debug,
            union_app_id: self.union_app_id.clone(),
        }
    }

    /// Returns a client with a different request timeout.
    pub fn with_timeout(&self, timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(format!("BotRS/{}", crate::VERSION))
            .build()
            .map_err(BotError::Http)?;
        Ok(self.clone_with_client(client, timeout, self.debug))
    }

    /// Returns a client with debug logging toggled.
    pub fn with_debug(&self, debug: bool) -> Self {
        Self {
            debug,
            ..self.clone()
        }
    }

    /// Returns a client for the requested API environment while preserving
    /// timeout and debug settings.
    pub fn with_sandbox(&self, is_sandbox: bool) -> Result<Self> {
        Self::new(self.timeout.as_secs(), is_sandbox).map(|client| {
            if self.debug {
                client.with_debug(true)
            } else {
                client
            }
        })
    }

    /// Returns a client that sends the X-Union-Appid header for OpenAPI calls.
    pub fn with_union_app_id(&self, app_id: impl Into<String>) -> Self {
        Self {
            union_app_id: Some(app_id.into()),
            ..self.clone()
        }
    }

    /// Gets the base URL being used by this client.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns true if this client is using the sandbox environment.
    pub fn is_sandbox(&self) -> bool {
        self.is_sandbox
    }

    /// Gets the configured timeout.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Returns the most recent OpenAPI trace ID.
    pub fn trace_id(&self) -> String {
        self.last_trace_id
            .read()
            .ok()
            .and_then(|trace_id| trace_id.clone())
            .unwrap_or_default()
    }

    /// Returns whether debug logging is enabled.
    pub fn debug_enabled(&self) -> bool {
        self.debug
    }

    /// Returns the app ID configured for the X-Union-Appid header.
    pub fn union_app_id(&self) -> Option<&str> {
        self.union_app_id.as_deref()
    }

    /// Closes the HTTP client and cleans up resources.
    pub async fn close(&self) {
        // reqwest::Client doesn't need explicit cleanup.
        debug!("HTTP client closed");
    }
}

impl std::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpClient")
            .field("base_url", &self.base_url)
            .field("is_sandbox", &self.is_sandbox)
            .field("timeout", &self.timeout)
            .field("debug", &self.debug)
            .finish()
    }
}
