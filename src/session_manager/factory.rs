use std::sync::{LazyLock, RwLock};

use super::{ChanManager, SessionManager};

type ManagerBox = Box<dyn SessionManager>;
type ManagerFactory = dyn Fn() -> ManagerBox + Send + Sync;

static DEFAULT_SESSION_MANAGER: LazyLock<RwLock<Box<ManagerFactory>>> =
    LazyLock::new(|| RwLock::new(Box::new(|| Box::new(ChanManager::new()))));

pub(crate) fn new_session_manager() -> ManagerBox {
    let factory = DEFAULT_SESSION_MANAGER
        .read()
        .expect("default session manager lock poisoned");
    factory()
}
