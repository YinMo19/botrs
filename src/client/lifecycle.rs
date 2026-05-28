use super::prelude::*;
use super::{Client, Context, EventHandler};
use crate::gateway::Gateway;
use crate::http::HttpClient;

impl<H: EventHandler + 'static> Client<H> {
    /// Creates a client using the crate default request timeout.
    ///
    /// The handler is shared across all gateway shards and receives the events
    /// selected by `intents`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{Client, Token, Intents, EventHandler, Context};
    /// use tracing::info;
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl EventHandler for MyHandler {
    ///     async fn message_create(&self, ctx: Context, message: botrs::Message) {
    ///         info!("Received message: {:?}", message.content);
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
        let timeout = crate::DEFAULT_TIMEOUT;

        let http = HttpClient::new(timeout, is_sandbox)?;
        token.validate()?;
        let api = Arc::new(BotApi::new(http, token));

        Ok(Self {
            intents,
            api,
            handler: Arc::new(handler),
        })
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
        let api = Arc::new(BotApi::new(http, token));

        Ok(Self {
            intents,
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

        // Set up event channel
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel();

        let start_interval =
            Gateway::session_start_interval(gateway_info.session_start_limit.max_concurrency);
        debug!(
            "Gateway session start interval: {:?} (max_concurrency: {})",
            start_interval, gateway_info.session_start_limit.max_concurrency
        );

        info!(
            "Starting {} gateway shard(s) with interval {:?}",
            gateway_info.shards, start_interval
        );
        let mut session_manager = new_session_manager();
        tokio::spawn({
            let gateway_info = gateway_info.clone();
            let token = self.api.token().clone();
            let intents = self.intents;
            let event_sender = event_sender.clone();
            async move {
                if let Err(e) = session_manager
                    .start(&gateway_info, token, intents, event_sender)
                    .await
                {
                    error!("Gateway session manager stopped: {}", e);
                }
            }
        });
        drop(event_sender);

        // Main event processing loop - continue running even if gateway disconnects
        info!("Bot client started, waiting for events...");
        while let Some(event) = event_receiver.recv().await {
            if let Err(e) = self.handle_event(ctx.clone(), event).await {
                self.handler.error(e).await;
            }
        }

        info!("Bot client stopped");
        Ok(())
    }
}
