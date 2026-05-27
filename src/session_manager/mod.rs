//! Websocket session management helpers.

mod channel;
mod errors;
mod factory;
mod limit;
mod session;
mod traits;

pub use channel::ChanManager;
pub use errors::{
    CANNOT_IDENTIFY_ERROR_CODES, CANNOT_RESUME_ERROR_CODES, can_not_identify, can_not_resume,
};
pub use factory::{
    SessionManagerFactory, new_session_manager, set_session_manager, set_session_manager_factory,
};
pub use limit::{calc_interval, check_session_limit};
pub use session::Session;
pub use traits::{BoxedSessionManager, SessionConnectFn, SessionFuture, SessionManager};

#[cfg(test)]
mod tests;
