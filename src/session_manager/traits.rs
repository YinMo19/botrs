use std::future::Future;
use std::pin::Pin;

use tokio::sync::mpsc;

use super::Session;
use crate::intents::Intents;
use crate::models::api::WebsocketAP;
use crate::models::gateway::GatewayEvent;
use crate::token::Token;

pub type SessionFuture = Pin<Box<dyn Future<Output = (Session, crate::Result<()>)> + Send>>;
pub type SessionConnectFn =
    dyn Fn(Session, mpsc::UnboundedSender<GatewayEvent>) -> SessionFuture + Send + Sync;
pub type BoxedSessionManager = Box<dyn SessionManager>;

/// Session manager interface for the QQ Bot Open API gateway.
#[async_trait::async_trait]
pub trait SessionManager: Send + Sync {
    async fn start(
        &mut self,
        ap_info: &WebsocketAP,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()>;

    #[allow(non_snake_case)]
    async fn Start(
        &mut self,
        ap_info: &WebsocketAP,
        token: Token,
        intents: Intents,
        event_sender: mpsc::UnboundedSender<GatewayEvent>,
    ) -> crate::Result<()> {
        self.start(ap_info, token, intents, event_sender).await
    }
}
