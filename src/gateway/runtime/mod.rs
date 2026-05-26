mod auth;
mod connect;
mod dispatch;
mod event_loop;
mod system;

use super::types::WsStream;
use futures_util::stream::SplitSink;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

type SharedWriter = Arc<Mutex<SplitSink<WsStream, Message>>>;
