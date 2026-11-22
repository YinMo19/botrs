use crate::error::{BotError, Result};
use reqwest::Client;
use std::time::Duration;

/// HTTP client for the QQ Guild Bot API.
#[derive(Clone)]
pub struct HttpClient {
    /// The underlying reqwest client
    pub(crate) client: Client,
    /// The base URL for API requests
    pub(crate) base_url: String,
    pub(crate) endpoint: Option<url::Url>,
    /// OpenAPI instance app ID used by the X-Union-Appid header.
    pub(crate) union_app_id: Option<String>,
}

impl HttpClient {
    /// Creates an HTTP client for the production or sandbox API environment.
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
            endpoint: None,
            union_app_id: None,
        })
    }

    /// Selects a trusted OpenAPI base URL for typed requests, including message sends.
    pub fn with_base_url(mut self, url: url::Url) -> Result<Self> {
        if !matches!(url.scheme(), "http" | "https")
            || url.host().is_none()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(BotError::config(
                "expected an HTTP(S) API base URL without query or fragment",
            ));
        }
        self.base_url = url.as_str().trim_end_matches('/').to_owned();
        Ok(self)
    }

    pub(crate) fn with_union_app_id(&self, app_id: impl Into<String>) -> Self {
        Self {
            union_app_id: Some(app_id.into()),
            ..self.clone()
        }
    }
}

impl std::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpClient")
            .field("base_url", &self.base_url)
            .finish()
    }
}
