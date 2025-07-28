//! HTTP client implementation for the QQ Guild Bot API.
//!
//! This module provides the HTTP client for making requests to the QQ Guild Bot API,
//! handling authentication, rate limiting, and error responses.

use crate::error::{http_error_from_status, BotError, Result};
use crate::models::api::{ApiError, RateLimit};
use crate::token::Token;
use reqwest::{Client, Method, Response, StatusCode};
use serde::Serialize;
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
        let url = format!("{}{}", self.base_url, path);
        debug!("Making {} request to: {}", method, url);

        let mut request = self.client.request(method, &url);

        // Add authorization header
        let auth_header = token.authorization_header().await?;
        request = request.header("Authorization", auth_header);

        // Add content type for requests with body
        if body.is_some() {
            request = request.header("Content-Type", "application/json");
        }

        // Add query parameters
        if let Some(q) = query {
            request = request.query(q);
        }

        // Add body
        if let Some(b) = body {
            request = request.json(b);
        }

        // Send the request
        let response = request.send().await.map_err(BotError::Http)?;

        self.handle_response(response).await
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
    async fn handle_response(&self, response: Response) -> Result<serde_json::Value> {
        let status = response.status();
        let headers = response.headers().clone();

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
        if !status.is_success() {
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
}
