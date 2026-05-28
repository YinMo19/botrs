use crate::error::session_limit_error;
use crate::models::api::GatewayResponse;

pub(crate) fn check_session_limit(ap_info: &GatewayResponse) -> crate::Result<()> {
    if ap_info.shards > ap_info.session_start_limit.remaining {
        Err(session_limit_error().into())
    } else {
        Ok(())
    }
}
