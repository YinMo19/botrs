use super::{BotApi, HeaderCallbackAppID, resource};
use crate::error::Result;
use crate::token::Token;
use reqwest::header::{HeaderMap, HeaderValue};
use tracing::debug;

impl BotApi {
    /// Sends a raw interaction response body with the callback app ID header.
    pub async fn put_interaction(
        &self,
        token: &Token,
        interaction_id: &str,
        body: &str,
    ) -> Result<()> {
        debug!("Updating interaction {}", interaction_id);
        let mut headers = HeaderMap::new();
        let app_id = if self.app_id.is_empty() {
            token.app_id()
        } else {
            &self.app_id
        };
        let app_id = HeaderValue::from_str(app_id)
            .map_err(|e| crate::BotError::invalid_data(format!("Invalid app ID header: {e}")))?;
        headers.insert(HeaderCallbackAppID, app_id);

        let path = resource::interaction(interaction_id);
        self.http
            .put_raw_with_headers(token, &path, None::<&()>, body, headers)
            .await?;
        Ok(())
    }
}
