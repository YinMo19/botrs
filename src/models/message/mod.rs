//! Message-related data models for the QQ Guild Bot API.
//!
//! This module contains message types for the QQ Bot Open API.
//!
//! # Migration Guide: New Message Parameter API
//!
//! Starting from version 0.2.0, this module introduces cleaner parameter structs for message sending
//! to replace functions with many `Option<T>` parameters.
//!
//! ## Benefits
//!
//! - **Cleaner code**: Use `..Default::default()` instead of many `None` parameters
//! - **Better readability**: Named fields instead of positional parameters
//! - **Type safety**: Structured parameters prevent parameter ordering mistakes
//! - **Extensibility**: Easy to add new fields without breaking existing code
//! - **Builder patterns**: Convenient methods for common operations
//!
//! ## Migration Examples
//!
//! ### Channel Messages
//!
//! **Old API (deprecated):**
//! ```rust,no_run
//! # use botrs::*;
//! # async fn example(api: &BotApi, token: &Token, channel_id: &str) -> Result<()> {
//! api.post_message(
//!     token,
//!     channel_id,
//!     Some("Hello!"),    // content
//!     None,              // embed
//!     None,              // ark
//!     None,              // message_reference
//!     None,              // image
//!     None,              // file_image
//!     None,              // msg_id
//!     None,              // event_id
//!     None,              // markdown
//!     None,              // keyboard
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! **New API:**
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token, channel_id: &str) -> Result<()> {
//! // Simple text message
//! let params = MessageParams::new_text("Hello!");
//! api.post_message_with_params(token, channel_id, params).await?;
//!
//! // Message with embed
//! // Message with embed
//! # let my_embed = Default::default();
//! let params = MessageParams {
//!     content: Some("Check this out!".to_string()),
//!     embed: Some(my_embed),
//!     ..Default::default()
//! };
//! api.post_message_with_params(token, channel_id, params).await?;
//!
//! // Reply to a message
//! # let message_id = "123456";
//! let params = MessageParams::new_text("Reply content").with_reply(message_id);
//! api.post_message_with_params(token, channel_id, params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Group Messages
//!
//! **Old API (deprecated):**
//! ```rust,no_run
//! # use botrs::*;
//! # async fn example(api: &BotApi, token: &Token, group_openid: &str) -> Result<()> {
//! api.post_group_message(
//!     token,
//!     group_openid,
//!     Some(0),           // msg_type
//!     Some("Hello!"),    // content
//!     None,              // embed
//!     None,              // ark
//!     None,              // message_reference
//!     None,              // media
//!     None,              // msg_id
//!     None,              // msg_seq
//!     None,              // event_id
//!     None,              // markdown
//!     None,              // keyboard
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! **New API:**
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::GroupMessageParams;
//! # async fn example(api: &BotApi, token: &Token, group_openid: &str) -> Result<()> {
//! let params = GroupMessageParams::new_text("Hello!");
//! api.post_group_message_with_params(token, group_openid, params).await?;
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
//! ## Breaking Changes
//!
//! - Old message sending functions are **deprecated** but still functional
//! - They will be removed in version 1.0.0
//! - No immediate breaking changes - old code compiles with warnings
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
