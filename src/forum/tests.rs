use super::*;

#[test]
fn test_format() {
    assert_eq!(Format::PlainText as u8, 1);
    assert_eq!(Format::Html as u8, 2);
    assert_eq!(Format::Markdown as u8, 3);
    assert_eq!(Format::Json as u8, 4);
}

#[test]
fn test_text_creation() {
    let data = serde_json::json!({
        "text": "Hello, world!"
    });
    let text = Text::new(&data);
    assert_eq!(text.text, Some("Hello, world!".to_string()));
}

#[test]
fn thread_info_keeps_title_and_content_as_strings() {
    let data = serde_json::json!({
        "thread_id": "thread-1",
        "title": "{\"paragraphs\":[]}",
        "content": "{\"paragraphs\":[{\"elems\":[]}]}",
        "date_time": "2024-01-02T03:04:05+08:00"
    });

    let thread_info = ThreadInfo::new(&data);
    assert_eq!(thread_info.thread_id.as_deref(), Some("thread-1"));
    assert_eq!(thread_info.title.as_deref(), Some("{\"paragraphs\":[]}"));
    assert_eq!(
        thread_info.content.as_deref(),
        Some("{\"paragraphs\":[{\"elems\":[]}]}")
    );
    assert_eq!(
        thread_info.date_time.as_deref(),
        Some("2024-01-02T03:04:05+08:00")
    );

    let value = serde_json::to_value(&thread_info).unwrap();
    assert_eq!(value["title"], serde_json::json!("{\"paragraphs\":[]}"));
    assert_eq!(
        value["content"],
        serde_json::json!("{\"paragraphs\":[{\"elems\":[]}]}")
    );
}

#[test]
fn forum_audit_result_serializes_zero_value_strings() {
    // The QQ Bot Open API audit payload defines every field as a bare
    // string/integer; zero values must serialize as `""`/`0` rather than
    // being omitted or rendered as `null`.
    let data = serde_json::json!({
        "task_id": "task-1",
        "guild_id": "guild-1",
        "channel_id": "channel-1",
        "author_id": "author-1",
        "thread_id": "thread-1",
        "post_id": "",
        "reply_id": "",
        "type": 1,
        "result": 2,
        "err_msg": "",
        "date_time": "2024-01-02T03:04:05+08:00"
    });
    let parsed = ForumAuditResult::new(Some("event-1".into()), &data);

    assert_eq!(parsed.task_id, "task-1");
    assert_eq!(parsed.publish_type, 1);
    assert_eq!(parsed.result, 2);
    assert_eq!(parsed.event_id.as_deref(), Some("event-1"));

    let value = serde_json::to_value(ForumAuditResult::default()).unwrap();
    assert_eq!(value["task_id"], "");
    assert_eq!(value["guild_id"], "");
    assert_eq!(value["type"], 0);
    assert_eq!(value["result"], 0);
    assert_eq!(value["date_time"], "");
    // event_id is internal-only and never appears on the wire.
    assert!(value.get("event_id").is_none());
}
