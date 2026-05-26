use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn create_session(&self, identity: &HttpIdentity) -> Result<HttpReady> {
        self.api.create_session(&self.token, identity).await
    }

    /// Checks HTTP webhook session health.

    pub async fn check_sessions(&self) -> Result<Vec<HttpSession>> {
        self.api.check_sessions(&self.token).await
    }

    /// Lists active HTTP webhook sessions.

    pub async fn session_list(&self) -> Result<Vec<HttpSession>> {
        self.api.session_list(&self.token).await
    }

    /// Removes an HTTP webhook session.

    pub async fn remove_session(&self, session_id: &str) -> Result<()> {
        self.api.remove_session(&self.token, session_id).await
    }
}
