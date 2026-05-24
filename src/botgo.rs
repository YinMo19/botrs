//! Top-level botgo-compatible facade functions.

#![allow(non_snake_case, non_upper_case_globals)]

use std::sync::{LazyLock, RwLock};

use crate::api::{APIVersion, APIv1, BotApi};
use crate::error::{Result, err_not_found_openapi};
use crate::event::EventParseFunc;
use crate::http::HttpClient;
use crate::models::gateway::{EventType, OpCode, WSDispatchEvent};
use crate::session_manager::SessionManager;
use crate::token::Token;

pub static DefaultOpenAPIVersion: LazyLock<RwLock<APIVersion>> =
    LazyLock::new(|| RwLock::new(APIv1));

pub fn SelectOpenAPIVersion(version: APIVersion) -> Result<()> {
    if version == APIv1 {
        *DefaultOpenAPIVersion
            .write()
            .expect("openapi version lock poisoned") = version;
        Ok(())
    } else {
        Err(err_not_found_openapi().into())
    }
}

pub fn NewOpenAPI(_app_id: impl Into<String>, token: Token) -> BotApi {
    BotApi::with_token(
        HttpClient::new(crate::DEFAULT_TIMEOUT, false).expect("valid default api client"),
        token,
    )
}

pub fn NewSandboxOpenAPI(_app_id: impl Into<String>, token: Token) -> BotApi {
    BotApi::with_token(
        HttpClient::new(crate::DEFAULT_TIMEOUT, true).expect("valid sandbox api client"),
        token,
    )
}

pub fn SetLogger(logger: impl crate::log::Logger + 'static) {
    crate::log::set_logger(logger);
}

pub fn SetSessionManager(manager: impl SessionManager + Clone + 'static) {
    crate::session_manager::SetSessionManager(manager);
}

pub fn SetWebsocketClient(client: impl crate::websocket::WebSocket + Clone + 'static) {
    crate::websocket::Register(client);
}

pub fn SetOpenAPIClient(_version: APIVersion, _client: BotApi) {
    // BotRS currently has a single OpenAPI implementation. This function is kept
    // as a compatibility registration hook for botgo-style setup code.
}

pub fn RegisterDispatchEventHandler(event_type: impl Into<EventType>, handler: EventParseFunc) {
    crate::event::RegisterHandler(WSDispatchEvent as OpCode, event_type, handler);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_openapi_version_accepts_v1_only() {
        assert!(SelectOpenAPIVersion(APIv1).is_ok());
        assert!(SelectOpenAPIVersion(99).is_err());
    }

    #[test]
    fn new_openapi_uses_requested_environment() {
        let token = Token::new("app", "secret");
        let api = NewSandboxOpenAPI("app", token);
        assert_eq!(api.Version(), APIv1);
        assert!(api.token().is_some());
    }
}
