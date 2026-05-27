//! Event registration and dispatch helpers.

mod handlers;
mod payload;
mod registry;

pub(crate) use payload::PayloadData;
#[cfg(test)]
use payload::parse_data;
pub(crate) use registry::parse_and_handle;

#[cfg(test)]
mod tests;
