use super::HttpClient;
use crate::error::{BotError, Result};
use crate::token_impl::Token;
use reqwest::{
    Method,
    header::{HeaderMap, HeaderValue},
};
use serde::Serialize;
use tracing::debug;

impl HttpClient {
    pub(crate) async fn get<Q>(
        &self,
        token: &Token,
        path: &str,
        query: Option<&Q>,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
    {
        self.request(Method::GET, token, path, query, None::<&()>)
            .await
    }

    pub(crate) async fn post<Q, B>(
        &self,
        token: &Token,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        self.request(Method::POST, token, path, query, body).await
    }

    pub(crate) async fn put<Q, B>(
        &self,
        token: &Token,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        self.request(Method::PUT, token, path, query, body).await
    }

    pub(crate) async fn delete<Q>(
        &self,
        token: &Token,
        path: &str,
        query: Option<&Q>,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
    {
        self.request(Method::DELETE, token, path, query, None::<&()>)
            .await
    }

    pub(crate) async fn patch<Q, B>(
        &self,
        token: &Token,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        self.request(Method::PATCH, token, path, query, body).await
    }

    /// Makes a generic HTTP request to the API.
    pub(crate) async fn request<Q, B>(
        &self,
        method: Method,
        token: &Token,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        self.request_with_headers(method, token, path, query, body, HeaderMap::new())
            .await
    }

    pub(crate) async fn request_with_headers<Q, B>(
        &self,
        method: Method,
        token: &Token,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        headers: HeaderMap,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let url = self.endpoint.as_ref().map_or_else(
            || std::borrow::Cow::Owned(format!("{}{path}", self.base_url)),
            |url| std::borrow::Cow::Borrowed(url.as_str()),
        );
        let mut request = self.client.request(method, url.as_ref()).headers(headers);
        if let Some(query) = query {
            request = request.query(query);
        }
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = self.execute(token, request).await?;
        let bytes = response.bytes().await?;
        if bytes.iter().all(u8::is_ascii_whitespace) {
            return Ok(serde_json::Value::Null);
        }
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub(crate) async fn execute(
        &self,
        token: &Token,
        builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        let mut request = builder.build()?;
        debug!("Making {} request to: {}", request.method(), request.url());
        self.authorize(token, request.headers_mut()).await?;
        self.checked_response(self.client.execute(request).await?)
            .await
    }

    pub(crate) async fn authorize(&self, token: &Token, headers: &mut HeaderMap) -> Result<()> {
        headers.insert(
            "Authorization",
            token.authorization_header().await?.parse().map_err(|err| {
                BotError::internal(format!("invalid authorization header: {err}"))
            })?,
        );
        let app_id = self
            .union_app_id
            .as_deref()
            .unwrap_or_else(|| token.app_id());
        if !app_id.is_empty() {
            headers.insert(
                "X-Union-Appid",
                HeaderValue::from_str(app_id).map_err(|err| {
                    BotError::internal(format!("invalid X-Union-Appid header: {err}"))
                })?,
            );
        }
        Ok(())
    }
}
