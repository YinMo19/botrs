# Calling Other QQ Open APIs

These examples show how to use `BotApi` for resources beyond plain message posting. Each one is a self-contained event handler that triggers the API call from a chat command.

| Example                                                                                                               | API surface                                                                                                                          |
|-----------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| [`guild/api_permission.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/api_permission.rs)              | `BotApi::get_api_permissions`, `BotApi::post_permission_demand`, `APIPermissionDemandIdentify`                                        |
| [`guild/announce.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/announce.rs)                          | `BotApi::create_announce`, `BotApi::delete_announce`, `BotApi::create_recommend_announce`, `RecommendChannel`, `AnnouncesType`       |
| [`guild/schedule.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/schedule.rs)                          | `BotApi::create_schedule`, `get_schedule`, `update_schedule`, `delete_schedule`, `RemindType`                                        |
| [`guild/pins_message.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/pins_message.rs)                  | `BotApi::get_pins`, `BotApi::put_pin`, `BotApi::delete_pin`                                                                          |
| [`guild/reaction_users.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reaction_users.rs)              | `BotApi::get_reaction_users`, `EmojiType::System`, paginates with the `cookie` field                                                  |
| [`guild/recall.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/recall.rs)                              | `session.recall_message` (also available as `BotApi::recall_message`)                                                                |
| [`api/message_params.rs`](https://github.com/YinMo19/botrs/blob/main/examples/api/message_params.rs)                  | `MessageParams`, `GroupMessageParams`, `C2CMessageParams`, `DirectMessageParams`                                                     |

## Pattern

Every REST call is available directly on `session` because sessions dereference to `BotApi`; the token is stored inside `BotApi`. Errors come back as `botrs::BotError`. Handle them locally — the framework does not wrap calls in retries.

```rust
// examples/guild/recall.rs — send, then immediately delete
let resp = session.reply("this will vanish").await?;
if let Some(message_id) = resp.id {
    session
        .recall_message(session.channel_id(), &message_id, /* hidetip */ true)
        .await?;
}
```

For paginated APIs like `get_reaction_users`, drive the loop yourself with the returned `cookie` and `is_end` fields, exactly as the example does.

## See also

- Guide: [`docs/guide/api-client.md`](../guide/api-client.md)
- Source of truth for available methods: `src/api/mod.rs` and the files under `src/api/`
- Example paths listed above under `examples/`
