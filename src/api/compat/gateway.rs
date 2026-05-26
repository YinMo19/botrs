use crate::api::BotApi;
use crate::error::Result;
use crate::models::api::GatewayResponse;
use std::collections::HashMap;

impl BotApi {
    /// Websocket gateway address API.
    #[allow(non_snake_case)]
    pub async fn WS(
        &self,
        _params: Option<&HashMap<String, String>>,
        _body: Option<&str>,
    ) -> Result<GatewayResponse> {
        self.get_gateway(self.token_required()?).await
    }
}
