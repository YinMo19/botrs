//! Authentication token management for QQ Guild Bot API.
//!
//! This module provides the `Token` struct for managing bot authentication
//! credentials including app ID and secret, with access token management.

use crate::error::{BotError, Result};
// use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

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
    /// The current access token (fetched from QQ API)
    #[serde(skip)]
    access_token: Option<String>,
    /// When the access token expires (Unix timestamp)
    #[serde(skip)]
    expires_at: Option<u64>,
    /// Mutex to prevent concurrent token refresh
    #[serde(skip)]
    refresh_mutex: Arc<Mutex<()>>,
}

impl Token {
    /// Creates a new token with the given app ID and secret.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The bot's application ID
    /// * `secret` - The bot's secret key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use botrs::Token;
    ///
    /// let token = Token::new("123", "secret");
    /// assert_eq!(token.app_id(), "123");
    /// ```
    pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            secret: secret.into(),
            access_token: None,
            expires_at: None,
            refresh_mutex: Arc::new(Mutex::new(())),
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
    pub async fn authorization_header(&self) -> Result<String> {
        self.ensure_valid_token().await?;
        if let Some(access_token) = &self.access_token {
            Ok(format!("QQBot {}", access_token))
        } else {
            Err(BotError::auth("No valid access token available"))
        }
    }

    /// Generates the bot token for WebSocket authentication.
    ///
    /// The bot token uses the format "QQBot {access_token}"
    /// which is the same as the authorization header.
    ///
    /// # Returns
    ///
    /// A string containing the bot token.
    pub async fn bot_token(&self) -> Result<String> {
        self.authorization_header().await
    }

    /// Ensures the token has a valid access token, refreshing if necessary.
    async fn ensure_valid_token(&self) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| BotError::internal("Failed to get current time"))?
            .as_secs();

        // Check if we need to refresh the token
        if self.access_token.is_none() || self.expires_at.map_or(true, |exp| current_time >= exp) {
            self.refresh_access_token().await?;
        }

        Ok(())
    }

    /// Refreshes the access token by calling the QQ API.
    async fn refresh_access_token(&self) -> Result<()> {
        let _guard = self.refresh_mutex.lock().await;

        // Double-check in case another thread already refreshed it
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| BotError::internal("Failed to get current time"))?
            .as_secs();

        if let Some(expires_at) = self.expires_at {
            if current_time < expires_at && self.access_token.is_some() {
                return Ok(());
            }
        }

        // Create HTTP client for token request
        let client = reqwest::Client::new();
        let request_body = serde_json::json!({
            "appId": self.app_id,
            "clientSecret": self.secret
        });

        let response = client
            .post("https://bots.qq.com/app/getAppAccessToken")
            .json(&request_body)
            .timeout(std::time::Duration::from_secs(20))
            .send()
            .await
            .map_err(|e| BotError::connection(format!("Failed to request access token: {}", e)))?;

        if !response.status().is_success() {
            return Err(BotError::api(
                response.status().as_u16() as u32,
                format!(
                    "Token request failed: {}",
                    response.text().await.unwrap_or_default()
                ),
            ));
        }

        let token_response: serde_json::Value =
            response.json().await.map_err(|e| BotError::Http(e))?;

        let access_token = token_response
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BotError::auth("No access_token in response"))?;

        let expires_in = token_response
            .get("expires_in")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| BotError::auth("No expires_in in response"))?;

        // Store the new token (we need to use unsafe here since we can't have mutable references)
        unsafe {
            let self_mut = self as *const Self as *mut Self;
            (*self_mut).access_token = Some(access_token.to_string());
            (*self_mut).expires_at = Some(current_time + expires_in);
        }

        Ok(())
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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.app_id == other.app_id && self.secret == other.secret
    }
}

impl Eq for Token {}

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

    #[tokio::test]
    async fn test_authorization_header() {
        let token = Token::new("test", "secret");

        // Since we don't have real credentials, this should fail
        let result = token.authorization_header().await;
        assert!(result.is_err());

        // The error should be related to authentication
        match result {
            Err(crate::error::BotError::Connection(_))
            | Err(crate::error::BotError::Api { .. }) => {
                // Expected - we don't have valid credentials
            }
            _ => panic!("Expected connection or API error"),
        }
    }

    #[tokio::test]
    async fn test_bot_token() {
        let token = Token::new("test", "secret");
        // In real usage, both methods would fetch the same access token
        // For this test, we just verify they both start with "QQBot "
        let bot_token_result = token.bot_token().await;
        let auth_header_result = token.authorization_header().await;

        // Both should fail in the same way since we don't have real credentials
        assert!(bot_token_result.is_err());
        assert!(auth_header_result.is_err());
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
