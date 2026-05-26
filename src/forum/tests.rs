use super::*;
use crate::BotApi;

#[test]
fn test_format() {
    assert_eq!(u8::from(Format::PlainText), 1);
    assert_eq!(u8::from(Format::Html), 2);
    assert_eq!(u8::from(Format::Markdown), 3);
    assert_eq!(u8::from(Format::Json), 4);
    assert_eq!(serde_json::to_value(Format::Markdown).unwrap(), 3);
    assert_eq!(
        serde_json::from_value::<Format>(serde_json::json!(4)).unwrap(),
        Format::Json
    );
    assert_eq!(
        serde_json::from_value::<Format>(serde_json::json!(99)).unwrap(),
        Format::Unknown(99)
    );
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
    assert_eq!(thread_info.thread_id, "thread-1");
    assert_eq!(thread_info.title, "{\"paragraphs\":[]}");
    assert_eq!(thread_info.content, "{\"paragraphs\":[{\"elems\":[]}]}");
    assert_eq!(thread_info.date_time, "2024-01-02T03:04:05+08:00");

    let value = serde_json::to_value(&thread_info).unwrap();
    assert_eq!(value["title"], serde_json::json!("{\"paragraphs\":[]}"));
    assert_eq!(
        value["content"],
        serde_json::json!("{\"paragraphs\":[{\"elems\":[]}]}")
    );
}

#[test]
fn forum_rest_models_match_botpy_shapes() {
    let body = ThreadToCreate::new("Title", "Content", Format::Markdown);
    assert_eq!(
        serde_json::to_value(&body).unwrap(),
        serde_json::json!({
            "title": "Title",
            "content": "Content",
            "format": 3
        })
    );

    let forum_rsp: ForumRsp = serde_json::from_value(serde_json::json!({
        "threads": [{
            "thread_id": "thread-1",
            "title": "Title",
            "content": "Content",
            "date_time": "2024-01-02T03:04:05+08:00"
        }],
        "is_finish": 1
    }))
    .unwrap();
    assert_eq!(forum_rsp.threads[0].thread_id, "thread-1");
    assert_eq!(forum_rsp.is_finish, 1);

    let post_rsp: PostThreadRsp = serde_json::from_value(serde_json::json!({
        "task_id": "task-1",
        "create_time": "1710000000"
    }))
    .unwrap();
    assert_eq!(post_rsp.task_id, "task-1");
    assert_eq!(post_rsp.create_time, "1710000000");
}

#[test]
fn forum_events_use_required_zero_value_fields() {
    let thread = serde_json::to_value(Thread::new(
        BotApi::new(crate::http::HttpClient::new(30, false).unwrap()),
        Some("event-1".to_string()),
        &serde_json::json!({}),
    ))
    .unwrap();
    assert_eq!(thread["guild_id"], "");
    assert_eq!(thread["channel_id"], "");
    assert_eq!(thread["author_id"], "");
    assert_eq!(thread["thread_info"]["thread_id"], "");
    assert_eq!(thread["thread_info"]["title"], "");
    assert_eq!(thread["thread_info"]["content"], "");
    assert_eq!(thread["thread_info"]["date_time"], "");
    assert!(thread.get("event_id").is_none());

    let post_info = serde_json::to_value(PostInfo::default()).unwrap();
    assert_eq!(post_info["thread_id"], "");
    assert_eq!(post_info["post_id"], "");
    assert_eq!(post_info["content"], "");
    assert_eq!(post_info["date_time"], "");

    let reply_info = serde_json::to_value(ReplyInfo::default()).unwrap();
    assert_eq!(reply_info["thread_id"], "");
    assert_eq!(reply_info["post_id"], "");
    assert_eq!(reply_info["reply_id"], "");
    assert_eq!(reply_info["content"], "");
    assert_eq!(reply_info["date_time"], "");
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

#[test]
fn forum_wrapper_event_ids_are_internal_only() {
    let http = crate::http::HttpClient::new(30, false).unwrap();
    let api = BotApi::new(http);
    let data = serde_json::json!({
        "guild_id": "guild-1",
        "channel_id": "channel-1",
        "author_id": "author-1",
        "thread_info": {
            "thread_id": "thread-1"
        },
        "post_info": {
            "thread_id": "thread-1",
            "post_id": "post-1"
        },
        "reply_info": {
            "thread_id": "thread-1",
            "post_id": "post-1",
            "reply_id": "reply-1"
        }
    });

    let thread = Thread::new(api.clone(), Some("event-1".to_string()), &data);
    let post = Post::new(api.clone(), Some("event-2".to_string()), &data);
    let reply = Reply::new(api.clone(), Some("event-3".to_string()), &data);
    let mut open_thread = OpenThread::new(api, &data);
    open_thread.event_id = Some("event-4".to_string());

    assert_eq!(thread.event_id.as_deref(), Some("event-1"));
    assert_eq!(post.event_id.as_deref(), Some("event-2"));
    assert_eq!(reply.event_id.as_deref(), Some("event-3"));
    assert_eq!(open_thread.event_id.as_deref(), Some("event-4"));

    for value in [
        serde_json::to_value(&thread).unwrap(),
        serde_json::to_value(&post).unwrap(),
        serde_json::to_value(&reply).unwrap(),
        serde_json::to_value(&open_thread).unwrap(),
    ] {
        assert!(value.get("event_id").is_none());
    }
}
