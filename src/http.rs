//! HTTP client implementation for the QQ Guild Bot API.
//!
//! This module provides the HTTP client for making requests to the QQ Guild Bot API,
//! handling authentication, rate limiting, and error responses.

use crate::error::{BotError, Result, http_error_from_status};
use crate::models::api::{ApiError, RateLimit};
use crate::token::Token;
use reqwest::{Client, Method, Response, StatusCode, header::HeaderMap};
use serde::Serialize;
use serde_json::Value;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tracing::{debug, error, warn};

/// HTTP client for the QQ Guild Bot API.
#[derive(Clone)]
pub struct HttpClient {
    /// The underlying reqwest client
    client: Client,
    /// The base URL for API requests
    base_url: String,
    /// Whether to use sandbox environment
    is_sandbox: bool,
    /// Request timeout
    timeout: Duration,
    /// Last trace ID returned by OpenAPI.
    last_trace_id: Arc<RwLock<Option<String>>>,
    /// Whether verbose HTTP debug logging is enabled.
    debug: bool,
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
        })
    }

    fn clone_with_client(&self, client: Client, timeout: Duration, debug: bool) -> Self {
        Self {
            client,
            base_url: self.base_url.clone(),
            is_sandbox: self.is_sandbox,
            timeout,
            last_trace_id: Arc::clone(&self.last_trace_id),
            debug,
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

    /// Makes a GET request to the API.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `path` - API endpoint path
    /// * `query` - Optional query parameters
    ///
    /// # Returns
    ///
    /// The response body as a JSON value.
    pub async fn get<Q>(
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

    /// Makes a POST request to the API.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `path` - API endpoint path
    /// * `query` - Optional query parameters
    /// * `body` - Request body
    ///
    /// # Returns
    ///
    /// The response body as a JSON value.
    pub async fn post<Q, B>(
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

    /// Makes a PUT request to the API.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `path` - API endpoint path
    /// * `query` - Optional query parameters
    /// * `body` - Request body
    ///
    /// # Returns
    ///
    /// The response body as a JSON value.
    pub async fn put<Q, B>(
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

    /// Makes a PUT request with additional headers.
    pub async fn put_with_headers<Q, B>(
        &self,
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
        self.request_with_headers(Method::PUT, token, path, query, body, headers)
            .await
    }

    /// Makes a PUT request with a raw request body and additional headers.
    pub async fn put_raw_with_headers<Q>(
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

    /// Makes a DELETE request to the API.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `path` - API endpoint path
    /// * `query` - Optional query parameters
    ///
    /// # Returns
    ///
    /// The response body as a JSON value.
    pub async fn delete<Q>(
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

    /// Makes a DELETE request with a JSON request body.
    pub async fn delete_with_body<Q, B>(
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

    /// Makes a PATCH request to the API.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `path` - API endpoint path
    /// * `query` - Optional query parameters
    /// * `body` - Request body
    ///
    /// # Returns
    ///
    /// The response body as a JSON value.
    pub async fn patch<Q, B>(
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

    /// Passes through an arbitrary request to the provided URL.
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
        let mut headers = self.authorized_headers(token, HeaderMap::new()).await?;
        if body.is_some() {
            headers.insert("Content-Type", "application/json".parse().unwrap());
        }

        let mut context = crate::openapi::FilterContext::request(method.clone(), url, headers);
        crate::openapi::DoReqFilterChains(&mut context)?;

        let request_headers = context.request_headers.clone();
        let mut request = self
            .client
            .request(method.clone(), url)
            .headers(request_headers.clone());
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request.send().await.map_err(BotError::Http)?;
        self.handle_bytes_response(response, method, url, request_headers)
            .await
    }

    /// Makes a generic HTTP request to the API.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method
    /// * `token` - Authentication token
    /// * `path` - API endpoint path
    /// * `query` - Optional query parameters
    /// * `body` - Optional request body
    ///
    /// # Returns
    ///
    /// The response body as a JSON value.
    async fn request<Q, B>(
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

    async fn request_with_headers<Q, B>(
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

    async fn request_json_url_with_headers<Q, B>(
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

        let mut context = crate::openapi::FilterContext::request(method.clone(), url, headers);
        crate::openapi::DoReqFilterChains(&mut context)?;
        let request_headers = context.request_headers.clone();

        let mut request = self
            .client
            .request(method.clone(), url)
            .headers(request_headers.clone());

        if let Some(q) = query {
            request = request.query(q);
        }

        if let Some(b) = body {
            request = request.json(b);
        }

        let response = request.send().await.map_err(BotError::Http)?;

        self.handle_response(response, method, url, request_headers)
            .await
    }

    async fn request_raw_with_headers<Q>(
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

        let mut context = crate::openapi::FilterContext::request(method.clone(), &url, headers);
        crate::openapi::DoReqFilterChains(&mut context)?;
        let request_headers = context.request_headers.clone();

        let mut request = self
            .client
            .request(method.clone(), &url)
            .headers(request_headers.clone());

        if let Some(q) = query {
            request = request.query(q);
        }

        let response = request
            .body(body.into())
            .send()
            .await
            .map_err(BotError::Http)?;

        self.handle_response(response, method, &url, request_headers)
            .await
    }

    async fn authorized_headers(&self, token: &Token, mut headers: HeaderMap) -> Result<HeaderMap> {
        headers.insert(
            "Authorization",
            token.authorization_header().await?.parse().map_err(|err| {
                BotError::internal(format!("invalid authorization header: {err}"))
            })?,
        );
        Ok(headers)
    }

    /// Handles the HTTP response and converts it to a JSON value.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response
    ///
    /// # Returns
    ///
    /// The response body as a JSON value or an error.
    async fn handle_response(
        &self,
        response: Response,
        method: Method,
        url: &str,
        request_headers: HeaderMap,
    ) -> Result<serde_json::Value> {
        let status = response.status();
        let mut headers = response.headers().clone();
        let mut context =
            crate::openapi::FilterContext::response(method, url, request_headers, status, headers);
        crate::openapi::DoRespFilterChains(&mut context)?;
        headers = context.response_headers;
        self.store_trace_id(&headers);

        // Check for rate limiting
        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = headers
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            warn!("Rate limited, retry after {} seconds", retry_after);
            return Err(BotError::rate_limit(retry_after));
        }

        // Get response body
        let body = response.text().await.map_err(BotError::Http)?;

        // Parse JSON
        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            error!("Response body: {}", body);
            BotError::Json(e)
        })?;

        // Check for API errors
        if !crate::openapi::IsSuccessStatus(status.as_u16()) {
            let api_error = self.parse_api_error(status, &json)?;
            error!("API error: {}", api_error);
            return Err(http_error_from_status(status.as_u16(), api_error.message));
        }

        // Log rate limit information if available
        if let Some(rate_limit) = self.parse_rate_limit(&headers) {
            debug!("Rate limit info: {:?}", rate_limit);
        }

        debug!("Request successful, response: {}", json);
        Ok(json)
    }

    async fn handle_bytes_response(
        &self,
        response: Response,
        method: Method,
        url: &str,
        request_headers: HeaderMap,
    ) -> Result<Vec<u8>> {
        let status = response.status();
        let mut headers = response.headers().clone();
        let mut context =
            crate::openapi::FilterContext::response(method, url, request_headers, status, headers);
        crate::openapi::DoRespFilterChains(&mut context)?;
        headers = context.response_headers;
        self.store_trace_id(&headers);

        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = headers
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            return Err(BotError::rate_limit(retry_after));
        }

        let body = response.bytes().await.map_err(BotError::Http)?.to_vec();
        if !crate::openapi::IsSuccessStatus(status.as_u16()) {
            let message = serde_json::from_slice::<Value>(&body)
                .ok()
                .and_then(|json| {
                    json.get("message")
                        .and_then(|value| value.as_str())
                        .map(ToOwned::to_owned)
                })
                .unwrap_or_else(|| String::from_utf8_lossy(&body).into_owned());
            return Err(http_error_from_status(status.as_u16(), message));
        }

        Ok(body)
    }

    fn store_trace_id(&self, headers: &reqwest::header::HeaderMap) {
        let trace_id = headers
            .get(crate::constant::HeaderTraceID)
            .or_else(|| headers.get("x-tps-trace-id"))
            .and_then(|value| value.to_str().ok())
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);

        if let Some(trace_id) = trace_id
            && let Ok(mut last_trace_id) = self.last_trace_id.write()
        {
            *last_trace_id = Some(trace_id);
        }
    }

    /// Parses an API error from the response.
    ///
    /// # Arguments
    ///
    /// * `status` - HTTP status code
    /// * `json` - Response JSON
    ///
    /// # Returns
    ///
    /// An `ApiError` instance.
    fn parse_api_error(&self, status: StatusCode, json: &serde_json::Value) -> Result<ApiError> {
        // Try to parse structured error response
        if let Ok(error) = serde_json::from_value::<ApiError>(json.clone()) {
            return Ok(error);
        }

        // Try to extract error information from different formats
        let code = json
            .get("code")
            .and_then(|c| c.as_u64())
            .map(|c| c as u32)
            .unwrap_or(status.as_u16() as u32);

        let message = json
            .get("message")
            .and_then(|m| m.as_str())
            .or_else(|| json.get("error").and_then(|e| e.as_str()))
            .unwrap_or_else(|| status.canonical_reason().unwrap_or("Unknown error"))
            .to_string();

        let trace_id = json
            .get("trace_id")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        Ok(ApiError {
            code,
            message,
            errors: Some(json.clone()),
            trace_id,
        })
    }

    /// Parses rate limit information from response headers.
    ///
    /// # Arguments
    ///
    /// * `headers` - Response headers
    ///
    /// # Returns
    ///
    /// Rate limit information if available.
    fn parse_rate_limit(&self, headers: &reqwest::header::HeaderMap) -> Option<RateLimit> {
        let limit = headers
            .get("x-ratelimit-limit")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())?;

        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())?;

        let reset = headers
            .get("x-ratelimit-reset")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok())?;

        let bucket = headers
            .get("x-ratelimit-bucket")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let retry_after = headers
            .get("retry-after")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse().ok());

        Some(RateLimit {
            bucket,
            limit,
            remaining,
            reset,
            retry_after,
        })
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

    /// Closes the HTTP client and cleans up resources.
    pub async fn close(&self) {
        // reqwest::Client doesn't need explicit cleanup
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new(30, false).unwrap();
        assert!(!client.is_sandbox());
        assert_eq!(client.timeout(), Duration::from_secs(30));
        assert_eq!(client.base_url(), crate::DEFAULT_API_URL);

        let sandbox_client = HttpClient::new(60, true).unwrap();
        assert!(sandbox_client.is_sandbox());
        assert_eq!(sandbox_client.base_url(), crate::SANDBOX_API_URL);
    }

    #[test]
    fn test_api_error_parsing() {
        let client = HttpClient::new(30, false).unwrap();

        let json = serde_json::json!({
            "code": 404,
            "message": "Not found",
            "trace_id": "test-trace"
        });

        let error = client
            .parse_api_error(StatusCode::NOT_FOUND, &json)
            .unwrap();
        assert_eq!(error.code, 404);
        assert_eq!(error.message, "Not found");
        assert_eq!(error.trace_id, Some("test-trace".to_string()));
    }

    #[test]
    fn test_rate_limit_parsing() {
        let client = HttpClient::new(30, false).unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-ratelimit-limit", "100".parse().unwrap());
        headers.insert("x-ratelimit-remaining", "50".parse().unwrap());
        headers.insert("x-ratelimit-reset", "1234567890".parse().unwrap());
        headers.insert("x-ratelimit-bucket", "global".parse().unwrap());

        let rate_limit = client.parse_rate_limit(&headers).unwrap();
        assert_eq!(rate_limit.limit, 100);
        assert_eq!(rate_limit.remaining, 50);
        assert_eq!(rate_limit.reset, 1234567890);
        assert_eq!(rate_limit.bucket, Some("global".to_string()));
    }

    #[test]
    fn test_trace_id_storage() {
        let client = HttpClient::new(30, false).unwrap();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Tps-trace-ID", "trace-123".parse().unwrap());

        client.store_trace_id(&headers);

        assert_eq!(client.trace_id(), "trace-123");
    }
}
