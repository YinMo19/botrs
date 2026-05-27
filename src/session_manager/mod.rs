//! Websocket session management helpers.

mod channel;
mod errors;
mod factory;
mod limit;
mod session;
mod traits;

pub use channel::ChanManager;
pub use errors::{can_not_identify, can_not_resume};
pub use factory::{new_session_manager, set_session_manager_factory};
pub use limit::check_session_limit;
pub use session::Session;
pub use traits::SessionManager;

#[cfg(test)]
mod tests;
