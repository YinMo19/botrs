use super::Gateway;
use crate::error::{
    CodeConnCloseCantIdentify, CodeConnCloseCantResume, CodeInvalidSession, Result,
    WSCodeBackendAuthenticationFail, WSCodeBackendBotBanned, WSCodeBackendBotOffline,
    WSCodeBackendInvalidSeq, WSCodeBackendSessionNoLongerValid, invalid_session_error, sdk_error,
};
use std::sync::atomic::Ordering;
use tracing::{debug, info};

impl Gateway {
    /// Handles close codes and determines reconnection behavior
    pub(super) fn handle_close_code(&mut self, close_code: u16) -> Result<()> {
        if close_code == WSCodeBackendAuthenticationFail {
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
                CodeConnCloseCantIdentify,
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
        const CODE_INVALID_SESSION: u16 = CodeInvalidSession as u16;
        const CODE_CONN_CLOSE_CANT_RESUME: u16 = CodeConnCloseCantResume as u16;
        const CODE_SESSION_NO_LONGER_VALID: u16 = WSCodeBackendSessionNoLongerValid;
        const CODE_INVALID_SEQ: u16 = WSCodeBackendInvalidSeq;

        matches!(
            close_code,
            CODE_INVALID_SESSION
                | CODE_CONN_CLOSE_CANT_RESUME
                | CODE_SESSION_NO_LONGER_VALID
                | CODE_INVALID_SEQ
        )
    }

    pub(super) fn cannot_identify_close_code(close_code: u16) -> bool {
        const CODE_BOT_OFFLINE: u16 = WSCodeBackendBotOffline;
        const CODE_BOT_BANNED: u16 = WSCodeBackendBotBanned;

        matches!(close_code, CODE_BOT_OFFLINE | CODE_BOT_BANNED)
    }
}
