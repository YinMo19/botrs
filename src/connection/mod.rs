//! Connection state management for QQ Bot
//!
//! This module provides connection session management, state handling, and event parsing
//! for the websocket connections to QQ's gateway.

mod parsers;
mod session;
mod state;

pub use session::{ConnectionSession, Session, WsSink, WsStream};
pub use state::ConnectionState;
