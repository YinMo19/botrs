//! API response models for the QQ Guild Bot API.

mod audio;
mod error;
mod gateway;
mod message;
mod pagination;
mod rate_limit;

pub use audio::AudioAction;
pub use error::ApiError;
pub use gateway::{BotInfo, GatewayResponse, SessionStartLimit, ShardConfig};
pub use message::{MessageResponse, PinsMessage};
pub use pagination::{PaginatedResponse, Pagination};
pub use rate_limit::RateLimit;

#[cfg(test)]
mod tests;
