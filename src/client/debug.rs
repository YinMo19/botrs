use super::{Client, EventHandler};

impl<H: EventHandler> std::fmt::Debug for Client<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("intents", &self.intents)
            .field("is_sandbox", &self.is_sandbox)
            .field("timeout", &self.timeout)
            .finish()
    }
}
