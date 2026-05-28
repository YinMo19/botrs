# Other Types

This page covers common models outside messages, users, guilds, and channels: gateway startup data, schedules, permissions, pins, announcements, reactions, interactions, audio events, manage events, and forum events.

## Gateway

`get_gateway` returns `GatewayResponse`, including the websocket URL, recommended shard count, and session start limit. `Client::start` uses it to create internal gateway sessions; application code normally does not handle it directly.

`Ready` is the `READY` event payload. It contains the session id, version, current bot `User`, and optional shard data.

## Schedules

Schedule models are used by `get_schedules`, `get_schedule`, `create_schedule`, `update_schedule`, and `delete_schedule`. Create or update schedules with `Schedule::new(...)`; reminders use `RemindType`, which converts to the string value expected by the platform.

```rust
let schedule = Schedule::new(
    "Daily sync",
    "1767225600",
    "1767229200",
    Some(channel_id.to_string()),
    RemindType::Before15Minutes,
);
session.create_schedule(&schedule_channel_id, &schedule).await?;
```

## API Permissions

Permission listing uses `get_api_permissions`. Permission requests use `post_permission_demand`, passing the target channel, API identifier, and description.

`post_permission_demand` builds the request body from those fields, so callers only need to describe the target API and the reason.

## Pins and Announcements

Pinned messages are represented by `PinsMessage`, which stores the pinned message ids for a channel. The common flow is pin, unpin, and list.

Announcement models include `Announce`, `RecommendChannel`, and `AnnouncesType`. The common flow is creating an announcement from an existing message, or creating a recommended-channel announcement from a channel list.

## Reactions

Reaction events and REST queries share the same core types:

- `Reaction` is the event object received by handlers.
- `MessageReaction` is the platform reaction payload DTO.
- `ReactionTarget` and `ReactionTargetType` describe the reacted target.
- `ReactionUsers` is the response from `get_reaction_users`.
- Emoji data uses `models::emoji::Emoji` and `EmojiType`.

When listing reaction users, pass the previous response's `cookie` and a `limit` to `get_reaction_users`. Pagination ends when `is_end` is true.

## Interaction

`Interaction` maps to `interaction_create` and is used to read button or application interaction payloads from the platform.

## Audio, Manage, and Forum Events

Audio models are gateway event payloads: `Audio`, `PublicAudio`, and `PublicAudioType`.

Manage events are dispatched as:

- Group: `GroupManageEvent`
- C2C: `C2CManageEvent`
- AIO entry: `EnterAioEvent`
- Subscribe message authorization status: `SubscribeMessageStatusData`

Forum events include private forum events and open forum events. Handlers can read threads, posts, replies, and audit results from their typed payloads.

## See Also

- [BotApi](../bot-api.md)
- [EventHandler](../event-handler.md)
- [Messages](./messages.md)
