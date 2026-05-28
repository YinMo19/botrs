use crate::intents::Intents;
use crate::token_impl::Token;

/// Websocket session descriptor.
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub url: String,
    pub token: Token,
    pub intent: Intents,
    pub last_seq: u64,
    pub shards: crate::models::api::ShardConfig,
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
        }
    }

    pub fn shard(&self) -> [u32; 2] {
        [self.shards.shard_id, self.shards.shard_count]
    }
}
