use super::{BotApi, resource};
use crate::error::Result;
use crate::models::webhook::{
    HttpIdentity, HttpReady, HttpSession, WebhookValidationRequest, WebhookValidationResponse,
};
use crate::token::Token;
use std::collections::HashMap;
use tracing::debug;

impl BotApi {
    /// Creates a new HTTP webhook session.
    pub async fn create_session(
        &self,
        token: &Token,
        identity: &HttpIdentity,
    ) -> Result<HttpReady> {
        debug!("Creating HTTP webhook session");
        let response = self
            .http
            .post(
                token,
                resource::WEBHOOK_SESSIONS,
                None::<&()>,
                Some(identity),
            )
            .await?;
        Self::decode_json(response)
    }

    /// Checks HTTP webhook session health.
    pub async fn check_sessions(&self, token: &Token) -> Result<Vec<HttpSession>> {
        debug!("Checking HTTP webhook sessions");
        let mut params = HashMap::new();
        params.insert("action", "check");
        let response = self
            .http
            .patch(
                token,
                resource::WEBHOOK_SESSIONS,
                Some(&params),
                None::<&()>,
            )
            .await?;
        Self::decode_json(response)
    }

    /// Lists active HTTP webhook sessions.
    pub async fn session_list(&self, token: &Token) -> Result<Vec<HttpSession>> {
        debug!("Listing HTTP webhook sessions");
        let response = self
            .http
            .get(token, resource::WEBHOOK_SESSIONS, None::<&()>)
            .await?;
        Self::decode_json(response)
    }

    /// Removes an HTTP webhook session.
    pub async fn remove_session(&self, token: &Token, session_id: &str) -> Result<()> {
        debug!("Removing HTTP webhook session {}", session_id);
        let path = resource::webhook_session(session_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }

    /// Builds a webhook validation response from a request and signature.
    pub fn webhook_validation_response(
        request: &WebhookValidationRequest,
        signature: impl Into<String>,
        data_version: impl Into<String>,
    ) -> WebhookValidationResponse {
        WebhookValidationResponse {
            plain_token: request.plain_token.clone(),
            signature: signature.into(),
            data_version: data_version.into(),
        }
    }
}
