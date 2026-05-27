//! Bot API implementation for the QQ Guild Bot API.
//!
//! This module provides the main typed REST client. Message sending uses explicit
//! parameter structs instead of positional `Option` lists:
//!
//! - [`BotApi::post_message_with_params`] with [`MessageParams`] for guild channels.
//! - [`BotApi::post_group_message_with_params`] with [`GroupMessageParams`] for groups.
//! - [`BotApi::post_c2c_message_with_params`] with [`C2CMessageParams`] for C2C chats.
//! - [`BotApi::post_dms_with_params`] with [`DirectMessageParams`] for DMs.
//!
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
//! See [`crate::models::message`] for the message parameter types and builders.
//!
//! [`MessageParams`]: crate::models::message::MessageParams
//! [`GroupMessageParams`]: crate::models::message::GroupMessageParams
//! [`C2CMessageParams`]: crate::models::message::C2CMessageParams
//! [`DirectMessageParams`]: crate::models::message::DirectMessageParams

use crate::http::HttpClient;
use crate::token::Token;

pub type APIVersion = u32;
pub const API_V1: APIVersion = 1;
/// Default idle connection count for the OpenAPI v1 client.
pub const DEFAULT_MAX_IDLE_CONNS: usize = 3000;
/// HTTP header carrying the bot app id for interaction callbacks.
pub const HEADER_CALLBACK_APP_ID: &str = "X-Callback-AppID";

pub fn api_version_string(version: APIVersion) -> String {
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
    use std::time::Duration;

    use super::{API_V1, BotApi, api_version_string};

    #[test]
    fn test_api_creation() {
        let http = HttpClient::new(30, false).unwrap();
        let api = BotApi::new(http);
        assert!(!api.http().is_sandbox());
    }

    #[test]
    fn test_base_helpers() {
        let (api, token) = BotApi::setup("app-id", "secret", true).unwrap();
        assert_eq!(api.version(), API_V1);
        assert_eq!(api_version_string(api.version()), "v1");
        assert_eq!(token.app_id(), "app-id");
        assert_eq!(api.get_app_id(), "app-id");
        assert_eq!(api.http().union_app_id(), Some("app-id"));
        assert!(api.http().is_sandbox());

        let api = api.with_timeout(Duration::from_secs(7)).unwrap();
        assert_eq!(api.http().timeout(), Duration::from_secs(7));
        assert_eq!(api.get_app_id(), "app-id");

        let api = api.set_debug(true);
        assert!(api.http().debug_enabled());
        assert_eq!(api.get_app_id(), "app-id");
        assert_eq!(api.trace_id(), "");
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
