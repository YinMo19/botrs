use super::Gateway;
use std::sync::atomic::Ordering;

impl Gateway {
    /// Returns true if the gateway is connected and ready.
    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::Relaxed)
    }

    /// Gets the current session ID.
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Gets the last sequence number.
    pub fn last_sequence(&self) -> u64 {
        self.last_seq.load(Ordering::Relaxed)
    }
}
