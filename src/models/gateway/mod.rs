//! Gateway event models for the QQ Guild Bot API.

mod auth;
mod constants;
mod events;
mod payload;

pub use auth::{
    Hello, Identify, IdentifyProperties, Ready, Resume, WSIdentityData, WSReadyData, WSResumeData,
    WSUser,
};
pub use constants::*;
pub use events::*;
pub use payload::{GatewayEvent, WSPayload, WSPayloadBase};

#[cfg(test)]
mod tests;
