use super::BotApi;
use crate::error::Result;
use crate::http::HttpClient;
use crate::token_impl::Token;
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
    /// use botrs::BotApi;
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
            token,
        }
    }

    /// Returns a scoped client whose typed requests use this exact trusted URL.
    /// Use `api.with_url(url).send_message(...)` to override one request while
    /// sharing the connection pool and token cache with the original client.
    pub fn with_url(&self, url: url::Url) -> Self {
        let mut api = self.clone();
        api.http.endpoint = Some(url);
        api
    }

    /// Sends an authenticated OpenAPI request to an explicitly supplied endpoint.
    /// Supply a trusted URL: this request includes the bot credentials. The raw
    /// successful response exposes its headers, trace ID and bytes to the caller.
    ///
    /// Configure the body and headers with reqwest's request builder, e.g.
    /// `|r| r.json(&payload)` or `|r| r.body(bytes)`. Use `|r| r` for a bare GET.
    pub async fn transport(
        &self,
        method: Method,
        url: &url::Url,
        configure: impl FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        let request = configure(self.http.client.request(method, url.clone()));
        self.http.execute(self.token(), request).await
    }

    pub(crate) fn token(&self) -> &Token {
        &self.token
    }

    pub(crate) fn decode_json<T>(response: Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(response).map_err(Into::into)
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
        let response = self
            .http
            .request(method, self.token(), path, query, body)
            .await?;
        Self::decode_json(response)
    }

    pub(crate) fn hide_tip_query(hide_tip: bool) -> Option<HashMap<&'static str, String>> {
        hide_tip.then(|| HashMap::from([("hidetip", "true".to_string())]))
    }
}
