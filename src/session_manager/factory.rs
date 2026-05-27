use std::sync::{LazyLock, RwLock};

use super::{ChanManager, SessionManager};

type ManagerBox = Box<dyn SessionManager>;
type ManagerFactory = dyn Fn() -> ManagerBox + Send + Sync;

static DEFAULT_SESSION_MANAGER: LazyLock<RwLock<Box<ManagerFactory>>> =
    LazyLock::new(|| RwLock::new(Box::new(|| Box::new(ChanManager::new()))));

pub fn new_session_manager() -> ManagerBox {
    let factory = DEFAULT_SESSION_MANAGER
        .read()
        .expect("default session manager lock poisoned");
    factory()
}

pub fn set_session_manager_factory(factory: impl Fn() -> ManagerBox + Send + Sync + 'static) {
    *DEFAULT_SESSION_MANAGER
        .write()
        .expect("default session manager lock poisoned") = Box::new(factory);
}
