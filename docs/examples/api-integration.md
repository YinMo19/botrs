# Calling Other QQ Open APIs

These demos show how to use `BotApi` for resources beyond plain message posting. Each one is a self-contained event handler that triggers the API call from a chat command.

| Demo                                                                                                                  | API surface                                                                                                                          |
|-----------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| [`demo_api_permission.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_api_permission.rs)                | `BotApi::get_api_permissions`, `BotApi::post_permission_demand`, `APIPermissionDemandIdentify::guild_members`                        |
| [`demo_announce.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_announce.rs)                            | `BotApi::create_announce`, `BotApi::delete_announce`, `BotApi::create_recommend_announce`, `RecommendChannel`, `AnnouncesType`       |
| [`demo_schedule.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_schedule.rs)                            | `BotApi::create_schedule`, `get_schedule`, `update_schedule`, `delete_schedule`, `RemindType`                                        |
| [`demo_pins_message.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_pins_message.rs)                    | `BotApi::get_pins`, `BotApi::put_pin`, `BotApi::delete_pin`                                                                          |
| [`demo_get_reaction_users.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_get_reaction_users.rs)        | `BotApi::get_reaction_users`, `EmojiType::System`, paginates with the `cookie` field                                                  |
| [`demo_recall.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_recall.rs)                                | `Context::recall_message` (also available as `BotApi::recall_message`)                                                               |

## Pattern

Every call goes through `ctx.api` (an `Arc<BotApi>`) and takes `&ctx.token` as the first argument. Errors come back as `botrs::BotError`. Handle them locally — the framework does not wrap calls in retries.

```rust
// demo_recall.rs — send, then immediately delete
let resp = message.reply(&ctx.api, &ctx.token, "this will vanish").await?;
if let Some(message_id) = resp.id {
    ctx.recall_message(channel_id, &message_id, /* hidetip */ true).await?;
}
```

For paginated APIs like `get_reaction_users`, drive the loop yourself with the returned `cookie` and `is_end` fields, exactly as the demo does.

## See also

- Guide: [`docs/guide/api-client.md`](../guide/api-client.md)
- Source of truth for available methods: `src/api.rs` (the `BotApi` impl)
- Demo paths listed above under `examples/`
