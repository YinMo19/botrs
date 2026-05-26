use crate::api::BotApi;
use crate::error::Result;
use crate::models::api::PinsMessage;

impl BotApi {
    /// Pins add API.
    #[allow(non_snake_case)]
    pub async fn AddPins(&self, channel_id: &str, message_id: &str) -> Result<PinsMessage> {
        self.put_pin(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Pins delete API.
    #[allow(non_snake_case)]
    pub async fn DeletePins(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete_pin(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Pins clean API.
    #[allow(non_snake_case)]
    pub async fn CleanPins(&self, channel_id: &str) -> Result<()> {
        self.clean_pins(self.token_required()?, channel_id).await
    }

    /// Pins list API.
    #[allow(non_snake_case)]
    pub async fn GetPins(&self, channel_id: &str) -> Result<PinsMessage> {
        self.get_pins(self.token_required()?, channel_id).await
    }
}
