use tokio::sync::mpsc;

use crate::intents::Intents;
use crate::models::api::GatewayResponse;
use crate::models::gateway::GatewayEvent;
use crate::token_impl::Token;

/// Session manager interface for the QQ Bot Open API gateway.
#[async_trait::async_trait]
pub trait SessionManager: Send + Sync {
    async fn start(
        &mut self,
        ap_info: &GatewayResponse,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()>;
}
