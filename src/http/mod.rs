//! HTTP client implementation for the QQ Guild Bot API.
//!
//! This module provides the HTTP client for making requests to the QQ Bot API,
//! handling authentication, rate limiting, and error responses.

mod client;
mod request;
mod response;

pub use client::HttpClient;

#[cfg(test)]
mod tests;
