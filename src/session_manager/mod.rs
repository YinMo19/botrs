//! Botgo-style websocket session management helpers.

#![allow(non_snake_case, non_upper_case_globals)]

mod channel;
mod errors;
mod factory;
mod limit;
mod session;
mod traits;

pub use channel::ChanManager;
pub use errors::{
    CanNotIdentify, CanNotIdentifyErrSet, CanNotResume, CanNotResumeErrSet, can_not_identify,
    can_not_resume,
};
pub use factory::{
    NewSessionManager, SessionManagerFactory, SetSessionManager, new_session_manager,
    set_session_manager, set_session_manager_factory,
};
pub use limit::{CalcInterval, CheckSessionLimit, calc_interval, check_session_limit};
pub use session::Session;
pub use traits::{BoxedSessionManager, SessionConnectFn, SessionFuture, SessionManager};

#[cfg(test)]
mod tests;
