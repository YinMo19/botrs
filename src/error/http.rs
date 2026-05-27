use super::BotError;

/// Maps HTTP status codes to specific error types.
pub(crate) fn http_error_from_status(status: u16, message: String) -> BotError {
    match status {
        401 => BotError::AuthenticationFailed(message),
        403 => BotError::Forbidden(message),
        404 => BotError::NotFound(message),
        405 => BotError::MethodNotAllowed(message),
        429 => BotError::SequenceNumber(message),
        500 | 504 => BotError::Server(message),
        _ => BotError::api(status as u32, message),
    }
}
