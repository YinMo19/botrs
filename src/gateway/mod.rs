//! WebSocket gateway implementation for the QQ Guild Bot API.
//!
//! This module provides the WebSocket client for connecting to the QQ Guild Bot API gateway,
//! handling authentication, heartbeats, and event dispatching.

mod accessors;
mod close;
mod debug;
mod heartbeat;
mod runtime;
mod types;

pub(crate) use types::Gateway;

#[cfg(test)]
mod tests;
