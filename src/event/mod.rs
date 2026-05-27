//! Event registration and dispatch helpers.

mod handlers;
mod payload;
mod registry;

pub(crate) use registry::parse_and_handle;

#[cfg(test)]
mod tests;
