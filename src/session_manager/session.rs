use crate::intents::Intents;
use crate::token::Token;

/// Websocket session descriptor.
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub url: String,
    pub token: Token,
    pub intent: Intents,
    pub last_seq: u64,
    pub shards: crate::models::api::ShardConfig,
    pub app_id: Option<String>,
}

impl Session {
    pub fn new(
        url: impl Into<String>,
        token: Token,
        intent: Intents,
        shard_id: u32,
        shard_count: u32,
    ) -> Self {
        Self {
            id: String::new(),
            url: url.into(),
            token,
            intent,
            last_seq: 0,
            shards: crate::models::api::ShardConfig {
                shard_id,
                shard_count,
            },
            app_id: None,
        }
    }

    pub fn shard(&self) -> [u32; 2] {
        [self.shards.shard_id, self.shards.shard_count]
    }

    pub fn from_app_id(app_id: impl Into<String>) -> Self {
        Self {
            id: String::new(),
            url: String::new(),
            token: Token::new("", ""),
            intent: Intents::default(),
            last_seq: 0,
            shards: crate::models::api::ShardConfig {
                shard_id: 0,
                shard_count: 0,
            },
            app_id: Some(app_id.into()),
        }
    }

    #[allow(non_snake_case)]
    pub fn FromAppID(app_id: impl Into<String>) -> Self {
        Self::from_app_id(app_id)
    }
}
