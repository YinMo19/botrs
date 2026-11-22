use crate::error::{BotError, CODE_CONN_CLOSE_CANT_IDENTIFY, CODE_CONN_CLOSE_CANT_RESUME};

pub(crate) fn can_not_resume(err: &BotError) -> bool {
    matches!(err, BotError::Sdk(err) if err.code() == CODE_CONN_CLOSE_CANT_RESUME)
}

pub(crate) fn can_not_identify(err: &BotError) -> bool {
    matches!(err, BotError::Sdk(err) if err.code() == CODE_CONN_CLOSE_CANT_IDENTIFY)
}
