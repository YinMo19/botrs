use std::collections::HashSet;
use std::sync::LazyLock;

use crate::error::{
    CODE_CONN_CLOSE_CANT_IDENTIFY, CODE_CONN_CLOSE_CANT_RESUME, sdk_error_from_error,
};

static CANNOT_RESUME_ERROR_CODES: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CODE_CONN_CLOSE_CANT_RESUME]));

static CANNOT_IDENTIFY_ERROR_CODES: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CODE_CONN_CLOSE_CANT_IDENTIFY]));

pub fn can_not_resume(err: &(dyn std::error::Error + 'static)) -> bool {
    CANNOT_RESUME_ERROR_CODES.contains(&sdk_error_from_error(err).code())
}

pub fn can_not_identify(err: &(dyn std::error::Error + 'static)) -> bool {
    CANNOT_IDENTIFY_ERROR_CODES.contains(&sdk_error_from_error(err).code())
}
