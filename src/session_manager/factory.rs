use std::sync::{LazyLock, RwLock};

use super::{BoxedSessionManager, ChanManager, SessionManager};

pub type SessionManagerFactory = dyn Fn() -> BoxedSessionManager + Send + Sync;

static DEFAULT_SESSION_MANAGER: LazyLock<RwLock<Box<SessionManagerFactory>>> =
    LazyLock::new(|| RwLock::new(Box::new(|| Box::new(ChanManager::new()))));

pub fn new_session_manager() -> BoxedSessionManager {
    let factory = DEFAULT_SESSION_MANAGER
        .read()
        .expect("default session manager lock poisoned");
    factory()
}

pub fn set_session_manager_factory(
    factory: impl Fn() -> BoxedSessionManager + Send + Sync + 'static,
) {
    *DEFAULT_SESSION_MANAGER
        .write()
        .expect("default session manager lock poisoned") = Box::new(factory);
}

pub fn set_session_manager(manager: impl SessionManager + Clone + 'static) {
    set_session_manager_factory(move || Box::new(manager.clone()));
}
