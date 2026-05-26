use super::Token;
use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.safe_display())
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.app_id == other.app_id && self.secret == other.secret
    }
}

impl Eq for Token {}

/// Implement custom Debug to avoid exposing secrets in debug output
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("app_id", &self.app_id)
            .field("secret", &"[REDACTED]")
            .finish()
    }
}
