use super::Gateway;
use crate::error::{
    CODE_CONN_CLOSE_CANT_IDENTIFY, CODE_CONN_CLOSE_CANT_RESUME, CODE_INVALID_SESSION, Result,
    WS_CODE_BACKEND_AUTHENTICATION_FAIL, WS_CODE_BACKEND_BOT_BANNED, WS_CODE_BACKEND_BOT_OFFLINE,
    WS_CODE_BACKEND_INVALID_SEQ, WS_CODE_BACKEND_SESSION_NO_LONGER_VALID, invalid_session_error,
    sdk_error,
};
use std::sync::atomic::Ordering;
use tracing::{debug, info};

impl Gateway {
    /// Handles close codes and determines reconnection behavior
    pub(super) fn handle_close_code(&mut self, close_code: u16) -> Result<()> {
        if close_code == WS_CODE_BACKEND_AUTHENTICATION_FAIL {
            info!("[botrs] 鉴权失败，重置token...");
            self.session_id = None;
            self.last_seq.store(0, Ordering::Relaxed);
            self.is_ready.store(false, Ordering::Relaxed);
        }

        if Self::cannot_resume_close_code(close_code) {
            debug!("[botrs] 无法重连，创建新连接!");
            self.session_id = None;
            self.last_seq.store(0, Ordering::Relaxed);
            self.is_ready.store(false, Ordering::Relaxed);
            self.can_reconnect.store(true, Ordering::Relaxed);
            Err(invalid_session_error().into())
        } else if Self::cannot_identify_close_code(close_code) {
            info!("[botrs] 连接关闭且不能重新鉴权，停止连接尝试");
            self.session_id = None;
            self.last_seq.store(0, Ordering::Relaxed);
            self.is_ready.store(false, Ordering::Relaxed);
            self.can_reconnect.store(false, Ordering::Relaxed);
            Err(sdk_error(
                CODE_CONN_CLOSE_CANT_IDENTIFY,
                format!("websocket closed with code {close_code}"),
            )
            .into())
        } else if !self.can_reconnect.load(Ordering::Relaxed) {
            debug!("[botrs] 当前状态不允许重连，停止连接尝试");
            Ok(())
        } else {
            debug!("[botrs] 连接断开，准备重连...");
            self.is_ready.store(false, Ordering::Relaxed);
            self.can_reconnect.store(true, Ordering::Relaxed);
            Ok(())
        }
    }

    pub(super) fn cannot_resume_close_code(close_code: u16) -> bool {
        const INVALID_SESSION: u16 = CODE_INVALID_SESSION as u16;
        const CONN_CLOSE_CANT_RESUME: u16 = CODE_CONN_CLOSE_CANT_RESUME as u16;
        const SESSION_NO_LONGER_VALID: u16 = WS_CODE_BACKEND_SESSION_NO_LONGER_VALID;
        const INVALID_SEQ: u16 = WS_CODE_BACKEND_INVALID_SEQ;

        matches!(
            close_code,
            INVALID_SESSION | CONN_CLOSE_CANT_RESUME | SESSION_NO_LONGER_VALID | INVALID_SEQ
        )
    }

    pub(super) fn cannot_identify_close_code(close_code: u16) -> bool {
        const BOT_OFFLINE: u16 = WS_CODE_BACKEND_BOT_OFFLINE;
        const BOT_BANNED: u16 = WS_CODE_BACKEND_BOT_BANNED;

        matches!(close_code, BOT_OFFLINE | BOT_BANNED)
    }
}
