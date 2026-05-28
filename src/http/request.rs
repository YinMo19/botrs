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

    pub(crate) async fn request_json_url<Q, B>(
        &self,
        token: &Token,
        method: Method,
        url: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        self.request_json_url_with_headers(method, token, url, query, body, HeaderMap::new())
            .await
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

    pub(crate) async fn put_raw_with_headers<Q>(
        &self,
        token: &Token,
        path: &str,
        query: Option<&Q>,
        body: impl Into<String>,
        headers: HeaderMap,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
    {
        self.request_raw_with_headers(Method::PUT, token, path, query, body, headers)
            .await
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

    pub(crate) async fn delete_with_body<Q, B>(
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
        self.request(Method::DELETE, token, path, query, body).await
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
        let url = format!("{}{}", self.base_url, path);
        self.request_json_url_with_headers(method, token, &url, query, body, headers)
            .await
    }

    pub(crate) async fn request_json_url_with_headers<Q, B>(
        &self,
        method: Method,
        token: &Token,
        url: &str,
        query: Option<&Q>,
        body: Option<&B>,
        headers: HeaderMap,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        debug!("Making {} request to: {}", method, url);

        let mut headers = self.authorized_headers(token, headers).await?;
        if body.is_some() {
            headers.insert("Content-Type", "application/json".parse().unwrap());
        }

        let mut request = self.client.request(method.clone(), url).headers(headers);

        if let Some(q) = query {
            request = request.query(q);
        }

        if let Some(b) = body {
            request = request.json(b);
        }

        let response = request.send().await.map_err(BotError::Http)?;

        self.handle_response(response).await
    }

    pub(crate) async fn request_raw_with_headers<Q>(
        &self,
        method: Method,
        token: &Token,
        path: &str,
        query: Option<&Q>,
        body: impl Into<String>,
        headers: HeaderMap,
    ) -> Result<serde_json::Value>
    where
        Q: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        debug!("Making {} request to: {}", method, url);

        let mut headers = self.authorized_headers(token, headers).await?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut request = self.client.request(method.clone(), &url).headers(headers);

        if let Some(q) = query {
            request = request.query(q);
        }

        let response = request
            .body(body.into())
            .send()
            .await
            .map_err(BotError::Http)?;

        self.handle_response(response).await
    }

    pub(crate) async fn authorized_headers(
        &self,
        token: &Token,
        mut headers: HeaderMap,
    ) -> Result<HeaderMap> {
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
        Ok(headers)
    }
}
