# v0.2.0 message API migration

The only breaking change in `botrs` 0.2.0 is the message-sending surface. Five `BotApi` methods used to take 10+ positional `Option` arguments; they're now `*_with_params` methods that take a typed builder. The old methods are kept with `#[deprecated]` and will be removed in 0.3.

## What changed

| Old method                      | New method                                 | Builder                |
|---------------------------------|--------------------------------------------|------------------------|
| `post_message`                  | `post_message_with_params`                 | `MessageParams`        |
| `post_group_message`            | `post_group_message_with_params`           | `GroupMessageParams`   |
| `post_c2c_message`              | `post_c2c_message_with_params`             | `C2CMessageParams`     |
| `post_dms`                      | `post_dms_with_params`                     | `DirectMessageParams`  |
| `patch_message` (legacy)        | `patch_message_with_params`                | `MessageParams`        |

All builders provide `new_text(content)` and `with_reply(message_id)`. `MessageParams` and `DirectMessageParams` additionally have `with_file_image(&bytes)`. Anything else (embed, ark, markdown, keyboard, media, etc.) is set with struct-update syntax.

## Mechanical rewrite

Before:

```rust
api.post_message(
    &token,
    "channel_id",
    Some("Hello!"),
    None, None, None, None, None,
    Some(&message_id),  // msg_id (reply target)
    None, None, None,
).await?;
```

After:

```rust
let params = MessageParams::new_text("Hello!").with_reply(&message_id);
api.post_message_with_params(&token, "channel_id", params).await?;
```

Anywhere you previously had a non-`None` argument, set the matching field on the builder:

```rust
let params = MessageParams {
    content: Some("with embed".into()),
    embed: Some(my_embed),
    msg_id: Some(reply_to.into()),
    ..Default::default()
};
api.post_message_with_params(&token, channel_id, params).await?;
```

Group / C2C / DM follow the same shape with the corresponding `*Params` and `*_with_params` method names.

## Why

The old API was a documented footgun: the order and meaning of the `Option` arguments was easy to misremember, and a misplaced `Some` or `None` produced a syntactically valid but semantically wrong call. The struct version makes the field name explicit at every call site, allows `..Default::default()` for the common case, and lets the compiler help you when fields are added or renamed.

## Deprecation timeline

The old methods compile in 0.2.x with `#[deprecated]` warnings. They'll be removed entirely in 0.3.0 — once your project compiles cleanly with `-W deprecated`, you're done migrating.
