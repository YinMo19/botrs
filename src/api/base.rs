use super::BotApi;
use crate::error::Result;
use crate::http::HttpClient;
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

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

    /// Returns the token stored for OpenAPI calls.
    pub fn token(&self) -> &Token {
        &self.token
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
        let url = format!("{}{}", self.http.base_url, path);
        self.request_url_json(method, &url, query, body).await
    }

    pub(crate) fn hide_tip_query(hide_tip: bool) -> Option<HashMap<&'static str, String>> {
        hide_tip.then(|| HashMap::from([("hidetip", "true".to_string())]))
    }
}
