use crate::error::{BotError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub(super) struct TokenState {
    pub(super) access_token: Option<String>,
    pub(super) expires_at: Option<u64>,
    pub(super) expires_in: Option<u64>,
}

pub(super) fn default_state() -> Arc<Mutex<TokenState>> {
    Arc::new(Mutex::new(TokenState::default()))
}

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
    pub(super) app_id: String,
    /// The application secret provided by QQ
    pub(super) secret: String,
    /// Shared token cache and refresh lock.
    #[serde(skip, default = "default_state")]
    pub(super) state: Arc<Mutex<TokenState>>,
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
            state: default_state(),
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

    /// App ID accessor.
    #[allow(non_snake_case)]
    pub fn GetAppID(&self) -> &str {
        self.app_id()
    }

    /// Generates the authorization header value for API requests.
    ///
    /// The authorization header uses the format "QQBot {access_token}"
    /// where the access_token is obtained from the QQ API using app_id and secret.
    ///
    /// # Returns
    ///
    /// A string containing the authorization header value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::Token;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let token = Token::new("valid_app_id", "valid_secret");
    ///     let auth_header = token.authorization_header().await?;
    ///     assert!(auth_header.starts_with("QQBot "));
    ///     Ok(())
    /// }
    /// ```
    pub async fn authorization_header(&self) -> Result<String> {
        let access_token = self.access_token().await?;
        Ok(format!("QQBot {access_token}"))
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
    pub(super) async fn ensure_valid_token(&self) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| BotError::internal("Failed to get current time"))?
            .as_secs();

        let is_valid = {
            let state = self.state.lock().await;
            state.access_token.is_some() && state.expires_at.is_some_and(|exp| current_time < exp)
        };
        if !is_valid {
            self.refresh_access_token(current_time, false).await?;
        }

        Ok(())
    }

    /// Refreshes the access token by calling the QQ API.
    async fn refresh_access_token(&self, current_time: u64, force: bool) -> Result<()> {
        let mut state = self.state.lock().await;
        if !force
            && state.access_token.is_some()
            && state.expires_at.is_some_and(|exp| current_time < exp)
        {
            return Ok(());
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
            .map_err(|e| BotError::connection(format!("Failed to request access token: {e}")))?;

        if !response.status().is_success() {
            return Err(BotError::api(
                response.status().as_u16() as u32,
                format!(
                    "Token request failed: {}",
                    response.text().await.unwrap_or_default()
                ),
            ));
        }

        let token_response: serde_json::Value = response.json().await.map_err(BotError::Http)?;

        let access_token = token_response
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BotError::auth("No access_token in response"))?;

        let expires_in = token_response
            .get("expires_in")
            .and_then(super::parse_expires_in)
            .ok_or_else(|| BotError::auth("No expires_in in response"))?;

        state.access_token = Some(access_token.to_string());
        state.expires_at = Some(current_time + expires_in);
        state.expires_in = Some(expires_in);

        Ok(())
    }

    pub(super) async fn force_refresh_access_token(&self) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| BotError::internal("Failed to get current time"))?
            .as_secs();
        self.refresh_access_token(current_time, true).await
    }

    async fn access_token(&self) -> Result<String> {
        self.ensure_valid_token().await?;
        self.state
            .lock()
            .await
            .access_token
            .clone()
            .ok_or_else(|| BotError::auth("No valid access token available"))
    }

    pub(super) async fn cached_expires_in(&self) -> Option<u64> {
        self.state.lock().await.expires_in
    }

    #[cfg(test)]
    pub(crate) async fn set_cached_access_token_for_test(&self, access_token: impl Into<String>) {
        let mut state = self.state.lock().await;
        state.access_token = Some(access_token.into());
        state.expires_at = Some(u64::MAX);
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
