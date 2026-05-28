# Forum and threads

Forum events arrive over the gateway when the `Intents::FORUMS` (private) or `Intents::OPEN_FORUM_EVENT` (public) bits are set. The framework decodes the gateway payload into typed structs and dispatches to your `EventHandler`, so forum handling is event-driven.

## Private forum callbacks

With `Intents::FORUMS`, the following `EventHandler` methods become live:

- `forum_thread_create(&self, session: ThreadSession)`
- `forum_thread_update(&self, session: ThreadSession)`
- `forum_thread_delete(&self, session: ThreadSession)`
- `forum_post_create(&self, session: PostSession)`
- `forum_post_delete(&self, session: PostSession)`
- `forum_reply_create(&self, session: ForumReplySession)`
- `forum_reply_delete(&self, session: ForumReplySession)`
- `forum_publish_audit_result(&self, session: ForumAuditSession)`

## Public forum callbacks

`Intents::OPEN_FORUM_EVENT` enables the same lifecycle on a single payload type, `OpenThread`, which carries optional `thread_info`, `post_info`, and `reply_info` blocks depending on which sub-event fired:

- `open_forum_thread_create` / `_update` / `_delete`
- `open_forum_post_create` / `_delete`
- `open_forum_reply_create` / `_delete`

## Payload shapes

`Thread` carries `channel_id`, `guild_id`, `author_id`, `event_id` (`Option<String>`), and a `thread_info: ThreadInfo`. `ThreadInfo` exposes the title and rich-content blocks the forum editor produced.

`Post` and `Reply` follow the same shape but with `post_info: PostInfo` / `reply_info: ReplyInfo` instead. Use the callback session when a forum event needs to make REST calls.

`ForumAuditResult` mirrors the audit payload directly:

```rust
pub struct ForumAuditResult {
    pub task_id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub author_id: String,
    pub thread_id: String,
    pub post_id: String,
    pub reply_id: String,
    pub publish_type: u32,
    pub result: u32,        // 0 = pass, non-zero = reject
    pub err_msg: String,
    pub date_time: String,
    pub event_id: Option<String>,
}
```

`forum_publish_audit_result` fires once per audit decision. The framework's gateway layer ACKs the dispatch automatically — your handler does not need to send anything back. `result == 0` indicates the publish went through; otherwise `err_msg` carries the reason supplied by the platform.

## Reading rich content

`ThreadInfo`, `PostInfo`, and `ReplyInfo` deserialize the QQ "paragraph" structure:

- `Content { paragraphs: Vec<Paragraph> }`
- `Paragraph { elems: Vec<Elem>, props }`
- `Elem` is a tagged union (`Text`, `Image`, `Video`, `Url`).

The exact field names and types are in `botrs::forum`; pattern-matching `Elem` is the typical way to extract text or attachments from a thread body.

## Minimal handler

```rust
async fn forum_publish_audit_result(&self, session: ForumAuditSession) {
    let result = session.event();
    if result.result != 0 {
        tracing::warn!(
            task = result.task_id,
            err = result.err_msg,
            "forum publish rejected"
        );
    }
}

async fn forum_thread_create(&self, session: ThreadSession) {
    let thread = session.event();
    tracing::info!(
        guild = ?thread.guild_id,
        channel = ?thread.channel_id,
        "new forum thread"
    );
}
```
