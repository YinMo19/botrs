//! Gateway event models for the QQ Guild Bot API.

mod auth;
mod events;
mod payload;

pub use auth::{
    Hello, Identify, IdentifyProperties, Ready, Resume, WSIdentityData, WSReadyData, WSResumeData,
    WSUser,
};
pub use events::*;
pub use payload::GatewayEvent;

#[cfg(test)]
mod tests;
