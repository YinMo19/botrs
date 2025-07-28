//! Authentication token management for QQ Guild Bot API.
//!
//! This module provides the `Token` struct for managing bot authentication
//! credentials including app ID and secret.

use crate::error::{BotError, Result};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the authentication token for a QQ Guild Bot.
///
/// The token contains the app ID and secret required for authenticating
/// with the QQ Guild Bot API. It can generate the appropriate authorization
/// headers for API requests.
///
/// # Examples
///
/// ```rust
/// use botrs::Token;
///
/// let token = Token::new("your_app_id", "your_secret");
/// let auth_header = token.authorization_header();
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct Token {
    /// The application ID provided by QQ
    app_id: String,
    /// The application secret provided by QQ
    secret: String,
}

impl Token {
    /// Creates a new token with the given app ID and secret.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The application ID from QQ Guild Bot settings
    /// * `secret` - The application secret from QQ Guild Bot settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use botrs::Token;
    ///
    /// let token = Token::new("123456789", "your_secret_here");
    /// ```
    pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            secret: secret.into(),
        }
    }

    /// Gets the app ID.
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Gets the secret.
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// Generates the authorization header value for API requests.
    ///
    /// The authorization header uses the format "QQBot {base64_encoded_credentials}"
    /// where the credentials are "app_id.secret" encoded in base64.
    ///
    /// # Returns
    ///
    /// A string containing the authorization header value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use botrs::Token;
    ///
    /// let token = Token::new("123", "secret");
    /// let auth_header = token.authorization_header();
    /// assert!(auth_header.starts_with("QQBot "));
    /// ```
    pub fn authorization_header(&self) -> String {
        let credentials = format!("{}.{}", self.app_id, self.secret);
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        format!("QQBot {}", encoded)
    }

    /// Generates the bot token for WebSocket authentication.
    ///
    /// The bot token uses the format "QQBot {base64_encoded_credentials}"
    /// which is the same as the authorization header.
    ///
    /// # Returns
    ///
    /// A string containing the bot token.
    pub fn bot_token(&self) -> String {
        self.authorization_header()
    }

    /// Validates that the token has non-empty app ID and secret.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the token is valid, otherwise returns a `BotError::Auth`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use botrs::Token;
    ///
    /// let token = Token::new("123", "secret");
    /// assert!(token.validate().is_ok());
    ///
    /// let invalid_token = Token::new("", "secret");
    /// assert!(invalid_token.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.app_id.is_empty() {
            return Err(BotError::auth("App ID cannot be empty"));
        }
        if self.secret.is_empty() {
            return Err(BotError::auth("Secret cannot be empty"));
        }
        Ok(())
    }

    /// Creates a token from environment variables.
    ///
    /// Looks for `QQ_BOT_APP_ID` and `QQ_BOT_SECRET` environment variables.
    ///
    /// # Returns
    ///
    /// A `Result` containing the token if both environment variables are found,
    /// otherwise returns a `BotError::Config`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::Token;
    ///
    /// // Assuming environment variables are set:
    /// // QQ_BOT_APP_ID=123456789
    /// // QQ_BOT_SECRET=your_secret
    /// let token = Token::from_env().unwrap();
    /// ```
    pub fn from_env() -> Result<Self> {
        let app_id = std::env::var("QQ_BOT_APP_ID")
            .map_err(|_| BotError::config("QQ_BOT_APP_ID environment variable not found"))?;
        let secret = std::env::var("QQ_BOT_SECRET")
            .map_err(|_| BotError::config("QQ_BOT_SECRET environment variable not found"))?;

        let token = Self::new(app_id, secret);
        token.validate()?;
        Ok(token)
    }

    /// Safely formats the token for logging purposes.
    ///
    /// This method masks the secret to prevent accidental exposure in logs.
    ///
    /// # Returns
    ///
    /// A string representation safe for logging.
    pub fn safe_display(&self) -> String {
        let masked_secret = if self.secret.len() > 8 {
            format!(
                "{}****{}",
                &self.secret[..4],
                &self.secret[self.secret.len() - 4..]
            )
        } else {
            "****".to_string()
        };
        format!(
            "Token {{ app_id: {}, secret: {} }}",
            self.app_id, masked_secret
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.safe_display())
    }
}

/// Implement custom Debug to avoid exposing secrets in debug output
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("app_id", &self.app_id)
            .field("secret", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new("123456", "secret123");
        assert_eq!(token.app_id(), "123456");
        assert_eq!(token.secret(), "secret123");
    }

    #[test]
    fn test_authorization_header() {
        let token = Token::new("test", "secret");
        let header = token.authorization_header();
        assert!(header.starts_with("QQBot "));

        // Verify the encoded content by manually creating expected encoding
        let expected_credentials = "test.secret";
        let expected_encoded = general_purpose::STANDARD.encode(expected_credentials.as_bytes());
        let expected_header = format!("QQBot {}", expected_encoded);
        assert_eq!(header, expected_header);

        // Also test decoding
        let encoded_part = &header[7..]; // Remove "QQBot " prefix
        if let Ok(decoded) = general_purpose::STANDARD.decode(encoded_part) {
            if let Ok(decoded_str) = String::from_utf8(decoded) {
                assert_eq!(decoded_str, "test.secret");
            }
        }
    }

    #[test]
    fn test_bot_token() {
        let token = Token::new("test", "secret");
        assert_eq!(token.bot_token(), token.authorization_header());
    }

    #[test]
    fn test_validation() {
        let valid_token = Token::new("123", "secret");
        assert!(valid_token.validate().is_ok());

        let empty_app_id = Token::new("", "secret");
        assert!(empty_app_id.validate().is_err());

        let empty_secret = Token::new("123", "");
        assert!(empty_secret.validate().is_err());
    }

    #[test]
    fn test_safe_display() {
        let token = Token::new("123456", "verylongsecret123");
        let display = token.safe_display();
        assert!(display.contains("123456"));
        assert!(display.contains("very"));
        assert!(display.contains("123"));
        assert!(display.contains("****"));
        assert!(!display.contains("longsecret"));

        let short_token = Token::new("123", "short");
        let short_display = short_token.safe_display();
        assert!(short_display.contains("****"));
        assert!(!short_display.contains("short"));
    }

    #[test]
    fn test_debug_format() {
        let token = Token::new("123456", "secret123");
        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("123456"));
        assert!(debug_str.contains("[REDACTED]"));
        assert!(!debug_str.contains("secret123"));
    }
}
