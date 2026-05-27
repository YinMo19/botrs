//! Event registration and dispatch helpers.

mod handlers;
mod payload;
mod register;
mod registry;
mod types;

pub use payload::{PayloadData, parse_data};
pub use register::{RegisterableHandler, register_handlers};
pub use registry::{parse_and_handle, register_handler};
pub use types::*;

#[cfg(test)]
mod tests;
