use std::time::Duration;

use crate::error::err_session_limit;
use crate::gateway::Gateway;
use crate::models::api::GatewayResponse;

pub fn calc_interval(max_concurrency: u32) -> Duration {
    Gateway::session_start_interval(max_concurrency)
}

#[allow(non_snake_case)]
pub fn CalcInterval(max_concurrency: u32) -> Duration {
    calc_interval(max_concurrency)
}

pub fn check_session_limit(ap_info: &GatewayResponse) -> crate::Result<()> {
    if ap_info.shards > ap_info.session_start_limit.remaining {
        Err(err_session_limit().into())
    } else {
        Ok(())
    }
}

#[allow(non_snake_case)]
pub fn CheckSessionLimit(ap_info: &GatewayResponse) -> crate::Result<()> {
    check_session_limit(ap_info)
}
