# v0.2.0 message API migration

The message-sending surface moved from positional `Option` arguments to typed builders. The legacy methods and the intermediate `*_with_params` names have since been removed; use the short send methods below.

## What changed

| Old method                      | New method                                 | Builder                |
|---------------------------------|--------------------------------------------|------------------------|
| `post_message`                  | `send_message`                 | `MessageParams`        |
| `post_group_message`            | `send_group_message`           | `GroupMessageParams`   |
| `post_c2c_message`              | `send_c2c_message`             | `C2CMessageParams`     |
| `post_dms`                      | `send_direct_message`                     | `DirectMessageParams`  |
| `patch_message` (legacy)        | `edit_message`                | `MessageParams`        |

All builders provide `new_text(content)` and `with_reply(message_id)`. Anything else (embed, ark, markdown, keyboard, image URL, media, etc.) is set with struct-update syntax.

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
api.send_message("channel_id", params).await?;
```

Anywhere you previously had a non-`None` argument, set the matching field on the builder:

```rust
let params = MessageParams {
    content: Some("with embed".into()),
    embed: Some(my_embed),
    msg_id: Some(reply_to.into()),
    ..Default::default()
};
api.send_message(channel_id, params).await?;
```

Group / C2C / DM follow the same shape with the corresponding `*Params` and short send method names.

## Why

The old API was a documented footgun: the order and meaning of the `Option` arguments was easy to misremember, and a misplaced `Some` or `None` produced a syntactically valid but semantically wrong call. The struct version makes the field name explicit at every call site, allows `..Default::default()` for the common case, and lets the compiler help you when fields are added or renamed.

## Removal timeline

The old methods compiled in 0.2.x with `#[deprecated]` warnings and have now been removed, along with the later `*_with_params` compatibility names. Once your project only calls the short send/edit methods, the migration is complete.
