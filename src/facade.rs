//! Top-level facade providing Go-style entry points for the QQ Bot Open API.
//!
//! These functions mirror the public surface of the official Go SDK so that
//! consumers used to that style can continue using the same names. The
//! framework itself is independent — these are simply convenience wrappers
//! around [`crate::api::BotApi`] and friends.

#![allow(non_snake_case, non_upper_case_globals)]

use std::collections::HashMap;
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

pub static VersionMapping: LazyLock<RwLock<HashMap<APIVersion, BotApi>>> =
    LazyLock::new(|| RwLock::new(HashMap::from([(APIv1, default_openapi_template())])));

fn default_openapi_template() -> BotApi {
    BotApi::new(HttpClient::new(crate::DEFAULT_TIMEOUT, false).expect("valid default api client"))
}

pub fn DefaultImpl() -> BotApi {
    let version = *DefaultOpenAPIVersion
        .read()
        .expect("openapi version lock poisoned");
    VersionMapping
        .read()
        .expect("openapi version mapping lock poisoned")
        .get(&version)
        .cloned()
        .unwrap_or_else(default_openapi_template)
}

pub fn SelectOpenAPIVersion(version: APIVersion) -> Result<()> {
    if VersionMapping
        .read()
        .expect("openapi version mapping lock poisoned")
        .contains_key(&version)
    {
        *DefaultOpenAPIVersion
            .write()
            .expect("openapi version lock poisoned") = version;
        Ok(())
    } else {
        Err(err_not_found_openapi().into())
    }
}

pub fn NewOpenAPI(app_id: impl Into<String>, token: Token) -> BotApi {
    new_openapi(app_id, token, false)
}

pub fn NewSandboxOpenAPI(app_id: impl Into<String>, token: Token) -> BotApi {
    new_openapi(app_id, token, true)
}

fn new_openapi(app_id: impl Into<String>, token: Token, in_sandbox: bool) -> BotApi {
    let version = *DefaultOpenAPIVersion
        .read()
        .expect("openapi version lock poisoned");
    let template = VersionMapping
        .read()
        .expect("openapi version mapping lock poisoned")
        .get(&version)
        .cloned()
        .unwrap_or_else(default_openapi_template);
    template
        .setup_from_template(app_id, token, in_sandbox)
        .expect("valid registered api template")
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

pub fn SetOpenAPIClient(version: APIVersion, client: BotApi) {
    VersionMapping
        .write()
        .expect("openapi version mapping lock poisoned")
        .insert(version, client);
}

pub fn RegisterDispatchEventHandler(event_type: impl Into<EventType>, handler: EventParseFunc) {
    crate::event::RegisterHandler(WSDispatchEvent as OpCode, event_type, handler);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{LazyLock, Mutex};

    static OPENAPI_REGISTRY_TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    #[test]
    fn select_openapi_version_accepts_v1_only() {
        let _guard = OPENAPI_REGISTRY_TEST_LOCK.lock().unwrap();
        assert!(SelectOpenAPIVersion(APIv1).is_ok());
        assert!(SelectOpenAPIVersion(99).is_err());
    }

    #[test]
    fn new_openapi_uses_requested_environment() {
        let _guard = OPENAPI_REGISTRY_TEST_LOCK.lock().unwrap();
        SelectOpenAPIVersion(APIv1).unwrap();

        let token = Token::new("app", "secret");
        let api = NewSandboxOpenAPI("app", token);
        assert_eq!(api.version(), APIv1);
        assert_eq!(api.get_app_id(), "app");
        assert!(api.token().is_some());
    }

    #[test]
    fn openapi_client_registry_controls_version_selection() {
        let _guard = OPENAPI_REGISTRY_TEST_LOCK.lock().unwrap();
        let custom_version = 42;
        let template = BotApi::new(HttpClient::new(9, false).unwrap()).set_debug(true);
        SetOpenAPIClient(custom_version, template);
        SelectOpenAPIVersion(custom_version).unwrap();

        let api = NewSandboxOpenAPI("app", Token::new("app", "secret"));
        assert_eq!(api.http().timeout(), std::time::Duration::from_secs(9));
        assert!(api.http().is_sandbox());
        assert!(api.http().debug_enabled());

        SelectOpenAPIVersion(APIv1).unwrap();
    }
}
