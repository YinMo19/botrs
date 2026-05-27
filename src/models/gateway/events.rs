pub mod opcodes {
    /// Dispatch event
    pub const DISPATCH: u8 = 0;
    /// Heartbeat
    pub const HEARTBEAT: u8 = 1;
    /// Identify
    pub const IDENTIFY: u8 = 2;
    /// Resume
    pub const RESUME: u8 = 6;
    /// Reconnect
    pub const RECONNECT: u8 = 7;
    /// Invalid session
    pub const INVALID_SESSION: u8 = 9;
    /// Hello
    pub const HELLO: u8 = 10;
    /// Heartbeat ACK
    pub const HEARTBEAT_ACK: u8 = 11;
}
