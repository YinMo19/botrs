use crate::api::BotApi;
use crate::error::Result;

impl BotApi {
    /// Interaction update API.
    #[allow(non_snake_case)]
    pub async fn PutInteraction(&self, interaction_id: &str, body: &str) -> Result<()> {
        self.put_interaction(self.token_required()?, interaction_id, body)
            .await
    }
}
