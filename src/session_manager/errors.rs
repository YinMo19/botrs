use std::collections::HashSet;
use std::sync::LazyLock;

use crate::error::{CodeConnCloseCantIdentify, CodeConnCloseCantResume, sdk_error_from_error};

pub static CANNOT_RESUME_ERROR_CODES: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CodeConnCloseCantResume]));

pub static CANNOT_IDENTIFY_ERROR_CODES: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CodeConnCloseCantIdentify]));

pub fn can_not_resume(err: &(dyn std::error::Error + 'static)) -> bool {
    CANNOT_RESUME_ERROR_CODES.contains(&sdk_error_from_error(err).code())
}

pub fn can_not_identify(err: &(dyn std::error::Error + 'static)) -> bool {
    CANNOT_IDENTIFY_ERROR_CODES.contains(&sdk_error_from_error(err).code())
}
