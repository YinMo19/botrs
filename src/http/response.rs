use super::HttpClient;
use crate::error::{HttpFailure, Result};
use crate::models::api::RateLimit;
use reqwest::{Response, StatusCode, header::HeaderMap};
use serde_json::Value;
use tracing::debug;

impl HttpClient {
    pub(crate) async fn checked_response(&self, response: Response) -> Result<Response> {
        let status = response.status();
        if is_success_status(status) {
            if let Some(limit) = self.parse_rate_limit(response.headers()) {
                debug!("Rate limit info: {:?}", limit);
            }
            return Ok(response);
        }
        let trace = response_trace_id(response.headers());
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok()?.parse().ok());
        let body = response.text().await?;
        let failure = parse_failure(status, trace, body);
        Err(failure.into_error(retry_after))
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

fn response_trace_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-tps-trace-id")
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

fn parse_failure(status: StatusCode, header_trace: Option<String>, body: String) -> HttpFailure {
    let json = serde_json::from_str::<Value>(&body).unwrap_or(Value::Null);
    let platform_code = ["err_code", "code"]
        .iter()
        .find_map(|key| u32::try_from(json.get(*key)?.as_u64()?).ok());
    let message = json
        .get("message")
        .and_then(Value::as_str)
        .or_else(|| json.get("error").and_then(Value::as_str))
        .unwrap_or_else(|| status.canonical_reason().unwrap_or("Unknown error"))
        .to_owned();
    let trace_id = json
        .get("trace_id")
        .and_then(Value::as_str)
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .or(header_trace);
    HttpFailure {
        status,
        platform_code,
        message,
        trace_id,
        body,
    }
}
