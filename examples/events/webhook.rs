//! Serve the QQ callback at POST /qq. Set QQ_BOT_APP_ID and QQ_BOT_SECRET.
use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, Router, body::Bytes, routing::post};
use botrs::client::webhook::WebhookReply;
use botrs::{BotError, Client, Event, EventHandler, Intents, Token};

struct Bot;

#[async_trait::async_trait]
impl EventHandler for Bot {
    async fn handle(&self, event: Event) -> botrs::Result<()> {
        if let Event::C2CMessageCreate(mut session) = event {
            session.reply("Pong!").await?;
        }
        Ok(())
    }
}

async fn callback(
    State(client): State<Arc<Client<Bot>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<WebhookReply>, StatusCode> {
    client
        .handle_webhook(&headers, &body)
        .await
        .map(Json)
        .map_err(|err| match err {
            BotError::Auth(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::BAD_REQUEST,
        })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Token::from_env()?, Intents::new(), Bot, false)?;
    let app = Router::new()
        .route("/qq", post(callback))
        .with_state(Arc::new(client));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
