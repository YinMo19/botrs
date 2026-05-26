use super::*;

#[test]
fn test_err_helpers() {
    let err = New(CodeNeedReConnect, "need reconnect");
    assert_eq!(err.Code(), CodeNeedReConnect);
    assert_eq!(err.Text(), "need reconnect");
    assert_eq!(err.Trace(), "");
    assert_eq!(err.to_string(), "code:9000, text:need reconnect, traceID:");

    assert_eq!(err_pager_is_nil().Code(), CodePagerIsNil);
    assert_eq!(err_invalid_session().Code(), CodeConnCloseCantResume);
}
