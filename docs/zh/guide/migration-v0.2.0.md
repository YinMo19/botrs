# v0.2.0 消息 API 迁移

`botrs` 0.2.0 唯一的破坏性变更是发送消息的接口：原本接收 10 多个位置 `Option` 参数的五个 `BotApi` 方法被替换为接收类型化构建器的 `*_with_params` 方法。旧方法保留并标注 `#[deprecated]`，将在 0.3 中移除。

## 改动一览

| 旧方法                          | 新方法                                      | 构建器                 |
|---------------------------------|---------------------------------------------|------------------------|
| `post_message`                  | `post_message_with_params`                  | `MessageParams`        |
| `post_group_message`            | `post_group_message_with_params`            | `GroupMessageParams`   |
| `post_c2c_message`              | `post_c2c_message_with_params`              | `C2CMessageParams`     |
| `post_dms`                      | `post_dms_with_params`                      | `DirectMessageParams`  |
| `patch_message`（旧版）         | `patch_message_with_params`                 | `MessageParams`        |

所有构建器都提供 `new_text(content)` 与 `with_reply(message_id)`。`MessageParams` 与 `DirectMessageParams` 额外提供 `with_file_image(&bytes)`。其余字段（embed、ark、markdown、keyboard、media 等）通过结构体字段赋值。

## 机械替换

迁移前：

```rust
api.post_message(
    &token,
    "channel_id",
    Some("你好！"),
    None, None, None, None, None,
    Some(&message_id),  // msg_id（回复目标）
    None, None, None,
).await?;
```

迁移后：

```rust
let params = MessageParams::new_text("你好！").with_reply(&message_id);
api.post_message_with_params(&token, "channel_id", params).await?;
```

任何原本非 `None` 的位置参数，改为在构建器上设置对应字段：

```rust
let params = MessageParams {
    content: Some("带 embed".into()),
    embed: Some(my_embed),
    msg_id: Some(reply_to.into()),
    ..Default::default()
};
api.post_message_with_params(&token, channel_id, params).await?;
```

群、C2C、私信形态完全相同，只需替换为对应的 `*Params` 与 `*_with_params` 方法。

## 为什么

旧 API 是一个文档化的“枪口”：参数顺序与含义容易记错，错放一个 `Some` / `None` 会产生语法合法但语义错误的调用。新 API 在每个调用点都暴露字段名，`..Default::default()` 即可覆盖常见情况，字段新增或改名时编译器还能帮你定位。

## 弃用时间线

旧方法在 0.2.x 仍可编译，但带有 `#[deprecated]` 警告，0.3.0 将彻底删除。当你的项目能在 `-W deprecated` 下零警告编译，迁移就完成了。
