use std::time::Duration;

use crate::error::session_limit_error;
use crate::gateway::Gateway;
use crate::models::api::GatewayResponse;

pub fn calc_interval(max_concurrency: u32) -> Duration {
    Gateway::session_start_interval(max_concurrency)
}

pub fn check_session_limit(ap_info: &GatewayResponse) -> crate::Result<()> {
    if ap_info.shards > ap_info.session_start_limit.remaining {
        Err(session_limit_error().into())
    } else {
        Ok(())
    }
}
