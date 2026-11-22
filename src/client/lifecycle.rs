use super::prelude::*;
use super::{Client, Context, EventHandler};
use crate::http::HttpClient;
use crate::session_manager::{ChanManager, SessionManager};

impl<H: EventHandler + 'static> Client<H> {
    /// Creates a client using the crate default request timeout.
    ///
    /// The handler is shared across all gateway shards and receives the events
    /// selected by `intents`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{ChannelReplySession, Client, Token, Intents, EventHandler};
    /// use tracing::info;
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl EventHandler for MyHandler {
    ///     async fn message_create(&self, session: ChannelReplySession) {
    ///         info!("Received message: {:?}", session.message().content);
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let token = Token::new("app_id", "secret");
    ///     let intents = Intents::default();
    ///     let handler = MyHandler;
    ///     let client = Client::new(token, intents, handler, false)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(token: Token, intents: Intents, handler: H, is_sandbox: bool) -> Result<Self> {
        Self::with_config(token, intents, handler, crate::DEFAULT_TIMEOUT, is_sandbox)
    }

    /// Creates a client with an explicit HTTP request timeout.
    pub fn with_config(
        token: Token,
        intents: Intents,
        handler: H,
        timeout: u64,
        is_sandbox: bool,
    ) -> Result<Self> {
        let http = HttpClient::new(timeout, is_sandbox)?;
        token.validate()?;
        let webhook_key = super::webhook::signing_key(token.secret())?;
        let api = Arc::new(BotApi::new(http, token));

        Ok(Self {
            intents,
            webhook_key,
            api,
            handler: Arc::new(handler),
        })
    }

    /// Connects to the gateway and processes events until the session manager stops.
    ///
    /// Handler errors are passed to [`EventHandler::error`] while the event loop
    /// continues to receive later gateway events.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{Client, Token, Intents, EventHandler};
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl EventHandler for MyHandler {}
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let token = Token::new("app_id", "secret");
    ///     let intents = Intents::default();
    ///     let handler = MyHandler;
    ///     let mut client = Client::new(token, intents, handler, false)?;
    ///     client.start().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting bot client");

        // Get bot information
        let bot_info = self.api.get_bot_info().await?;
        info!("Bot info: {} ({})", bot_info.username, bot_info.id);

        // Get gateway information
        let gateway_info = self.api.get_gateway().await?;
        info!("Gateway URL: {}", gateway_info.url);

        check_session_limit(&gateway_info)?;

        // Create context
        let ctx = Context::new(self.api.clone()).with_bot_info(bot_info);

        // Bound queued payloads across shards when application handlers fall behind.
        let (event_sender, mut event_receiver) = mpsc::channel(1024);

        let mut manager = ChanManager::new();
        let run = manager.start(
            &gateway_info,
            self.api.token().clone(),
            self.intents,
            event_sender,
        );
        let dispatch = async {
            while let Some(event) = event_receiver.recv().await {
                if let Err(err) = self.handle_event(ctx.clone(), event).await {
                    self.handler.error(err).await;
                }
            }
            Ok(())
        };
        tokio::try_join!(run, dispatch)?;
        Ok(())
    }
}
