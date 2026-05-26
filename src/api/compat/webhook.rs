use crate::api::BotApi;
use crate::error::Result;
use crate::models::webhook::{HttpIdentity, HttpReady, HttpSession};

impl BotApi {
    /// HTTP webhook session creation API.
    #[allow(non_snake_case)]
    pub async fn CreateSession(&self, identity: HttpIdentity) -> Result<HttpReady> {
        self.create_session(self.token_required()?, &identity).await
    }

    /// HTTP webhook session check API.
    #[allow(non_snake_case)]
    pub async fn CheckSessions(&self) -> Result<Vec<HttpSession>> {
        self.check_sessions(self.token_required()?).await
    }

    /// HTTP webhook session list API.
    #[allow(non_snake_case)]
    pub async fn SessionList(&self) -> Result<Vec<HttpSession>> {
        self.session_list(self.token_required()?).await
    }

    /// HTTP webhook session remove API.
    #[allow(non_snake_case)]
    pub async fn RemoveSession(&self, session_id: &str) -> Result<()> {
        self.remove_session(self.token_required()?, session_id)
            .await
    }
}
