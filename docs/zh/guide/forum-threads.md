# 论坛与主题

启用 `Intents::FORUMS`（特权）或 `Intents::OPEN_FORUM_EVENT`（公开）后，论坛事件会通过网关投递。框架将网关载荷解码为类型化的结构体并交给 `EventHandler`，因此论坛处理是事件驱动的。

## 私域论坛回调

启用 `Intents::FORUMS` 后，`EventHandler` 会触发以下方法：

- `forum_thread_create(&self, session: ThreadSession)`
- `forum_thread_update(&self, session: ThreadSession)`
- `forum_thread_delete(&self, session: ThreadSession)`
- `forum_post_create(&self, session: PostSession)`
- `forum_post_delete(&self, session: PostSession)`
- `forum_reply_create(&self, session: ForumReplySession)`
- `forum_reply_delete(&self, session: ForumReplySession)`
- `forum_publish_audit_result(&self, session: ForumAuditSession)`

## 公开论坛回调

`Intents::OPEN_FORUM_EVENT` 触发的所有事件共用 `OpenThread` 一种载荷。当前它携带可选的 `guild_id`、`channel_id`、`author_id` 和内部 `event_id`：

- `open_forum_thread_create` / `_update` / `_delete`
- `open_forum_post_create` / `_delete`
- `open_forum_reply_create` / `_delete`

## 载荷结构

`Thread` 带有 `channel_id`、`guild_id`、`author_id`、`event_id`（`Option<String>`），以及 `thread_info: ThreadInfo`。`ThreadInfo` 暴露 `title`、`content`、`thread_id` 和 `date_time`。

`Post` 与 `Reply` 形态相同，仅把 `thread_info` 换成 `post_info: PostInfo` / `reply_info: ReplyInfo`；这些 info 结构保留 id、字符串 content 和创建时间。论坛事件需要发起 REST 调用时，直接使用回调里的 session。

`ForumAuditResult` 直接映射审核载荷：

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
    pub result: u32,        // 0 = 通过，非 0 = 驳回
    pub err_msg: String,
    pub date_time: String,
    pub event_id: Option<String>,
}
```

每次审核结果触发一次 `forum_publish_audit_result`。网关层会自动 ACK，处理器无需主动回执。`result == 0` 表示通过，否则 `err_msg` 即平台给出的拒绝原因。

## 读取内容

论坛 info 结构通过 `botrs::models::forum` 暴露。当前公开 DTO 将正文保留为 `String`；如果业务需要结构化文本或附件提取，请在应用代码里解析这个字符串。

## 最小处理器示例

```rust
async fn forum_publish_audit_result(&self, session: ForumAuditSession) {
    let result = session.event();
    if result.result != 0 {
        tracing::warn!(
            task = result.task_id,
            err = result.err_msg,
            "论坛发布被驳回"
        );
    }
}

async fn forum_thread_create(&self, session: ThreadSession) {
    let thread = session.event();
    tracing::info!(
        guild = ?thread.guild_id,
        channel = ?thread.channel_id,
        "新论坛主题"
    );
}
```
