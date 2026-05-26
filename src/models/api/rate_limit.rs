use serde::{Deserialize, Serialize};

/// Rate limit information from API headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RateLimit {
    /// The rate limit bucket
    pub bucket: Option<String>,
    /// The number of requests allowed per window
    pub limit: u32,
    /// The number of requests remaining in the current window
    pub remaining: u32,
    /// The time when the rate limit resets (Unix timestamp)
    pub reset: u64,
    /// The time after which to retry (in seconds)
    pub retry_after: Option<u64>,
}

impl RateLimit {
    /// Returns true if the rate limit has been exceeded.
    pub fn is_exceeded(&self) -> bool {
        self.remaining == 0
    }

    /// Returns the time until the rate limit resets (in seconds).
    pub fn reset_in(&self) -> u64 {
        let now = chrono::Utc::now().timestamp() as u64;
        self.reset.saturating_sub(now)
    }
}
