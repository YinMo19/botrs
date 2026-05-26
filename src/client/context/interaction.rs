use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn on_interaction_result(&self, interaction_id: &str, code: i32) -> Result<()> {
        self.api
            .on_interaction_result(&self.token, interaction_id, code)
            .await
    }

    pub async fn put_interaction(&self, interaction_id: &str, body: &str) -> Result<()> {
        self.api
            .put_interaction(&self.token, interaction_id, body)
            .await
    }
}
