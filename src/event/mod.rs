//! Botgo-style event registration and dispatch helpers.

#![allow(non_snake_case, non_upper_case_globals)]

mod handlers;
mod payload;
mod register;
mod registry;
mod types;

pub use payload::{ParseData, PayloadData};
pub use register::{RegisterHandlers, RegisterableHandler};
pub use registry::{ParseAndHandle, RegisterHandler};
pub use types::*;

#[cfg(test)]
mod tests;
