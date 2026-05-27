//! Event registration and dispatch helpers.

mod handlers;
mod payload;
mod registry;
mod types;

pub(crate) use payload::PayloadData;
#[cfg(test)]
use payload::parse_data;
pub(crate) use registry::parse_and_handle;
pub(crate) use types::EventParseFn;

#[cfg(test)]
mod tests;
