use super::*;

#[test]
fn test_sdk_error_helpers() {
    let err = sdk_error(CODE_NEED_RECONNECT, "need reconnect");
    assert_eq!(err.code(), CODE_NEED_RECONNECT);
    assert_eq!(err.message(), "need reconnect");
    assert_eq!(err.trace_id(), "");
    assert_eq!(err.to_string(), "code:9000, text:need reconnect, traceID:");

    assert_eq!(invalid_session_error().code(), CODE_CONN_CLOSE_CANT_RESUME);
}
