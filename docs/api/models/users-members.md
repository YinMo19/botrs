# Users and Members

User models cover the shapes that appear in the core runtime path: the bot itself, `READY` user data, reaction user lists, message authors, guild member events, and schedule creators.

## User and BotInfo

`User` is the general user structure. It includes id, username, avatar, bot flag, and union fields. `Ready::user` uses this type.

`BotInfo` is the `/users/@me` response. In addition to basic user fields, it keeps the platform's `share_url` and `welcome_msg`. `session.bot_info()` is populated from this endpoint during startup when a handler is dispatched.

## Member Shapes

There are two member shapes:

- `botrs::models::guild::Member`, also `models::guild::Member`, is used by guild member gateway events. It has `guild_id`, `user`, `nick`, `roles`, `joined_at`, and `op_user_id`.
- `models::user::Member` flattens a `User` and adds nick, roles, joined_at, deaf, and mute. It mainly appears in response fields such as schedule creator.

Do not treat them as interchangeable. The `guild_member_add`, `guild_member_update`, and `guild_member_remove` callbacks receive the first shape.

## Message Authors

Author fields differ by message scene:

- Guild channel messages use `MessageUser`, close to a normal guild user shape.
- Group messages use `GroupMessageUser`, centered on `member_openid` and `union_openid`.
- C2C messages use `C2CMessageUser`, centered on `user_openid`.
- Guild channel member info uses `MessageMember`, which only contains nick, roles, and joined_at.

Direct-message events use `Message`; direct-message sessions use `DirectMessage`.

## Practical Guidance

For normal message events, `message.author` is a required field. Member details and some platform-specific user fields may still be absent or defaulted depending on the event.

```rust
let is_bot = message.author.bot;
```

For group and C2C replies, use the openid fields from the event model.

## See Also

- [Messages](./messages.md)
- [Guilds and Channels](./guilds-channels.md)
- [Other types](./other-types.md)
