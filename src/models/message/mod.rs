//! Message-related data models for the QQ Guild Bot API.
//!
//! This module contains message types for the QQ Bot Open API.
//!
//! # Message Parameter API
//!
//! Message sending uses parameter structs instead of functions with many
//! `Option<T>` parameters.
//!
//! ## Benefits
//!
//! - **Cleaner code**: Use `..Default::default()` instead of many `None` parameters
//! - **Better readability**: Named fields instead of positional parameters
//! - **Type safety**: Structured parameters prevent parameter ordering mistakes
//! - **Extensibility**: Easy to add new fields without breaking existing code
//! - **Builder patterns**: Convenient methods for common operations
//!
//! ### Channel Messages
//!
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, channel_id: &str) -> Result<()> {
//! // Simple text message
//! let params = MessageParams::new_text("Hello!");
//! api.send_message(channel_id, params).await?;
//!
//! // Message with embed
//! # let my_embed = Default::default();
//! let params = MessageParams {
//!     content: Some("Check this out!".to_string()),
//!     embed: Some(my_embed),
//!     ..Default::default()
//! };
//! api.send_message(channel_id, params).await?;
//!
//! // Reply to a message
//! # let message_id = "123456";
//! let params = MessageParams::new_text("Reply content").with_reply(message_id);
//! api.send_message(channel_id, params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Group Messages
//!
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::GroupMessageParams;
//! # async fn example(api: &BotApi, group_openid: &str) -> Result<()> {
//! let params = GroupMessageParams::new_text("Hello!");
//! api.send_group_message(group_openid, params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Parameter Structs
//!
//! - [`MessageParams`] - For channel messages
//! - [`GroupMessageParams`] - For group messages
//! - [`C2CMessageParams`] - For C2C (client-to-client) messages
//! - [`DirectMessageParams`] - For direct messages
//!
//! Each struct provides:
//! - `new_text(content)` - Create simple text message
//! - `with_reply(message_id)` - Add reply reference
//! - `with_file_image(&bytes)` - Add file attachment (MessageParams/DirectMessageParams only)
//! - `Default` implementation for easy struct building
//!
//! See the examples in `/examples` directory for comprehensive usage patterns.

mod content;
mod event;
mod mention;
mod pager;
mod params;
mod payload;

pub use content::*;
pub use event::*;
pub use mention::*;
pub use pager::*;
pub use params::*;
pub use payload::*;

#[cfg(test)]
mod tests;
