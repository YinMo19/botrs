//! Event registration and dispatch helpers.

mod handlers;
mod payload;
#[cfg(test)]
mod register;
mod registry;
mod types;

pub(crate) use payload::PayloadData;
#[cfg(test)]
use payload::parse_data;
#[cfg(test)]
use register::register_handlers;
pub(crate) use registry::parse_and_handle;
#[cfg(test)]
pub(crate) use registry::register_handler;
pub(crate) use types::*;

#[cfg(test)]
mod tests;
