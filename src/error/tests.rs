use super::*;

#[test]
fn test_sdk_error_helpers() {
    let err = sdk_error(9999, "unknown sdk error");
    assert_eq!(err.code(), 9999);
    assert_eq!(err.message(), "unknown sdk error");
    assert_eq!(err.trace_id(), "");
    assert_eq!(
        err.to_string(),
        "code:9999, text:unknown sdk error, traceID:"
    );

    assert_eq!(invalid_session_error().code(), CODE_CONN_CLOSE_CANT_RESUME);
}
