# v0.2.0 message API migration

The message-sending surface moved from positional `Option` arguments to typed builders. The legacy methods have since been removed; use the `*_with_params` methods below.

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

## Removal timeline

The old methods compiled in 0.2.x with `#[deprecated]` warnings and have now been removed. Once your project only calls the `*_with_params` methods, the migration is complete.
