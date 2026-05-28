//! Bot API implementation for the QQ Guild Bot API.
//!
//! This module provides the main typed REST client. Message sending uses explicit
//! parameter structs instead of positional `Option` lists:
//!
//! - [`BotApi::send_message`] with [`MessageParams`] for guild channels.
//! - [`BotApi::send_group_message`] with [`GroupMessageParams`] for groups.
//! - [`BotApi::send_c2c_message`] with [`C2CMessageParams`] for C2C chats.
//! - [`BotApi::send_direct_message`] with [`DirectMessageParams`] for DMs.
//!
//! ```rust,no_run
//! # use botrs::*;
//! # use botrs::models::message::MessageParams;
//! # async fn example(api: &BotApi) -> Result<()> {
//! let params = MessageParams::new_text("Hello!");
//! api.send_message("channel_id", params).await?;
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
use crate::token_impl::Token;

/// Bot API client for the QQ Guild Bot API.
#[derive(Clone)]
pub struct BotApi {
    /// The HTTP client used for making requests
    http: HttpClient,
    /// Token used for OpenAPI calls.
    token: Token,
}

mod announces;
mod api_permissions;
mod audio;
mod base;
mod channel;
mod channel_permissions;
mod direct_message;
mod gateway;
mod guild;
mod interaction;
mod me;
mod message;
mod message_reaction;
mod mute;
mod pins;
mod resource;
mod role;
mod schedule;
pub mod session;

impl std::fmt::Debug for BotApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BotApi").field("http", &self.http).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::HttpClient;

    use super::BotApi;

    #[test]
    fn test_api_creation() {
        let token = crate::Token::new("app-id", "secret");
        let http = HttpClient::new(30, false).unwrap();
        BotApi::new(http, token);
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
}
