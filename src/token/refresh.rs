use super::Token;
use crate::error::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_EXPIRY_DELTA_MILLIS: u64 = 9_000;
const RAND_TIME_UPPER_LIMIT_MILLIS: u64 = 500;

pub async fn start_access_token_refresh(token: Token) -> Result<tokio::task::JoinHandle<()>> {
    token.ensure_valid_token().await?;

    Ok(tokio::spawn(async move {
        let mut consecutive_failures = 0;
        loop {
            let refresh_millis = if consecutive_failures > 0 {
                if consecutive_failures > 10 {
                    panic!("get token failed continuously for more than ten times");
                }
                1_000
            } else {
                token
                    .cached_expires_in()
                    .await
                    .map(get_refresh_millis)
                    .unwrap_or(1_000)
            };

            tracing::debug!("refresh after {} milli sec", refresh_millis);
            tokio::time::sleep(Duration::from_millis(refresh_millis)).await;

            match token.force_refresh_access_token().await {
                Ok(()) => consecutive_failures = 0,
                Err(err) => {
                    consecutive_failures += 1;
                    tracing::error!("refresh access token failed: {}", err);
                }
            }
        }
    }))
}

pub(super) fn parse_expires_in(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
}

pub(super) fn get_refresh_millis(token_ttl_secs: u64) -> u64 {
    let refresh_millis = token_ttl_secs.saturating_mul(1_000);
    if refresh_millis < DEFAULT_EXPIRY_DELTA_MILLIS {
        return refresh_millis;
    }

    let refresh_millis = refresh_millis - DEFAULT_EXPIRY_DELTA_MILLIS;
    if refresh_millis > RAND_TIME_UPPER_LIMIT_MILLIS {
        refresh_millis - jitter_millis(RAND_TIME_UPPER_LIMIT_MILLIS)
    } else {
        refresh_millis
    }
}

pub(super) fn jitter_millis(upper_bound: u64) -> u64 {
    if upper_bound == 0 {
        return 0;
    }

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| u64::from(duration.subsec_nanos()) % upper_bound)
        .unwrap_or_default()
}
