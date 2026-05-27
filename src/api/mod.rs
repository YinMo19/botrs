//! Bot API implementation for the QQ Guild Bot API.
//!
//! This module provides the main API client for interacting with the QQ Guild Bot API,
//! implementing all endpoints available in the Python SDK.
//!
//! # Message Sending API Refactoring (v0.2.0)
//!
//! ## 🚀 **Major Improvement: Parameter Struct API**
//!
//! We've completely refactored the message sending API to eliminate the problem of
//! functions with many `None` parameters. The new API uses structured parameters
//! with `..Default::default()` for a much cleaner developer experience.
//!
//! ### **Problem Solved**
//!
//! **Before (Multiple None Parameters):**
//! ```rust,no_run
//! # use botrs::*;
//! # async fn example(api: &BotApi, token: &Token) -> Result<()> {
//! api.post_message(
//!     token, "channel_id", Some("Hello!"),
//!     None, None, None, None, None, None, None, None, None  // 😱 Too many Nones!
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! **After (Clean Parameter Structs):**
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token) -> Result<()> {
//! let params = MessageParams::new_text("Hello!");
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## **New API Functions (Recommended)**
//!
//! - [`BotApi::post_message_with_params`] - Send channel messages with [`MessageParams`]
//! - [`BotApi::post_group_message_with_params`] - Send group messages with [`GroupMessageParams`]
//! - [`BotApi::post_c2c_message_with_params`] - Send C2C messages with [`C2CMessageParams`]
//! - [`BotApi::post_dms_with_params`] - Send direct messages with [`DirectMessageParams`]
//!
//! ## **Legacy API Functions (Deprecated)**
//!
//! - [`BotApi::post_message`] ⚠️ Use `post_message_with_params` instead
//! - [`BotApi::post_group_message`] ⚠️ Use `post_group_message_with_params` instead
//! - [`BotApi::post_c2c_message`] ⚠️ Use `post_c2c_message_with_params` instead
//! - [`BotApi::post_dms`] ⚠️ Use `post_dms_with_params` instead
//!
//! ## **Key Benefits**
//!
//!  - **Cleaner Code**: Use `..Default::default()` instead of many `None` parameters
//!  - **Better Readability**: Named fields instead of positional parameters
//!  - **Type Safety**: Structured parameters prevent parameter ordering mistakes
//!  - **Builder Patterns**: Convenient methods like `.with_reply()` and `.with_file_image()`
//!  - **Extensibility**: Easy to add new fields without breaking existing code
//!  - **Compatibility**: Based on the official QQ Bot Open API message structure
//!
//! ## **Migration Examples**
//!
//! ### Simple Text Message
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token) -> Result<()> {
//! let params = MessageParams::new_text("Hello World!");
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Message with Embed
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::{MessageParams, Embed};
//! # async fn example(api: &BotApi, token: &Token, embed: Embed) -> Result<()> {
//! let params = MessageParams {
//!     content: Some("Check this out!".to_string()),
//!     embed: Some(embed),
//!     ..Default::default()
//! };
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Reply with File
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi, token: &Token, file_data: &[u8]) -> Result<()> {
//! let params = MessageParams::new_text("Here's your file!")
//!     .with_file_image(file_data)
//!     .with_reply("message_id_to_reply_to");
//! api.post_message_with_params(token, "channel_id", params).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## **Backward Compatibility**
//!
//! All legacy functions still work but are marked as deprecated. They will be
//! removed in version 1.0.0. Legacy functions internally call the new API
//! to ensure identical behavior.
//!
//! See [`crate::models::message`] for complete migration guide and API documentation.
//!
//! [`MessageParams`]: crate::models::message::MessageParams
//! [`GroupMessageParams`]: crate::models::message::GroupMessageParams
//! [`C2CMessageParams`]: crate::models::message::C2CMessageParams
//! [`DirectMessageParams`]: crate::models::message::DirectMessageParams

use crate::http::HttpClient;
use crate::token::Token;

pub type APIVersion = u32;
#[allow(non_upper_case_globals)]
pub const APIv1: APIVersion = 1;
/// Default idle connection count for the OpenAPI v1 client.
#[allow(non_upper_case_globals)]
pub const MaxIdleConns: usize = 3000;
/// HTTP header carrying the bot app id for interaction callbacks.
#[allow(non_upper_case_globals)]
pub const HeaderCallbackAppID: &str = "X-Callback-AppID";

#[allow(non_snake_case)]
pub fn APIVersionString(version: APIVersion) -> String {
    format!("v{version}")
}

/// Bot API client for the QQ Guild Bot API.
#[derive(Clone)]
pub struct BotApi {
    /// The HTTP client used for making requests
    http: HttpClient,
    /// Bot application ID stored on the OpenAPI instance.
    app_id: String,
    /// Optional token stored for OpenAPI calls.
    token: Option<Token>,
}

mod announces;
mod api_permissions;
mod audio;
mod base;
mod channel;
mod channel_permissions;
mod compat;
mod direct_message;
mod forum;
mod gateway;
mod guild;
mod interaction;
mod me;
mod member;
mod message;
mod message_reaction;
mod message_setting;
mod pins;
mod resource;
mod role;
mod schedule;
mod webhook;

impl std::fmt::Debug for BotApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BotApi").field("http", &self.http).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::HttpClient;
    use crate::options::Options;
    use std::time::Duration;

    use super::{APIVersionString, APIv1, BotApi};

    #[test]
    fn test_api_creation() {
        let http = HttpClient::new(30, false).unwrap();
        let api = BotApi::new(http);
        assert!(!api.http().is_sandbox());
    }

    #[test]
    fn test_base_helpers() {
        let (api, token) = BotApi::Setup("app-id", "secret", true).unwrap();
        assert_eq!(api.Version(), APIv1);
        assert_eq!(APIVersionString(api.version()), "v1");
        assert_eq!(token.app_id(), "app-id");
        assert_eq!(api.GetAppID(), "app-id");
        assert_eq!(api.http().union_app_id(), Some("app-id"));
        assert!(api.http().is_sandbox());

        let api = api.WithTimeout(Duration::from_secs(7)).unwrap();
        assert_eq!(api.http().timeout(), Duration::from_secs(7));
        assert_eq!(api.GetAppID(), "app-id");

        let api = api.SetDebug(true);
        assert!(api.http().debug_enabled());
        assert_eq!(api.GetAppID(), "app-id");
        assert_eq!(api.TraceID(), "");
    }

    #[test]
    fn options_build_custom_urls() {
        let api = BotApi::new(HttpClient::new(30, false).unwrap());
        let options = Options::from_options([crate::WithURL("https://example.com/custom")]);
        assert_eq!(
            api.url_with_options("/channels/1/messages", &options),
            "https://example.com/custom"
        );

        let options = Options::default();
        assert_eq!(
            api.url_with_options("/channels/1/messages", &options),
            format!("{}{}", crate::DEFAULT_API_URL, "/channels/1/messages")
        );
    }

    #[test]
    fn hide_tip_option_sets_flag() {
        let options = Options::from_options([crate::WithHideTip()]);
        assert!(options.hide_tip);
        assert!(options.url.is_none());
    }

    #[test]
    fn hide_tip_query_omits_false() {
        assert!(BotApi::hide_tip_query(false).is_none());
        assert_eq!(
            BotApi::hide_tip_query(true)
                .unwrap()
                .get("hidetip")
                .map(String::as_str),
            Some("true")
        );
    }

    #[test]
    fn message_response_accepts_legacy_wrapper() {
        let message = BotApi::parse_message_response(serde_json::json!({
            "message": {
                "id": "msg-1",
                "content": "wrapped",
                "channel_id": "channel-1"
            }
        }))
        .unwrap();

        assert_eq!(message.id.as_deref(), Some("msg-1"));
        assert_eq!(message.content.as_deref(), Some("wrapped"));
        assert_eq!(message.channel_id.as_deref(), Some("channel-1"));
    }

    #[test]
    fn message_response_keeps_direct_shape() {
        let message = BotApi::parse_message_response(serde_json::json!({
            "id": "msg-2",
            "content": "direct"
        }))
        .unwrap();

        assert_eq!(message.id.as_deref(), Some("msg-2"));
        assert_eq!(message.content.as_deref(), Some("direct"));
    }
}
