//! Botgo-style websocket facade.

#![allow(non_snake_case, non_upper_case_globals)]

use std::sync::{LazyLock, RwLock};

use crate::intents::Intent;
use crate::models::gateway::WSPayload;
use crate::session_manager::Session;

pub const DefaultQueueSize: usize = 10_000;
pub static PanicBufLen: LazyLock<RwLock<usize>> = LazyLock::new(|| RwLock::new(1024));
pub static ResumeSignal: LazyLock<RwLock<Option<i32>>> = LazyLock::new(|| RwLock::new(None));

pub trait WebSocket: Send + Sync {
    fn New(&self, session: Session) -> Box<dyn WebSocket>;
    fn Connect(&mut self) -> crate::Result<()>;
    fn Identify(&mut self) -> crate::Result<()>;
    fn Session(&self) -> &Session;
    fn Resume(&mut self) -> crate::Result<()>;
    fn Listening(&mut self) -> crate::Result<()>;
    fn Write(&mut self, message: &WSPayload) -> crate::Result<()>;
    fn Close(&mut self);
}

pub type BoxedWebSocket = Box<dyn WebSocket>;
pub type WebSocketFactory = dyn Fn(Session) -> BoxedWebSocket + Send + Sync;

pub static ClientImpl: LazyLock<RwLock<Option<Box<WebSocketFactory>>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn RegisterFactory(factory: impl Fn(Session) -> BoxedWebSocket + Send + Sync + 'static) {
    *ClientImpl.write().expect("websocket client lock poisoned") = Some(Box::new(factory));
}

pub fn Register(ws: impl WebSocket + Clone + 'static) {
    RegisterFactory(move |session| ws.clone().New(session));
}

pub fn new_client(session: Session) -> Option<BoxedWebSocket> {
    ClientImpl
        .read()
        .expect("websocket client lock poisoned")
        .as_ref()
        .map(|factory| factory(session))
}

pub fn RegisterResumeSignal(signal: i32) {
    *ResumeSignal.write().expect("resume signal lock poisoned") = Some(signal);
}

pub fn PanicHandler(error: impl std::fmt::Display, session: &Session) {
    let len = *PanicBufLen.read().expect("panic buf len lock poisoned");
    crate::log::Errorf(format_args!(
        "[PANIC][ws][ID:{}][Shard:({}/{})][Intent:{}]\n{}\nstack capture len:{}",
        session.id,
        session.shards.shard_id,
        session.shards.shard_count,
        session.intent.bits,
        error,
        len
    ));
}

pub fn RegisterHandlers<H: crate::event::RegisterableHandler>(
    handlers: impl IntoIterator<Item = H>,
) -> Intent {
    crate::event::RegisterHandlers(handlers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct StubWebSocket {
        session: Session,
    }

    impl StubWebSocket {
        fn new() -> Self {
            Self {
                session: Session::new(
                    "wss://example.com",
                    crate::Token::new("app", "secret"),
                    crate::Intents::default(),
                    0,
                    1,
                ),
            }
        }
    }

    impl WebSocket for StubWebSocket {
        fn New(&self, session: Session) -> Box<dyn WebSocket> {
            Box::new(Self { session })
        }

        fn Connect(&mut self) -> crate::Result<()> {
            Ok(())
        }

        fn Identify(&mut self) -> crate::Result<()> {
            Ok(())
        }

        fn Session(&self) -> &Session {
            &self.session
        }

        fn Resume(&mut self) -> crate::Result<()> {
            Ok(())
        }

        fn Listening(&mut self) -> crate::Result<()> {
            Ok(())
        }

        fn Write(&mut self, _message: &WSPayload) -> crate::Result<()> {
            Ok(())
        }

        fn Close(&mut self) {}
    }

    #[test]
    fn websocket_register_factory_creates_clients() {
        Register(StubWebSocket::new());
        let session = Session::new(
            "wss://example.com",
            crate::Token::new("app", "secret"),
            crate::Intents::default(),
            0,
            1,
        );

        let client = new_client(session).expect("registered client");
        assert_eq!(client.Session().shards.shard_count, 1);
    }
}
