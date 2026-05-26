use super::{APIVersion, APIv1, BotApi};
use crate::error::Result;
use crate::http::HttpClient;
use crate::options::{OpenApiOption, Options};
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

impl BotApi {
    /// Creates a Bot API client backed by the provided HTTP client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::api::BotApi;
    /// use botrs::http::HttpClient;
    ///
    /// let http = HttpClient::new(30, false).unwrap();
    /// let api = BotApi::new(http);
    /// ```
    pub fn new(http: HttpClient) -> Self {
        Self {
            http,
            app_id: String::new(),
            token: None,
        }
    }

    /// Creates a Bot API client that carries its own token.
    pub fn with_token(http: HttpClient, token: Token) -> Self {
        let app_id = token.app_id().to_string();
        Self {
            http: http.with_union_app_id(&app_id),
            app_id,
            token: Some(token),
        }
    }

    /// Creates a new instance from this client as an OpenAPI template.
    pub fn setup_from_template(
        &self,
        bot_app_id: impl Into<String>,
        token: Token,
        in_sandbox: bool,
    ) -> Result<Self> {
        let app_id = bot_app_id.into();
        Ok(Self {
            http: self
                .http
                .with_sandbox(in_sandbox)?
                .with_union_app_id(&app_id),
            app_id,
            token: Some(token),
        })
    }

    /// Creates a configured API client and token in one step.
    pub fn setup(
        bot_app_id: impl Into<String>,
        secret: impl Into<String>,
        in_sandbox: bool,
    ) -> Result<(Self, Token)> {
        let token = Token::new(bot_app_id, secret);
        let api = Self::new(HttpClient::new(crate::DEFAULT_TIMEOUT, in_sandbox)?);
        let app_id = token.app_id().to_string();
        Ok((
            api.setup_from_template(app_id, token.clone(), in_sandbox)?,
            token,
        ))
    }

    /// Returns the OpenAPI version implemented by this client.
    pub const fn version(&self) -> APIVersion {
        APIv1
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
    pub fn token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    /// Returns the bot app ID stored on this OpenAPI instance.
    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }

    pub(crate) fn token_required(&self) -> Result<&Token> {
        self.token.as_ref().ok_or_else(|| {
            crate::BotError::config(
                "BotApi has no stored token; use NewOpenAPI/NewSandboxOpenAPI or explicit-token methods",
            )
        })
    }

    pub(crate) fn url_with_options(&self, path: &str, options: &Options) -> String {
        options
            .url
            .clone()
            .unwrap_or_else(|| format!("{}{}", self.http.base_url(), path))
    }

    pub(crate) fn no_options() -> Vec<OpenApiOption> {
        Vec::new()
    }

    pub(crate) fn decode_json<T>(response: Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(response).map_err(Into::into)
    }

    pub(crate) async fn request_url_json<T, Q, B>(
        &self,
        token: &Token,
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
            .request_json_url(token, method, url, query, body)
            .await?;
        Self::decode_json(response)
    }

    pub(crate) async fn request_json<T, Q, B>(
        &self,
        token: &Token,
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
        self.request_url_json(token, method, &url, query, body)
            .await
    }

    pub(crate) async fn request_options_json<T, Q, B>(
        &self,
        options: &Options,
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
        let url = self.url_with_options(path, options);
        self.request_url_json(self.token_required()?, method, &url, query, body)
            .await
    }

    pub(crate) fn hide_tip_query(hide_tip: bool) -> Option<HashMap<&'static str, String>> {
        hide_tip.then(|| HashMap::from([("hidetip", "true".to_string())]))
    }

    pub(crate) fn recall_hide_tip_query(hide_tip: Option<bool>) -> HashMap<&'static str, String> {
        HashMap::from([(
            "hidetip",
            if hide_tip.unwrap_or(false) {
                "true"
            } else {
                "false"
            }
            .to_string(),
        )])
    }

    /// Passes through an arbitrary request to a full URL.
    pub async fn transport<B>(
        &self,
        token: &Token,
        method: Method,
        url: &str,
        body: Option<&B>,
    ) -> Result<Vec<u8>>
    where
        B: Serialize + ?Sized,
    {
        self.http.transport(token, method, url, body).await
    }

    /// Returns the last OpenAPI trace ID observed by the underlying HTTP client.
    pub fn trace_id(&self) -> String {
        self.http.trace_id()
    }

    /// Gets the HTTP client reference.
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Closes the API client and cleans up resources.
    pub async fn close(&self) {
        self.http.close().await;
    }
}
