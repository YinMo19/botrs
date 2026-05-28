//! Websocket session management helpers.

mod channel;
mod errors;
mod factory;
mod limit;
mod session;
mod traits;

pub(crate) use channel::ChanManager;
pub(crate) use errors::{can_not_identify, can_not_resume};
pub(crate) use factory::new_session_manager;
pub(crate) use limit::check_session_limit;
pub(crate) use session::Session;
pub(crate) use traits::SessionManager;

#[cfg(test)]
mod tests;
