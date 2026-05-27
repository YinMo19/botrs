use super::HttpClient;
use crate::error::{BotError, Result, http_error_from_status};
use crate::models::api::{ApiError, RateLimit};
use reqwest::{Response, StatusCode};
use serde_json::Value;
use tracing::{debug, error, warn};

impl HttpClient {
    /// Handles the HTTP response and converts it to a JSON value.
    pub(crate) async fn handle_response(&self, response: Response) -> Result<serde_json::Value> {
        let status = response.status();
        let headers = response.headers().clone();
        self.store_trace_id(&headers);

        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = headers
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            warn!("Rate limited, retry after {} seconds", retry_after);
            return Err(BotError::rate_limit(retry_after));
        }

        let body = response.text().await.map_err(BotError::Http)?;
        if is_success_status(status) && body.trim().is_empty() {
            debug!("Request successful, empty response body");
            return Ok(Value::Null);
        }

        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            error!("Response body: {}", body);
            BotError::Json(e)
        })?;

        if !is_success_status(status) {
            let api_error = self.parse_api_error(status, &json);
            error!("API error: {}", api_error);
            return Err(http_error_from_status(status.as_u16(), api_error.message));
        }

        if let Some(rate_limit) = self.parse_rate_limit(&headers) {
            debug!("Rate limit info: {:?}", rate_limit);
        }

        debug!("Request successful, response: {}", json);
        Ok(json)
    }

    pub(crate) async fn handle_bytes_response(&self, response: Response) -> Result<Vec<u8>> {
        let status = response.status();
        let headers = response.headers().clone();
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
        if !is_success_status(status) {
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

    pub(crate) fn store_trace_id(&self, headers: &reqwest::header::HeaderMap) {
        let trace_id = headers
            .get(crate::constant::HEADER_TRACE_ID)
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
    pub(crate) fn parse_api_error(&self, status: StatusCode, json: &serde_json::Value) -> ApiError {
        if let Ok(error) = serde_json::from_value::<ApiError>(json.clone()) {
            return error;
        }

        let code = json
            .get("err_code")
            .or_else(|| json.get("code"))
            .and_then(|c| c.as_u64())
            .map(|c| c as u32)
            .unwrap_or(status.as_u16() as u32);
        let err_code = json
            .get("err_code")
            .and_then(|c| c.as_u64())
            .map(|c| c as u32);

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

        ApiError {
            code,
            err_code,
            message,
            errors: Some(json.clone()),
            trace_id,
        }
    }

    /// Parses rate limit information from response headers.
    pub(crate) fn parse_rate_limit(
        &self,
        headers: &reqwest::header::HeaderMap,
    ) -> Option<RateLimit> {
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
}

fn is_success_status(status: StatusCode) -> bool {
    matches!(status.as_u16(), 200 | 204)
}
