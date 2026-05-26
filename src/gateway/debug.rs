use super::Gateway;

impl std::fmt::Debug for Gateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gateway")
            .field("url", &self.url)
            .field("intents", &self.intents)
            .field("shard", &self.shard)
            .field("session_id", &self.session_id)
            .field("is_ready", &self.is_ready())
            .finish()
    }
}
