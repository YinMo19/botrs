use super::*;

#[test]
fn test_sdk_error_helpers() {
    let err = sdk_error(CodeNeedReConnect, "need reconnect");
    assert_eq!(err.code(), CodeNeedReConnect);
    assert_eq!(err.message(), "need reconnect");
    assert_eq!(err.trace_id(), "");
    assert_eq!(err.to_string(), "code:9000, text:need reconnect, traceID:");

    assert_eq!(missing_pager_error().code(), CodePagerIsNil);
    assert_eq!(invalid_session_error().code(), CodeConnCloseCantResume);
}
