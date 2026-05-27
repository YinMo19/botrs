use super::BotApi;
use crate::error::Result;
use crate::http::HttpClient;
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

impl BotApi {
    /// Creates a Bot API client backed by the provided HTTP client and token.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::api::BotApi;
    /// use botrs::http::HttpClient;
    ///
    /// let http = HttpClient::new(30, false).unwrap();
    /// let token = botrs::Token::new("app_id", "secret");
    /// let api = BotApi::new(http, token);
    /// ```
    pub fn new(http: HttpClient, token: Token) -> Self {
        let app_id = token.app_id().to_string();
        Self {
            http: http.with_union_app_id(&app_id),
            app_id,
            token,
        }
    }

    /// Creates a new instance from this client as an OpenAPI template.
    pub fn setup_from_template(&self, token: Token, in_sandbox: bool) -> Result<Self> {
        let app_id = token.app_id().to_string();
        Ok(Self {
            http: self
                .http
                .with_sandbox(in_sandbox)?
                .with_union_app_id(&app_id),
            app_id,
            token,
        })
    }

    /// Creates a configured API client in one step.
    pub fn setup(
        bot_app_id: impl Into<String>,
        secret: impl Into<String>,
        in_sandbox: bool,
    ) -> Result<Self> {
        let token = Token::new(bot_app_id, secret);
        let http = HttpClient::new(crate::DEFAULT_TIMEOUT, in_sandbox)?;
        Ok(Self::new(http, token))
    }

    /// Returns a client configured with the given request timeout.
    pub fn with_timeout(&self, duration: Duration) -> Result<Self> {
        Ok(Self {
            http: self.http.with_timeout(duration)?,
            app_id: self.app_id.clone(),
            token: self.token.clone(),
        })
    }

    /// Returns a client with verbose HTTP debug logging toggled.
    pub fn set_debug(&self, debug: bool) -> Self {
        Self {
            http: self.http.with_debug(debug),
            app_id: self.app_id.clone(),
            token: self.token.clone(),
        }
    }

    /// Returns the token stored for OpenAPI calls.
    pub fn token(&self) -> &Token {
        &self.token
    }

    /// Returns the bot app ID stored on this OpenAPI instance.
    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }

    pub(crate) fn decode_json<T>(response: Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(response).map_err(Into::into)
    }

    pub(crate) async fn request_url_json<T, Q, B>(
        &self,
        method: Method,
        url: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let response = self
            .http
            .request_json_url(self.token(), method, url, query, body)
            .await?;
        Self::decode_json(response)
    }

    pub(crate) async fn request_json<T, Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.http.base_url(), path);
        self.request_url_json(method, &url, query, body).await
    }

    pub(crate) fn hide_tip_query(hide_tip: bool) -> Option<HashMap<&'static str, String>> {
        hide_tip.then(|| HashMap::from([("hidetip", "true".to_string())]))
    }

    /// Passes through an arbitrary request to a full URL.
    pub async fn transport<B>(&self, method: Method, url: &str, body: Option<&B>) -> Result<Vec<u8>>
    where
        B: Serialize + ?Sized,
    {
        self.http.transport(self.token(), method, url, body).await
    }

    /// Returns the last OpenAPI trace ID observed by the underlying HTTP client.
    pub fn trace_id(&self) -> String {
        self.http.trace_id()
    }

    /// Gets the HTTP client reference.
    pub fn http(&self) -> &HttpClient {
        &self.http
    }
}
