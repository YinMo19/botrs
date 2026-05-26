use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn put_interaction(&self, interaction_id: &str, body: &str) -> Result<()> {
        self.api
            .put_interaction(&self.token, interaction_id, body)
            .await
    }
}
