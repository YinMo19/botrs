# 论坛与主题

启用 `Intents::FORUMS`（特权）或 `Intents::OPEN_FORUM_EVENT`（公开）后，论坛事件会通过网关投递。框架将网关载荷解码为类型化的结构体并交给 `EventHandler`。`BotApi` 当前并没有创建／删除主题的 REST 接口 —— 机器人是通知消费者而非作者。

## 私域论坛回调

启用 `Intents::FORUMS` 后，`EventHandler` 会触发以下方法：

- `forum_thread_create(&self, ctx, thread: Thread)`
- `forum_thread_update(&self, ctx, thread: Thread)`
- `forum_thread_delete(&self, ctx, thread: Thread)`
- `forum_post_create(&self, ctx, post: Post)`
- `forum_post_delete(&self, ctx, post: Post)`
- `forum_reply_create(&self, ctx, reply: Reply)`
- `forum_reply_delete(&self, ctx, reply: Reply)`
- `forum_publish_audit_result(&self, ctx, result: ForumAuditResult)`

## 公开论坛回调

`Intents::OPEN_FORUM_EVENT` 触发的所有事件共用 `OpenThread` 一种载荷，依据子事件不同它会带 `thread_info` / `post_info` / `reply_info` 中的某一项：

- `open_forum_thread_create` / `_update` / `_delete`
- `open_forum_post_create` / `_delete`
- `open_forum_reply_create` / `_delete`

## 载荷结构

`Thread` 带有 `channel_id`、`guild_id`、`author_id`、`event_id`（均为 `Option<String>`），以及 `thread_info: ThreadInfo`，其中 `ThreadInfo` 暴露主题标题和编辑器生成的富文本块。

`Post` 与 `Reply` 形态相同，仅把 `thread_info` 换成 `post_info: PostInfo` / `reply_info: ReplyInfo`。论坛事件需要发起 REST 调用时，直接使用回调里的 `ctx`。

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

## 解析富文本

`ThreadInfo`、`PostInfo`、`ReplyInfo` 反序列化 QQ 的“段落”结构：

- `Content { paragraphs: Vec<Paragraph> }`
- `Paragraph { elems: Vec<Elem>, props }`
- `Elem` 为标签联合（`Text`、`Image`、`Video`、`Url`）。

字段命名见 `botrs::forum`；从主题正文里提取文字或附件通常通过对 `Elem` 模式匹配实现。

## 最小处理器示例

```rust
async fn forum_publish_audit_result(&self, _ctx: Context, result: ForumAuditResult) {
    if result.result != 0 {
        tracing::warn!(
            task = result.task_id,
            err = result.err_msg,
            "论坛发布被驳回"
        );
    }
}

async fn forum_thread_create(&self, _ctx: Context, thread: Thread) {
    tracing::info!(
        guild = ?thread.guild_id,
        channel = ?thread.channel_id,
        "新论坛主题"
    );
}
```
