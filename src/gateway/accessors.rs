use super::Gateway;
use std::sync::atomic::Ordering;

impl Gateway {
    /// Returns true if the gateway is connected and ready.
    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::Relaxed)
    }

    /// Returns true if the gateway can reconnect.
    pub fn can_reconnect(&self) -> bool {
        self.can_reconnect.load(Ordering::Relaxed)
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
