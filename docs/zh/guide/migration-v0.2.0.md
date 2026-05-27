# v0.2.0 消息 API 迁移

发送消息接口已经从位置 `Option` 参数迁移到类型化构建器。旧方法现已移除，请使用下面的 `*_with_params` 方法。

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

## 移除时间线

旧方法在 0.2.x 仍可编译，但带有 `#[deprecated]` 警告；现在已经彻底移除。项目只调用 `*_with_params` 方法后，迁移即完成。
