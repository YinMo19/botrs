use std::collections::HashSet;
use std::sync::LazyLock;

use crate::error::{CodeConnCloseCantIdentify, CodeConnCloseCantResume};

pub static CanNotResumeErrSet: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CodeConnCloseCantResume]));

pub static CanNotIdentifyErrSet: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([CodeConnCloseCantIdentify]));

pub fn can_not_resume(err: &(dyn std::error::Error + 'static)) -> bool {
    CanNotResumeErrSet.contains(&crate::error::Error(err).Code())
}

#[allow(non_snake_case)]
pub fn CanNotResume(err: &(dyn std::error::Error + 'static)) -> bool {
    can_not_resume(err)
}

pub fn can_not_identify(err: &(dyn std::error::Error + 'static)) -> bool {
    CanNotIdentifyErrSet.contains(&crate::error::Error(err).Code())
}

#[allow(non_snake_case)]
pub fn CanNotIdentify(err: &(dyn std::error::Error + 'static)) -> bool {
    can_not_identify(err)
}
