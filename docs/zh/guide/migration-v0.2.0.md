# v0.2.0 消息 API 迁移

发送消息接口已经从位置 `Option` 参数迁移到类型化构建器。旧方法以及中间阶段的 `*_with_params` 名称现已移除，请使用下面的短方法。

## 改动一览

| 旧方法                          | 新方法                                      | 构建器                 |
|---------------------------------|---------------------------------------------|------------------------|
| `post_message`                  | `send_message`                  | `MessageParams`        |
| `post_group_message`            | `send_group_message`            | `GroupMessageParams`   |
| `post_c2c_message`              | `send_c2c_message`              | `C2CMessageParams`     |
| `post_dms`                      | `send_direct_message`                      | `DirectMessageParams`  |
| `patch_message`（旧版）         | `edit_message`                 | `MessageParams`        |

所有构建器都提供 `new_text(content)` 与 `with_reply(message_id)`。其余字段（embed、ark、markdown、keyboard、图片 URL、media 等）通过结构体字段赋值。

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
api.send_message("channel_id", params).await?;
```

任何原本非 `None` 的位置参数，改为在构建器上设置对应字段：

```rust
let params = MessageParams {
    content: Some("带 embed".into()),
    embed: Some(my_embed),
    msg_id: Some(reply_to.into()),
    ..Default::default()
};
api.send_message(channel_id, params).await?;
```

群、C2C、私信形态完全相同，只需替换为对应的 `*Params` 与短发送方法。

## 为什么

旧 API 是一个文档化的“枪口”：参数顺序与含义容易记错，错放一个 `Some` / `None` 会产生语法合法但语义错误的调用。新 API 在每个调用点都暴露字段名，`..Default::default()` 即可覆盖常见情况，字段新增或改名时编译器还能帮你定位。

## 移除时间线

旧方法在 0.2.x 仍可编译，但带有 `#[deprecated]` 警告；现在已经彻底移除，后来的 `*_with_params` 兼容名也已移除。项目只调用短发送/编辑方法后，迁移即完成。
