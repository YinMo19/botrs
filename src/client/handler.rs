use super::Context;
use super::prelude::*;

/// Event handler trait for processing gateway events.
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, _ready: Ready) {}

    /// Called when the bot session has resumed.
    async fn resumed(&self, _ctx: Context) {}

    /// Called when a message is created (@ mentions).
    async fn message_create(&self, _ctx: Context, _message: Message) {}

    /// Called when a direct message is created.
    async fn direct_message_create(&self, _ctx: Context, _message: Message) {}

    /// Called when a direct message is deleted.
    async fn direct_message_delete(&self, _ctx: Context, _message: MessageDelete) {}

    /// Called when a group message is created.
    async fn group_message_create(&self, _ctx: Context, _message: GroupMessage) {}

    /// Called when a C2C message is created.
    async fn c2c_message_create(&self, _ctx: Context, _message: C2CMessage) {}

    /// Called when a message is deleted.
    async fn message_delete(&self, _ctx: Context, _message: MessageDelete) {}

    /// Called when a reaction is added to a message.
    async fn message_reaction_add(&self, _ctx: Context, _reaction: Reaction) {}

    /// Called when a reaction is removed from a message.
    async fn message_reaction_remove(&self, _ctx: Context, _reaction: Reaction) {}

    /// Called when an interaction event is created.
    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {}

    /// Called when audio starts.
    async fn audio_start(&self, _ctx: Context, _audio: Audio) {}

    /// Called when audio finishes.
    async fn audio_finish(&self, _ctx: Context, _audio: Audio) {}

    /// Called when microphone is turned on.
    async fn on_mic(&self, _ctx: Context, _audio: Audio) {}

    /// Called when microphone is turned off.
    async fn off_mic(&self, _ctx: Context, _audio: Audio) {}

    /// Called when a guild is created (bot joins).
    async fn guild_create(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a guild is updated.
    async fn guild_update(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a guild is deleted (bot leaves).
    async fn guild_delete(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a channel is created.
    async fn channel_create(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is updated.
    async fn channel_update(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is deleted.
    async fn channel_delete(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a guild member is added.
    async fn guild_member_add(&self, _ctx: Context, _member: Member) {}

    /// Called when a guild member is updated.
    async fn guild_member_update(&self, _ctx: Context, _member: Member) {}

    /// Called when a guild member is removed.
    async fn guild_member_remove(&self, _ctx: Context, _member: Member) {}

    /// Called when a message audit passes.
    async fn message_audit_pass(&self, _ctx: Context, _audit: MessageAudit) {}

    /// Called when a message audit is rejected.
    async fn message_audit_reject(&self, _ctx: Context, _audit: MessageAudit) {}

    /// Called when a friend is added.
    async fn friend_add(&self, _ctx: Context, _event: C2CManageEvent) {}

    /// Called when a friend is deleted.
    async fn friend_del(&self, _ctx: Context, _event: C2CManageEvent) {}

    /// Called when C2C message is rejected.
    async fn c2c_msg_reject(&self, _ctx: Context, _event: C2CManageEvent) {}

    /// Called when C2C message is received.
    async fn c2c_msg_receive(&self, _ctx: Context, _event: C2CManageEvent) {}

    /// Called when subscribe message authorization status changes.
    async fn subscribe_message_status(&self, _ctx: Context, _event: SubscribeMessageStatusData) {}

    /// Called when a user enters AIO.
    async fn enter_aio(&self, _ctx: Context, _event: EnterAioEvent) {}

    /// Called when robot is added to group.
    async fn group_add_robot(&self, _ctx: Context, _event: GroupManageEvent) {}

    /// Called when robot is deleted from group.
    async fn group_del_robot(&self, _ctx: Context, _event: GroupManageEvent) {}

    /// Called when group message is rejected.
    async fn group_msg_reject(&self, _ctx: Context, _event: GroupManageEvent) {}

    /// Called when group message is received.
    async fn group_msg_receive(&self, _ctx: Context, _event: GroupManageEvent) {}

    /// Called when a user enters an audio or live channel.
    async fn audio_or_live_channel_member_enter(&self, _ctx: Context, _audio: PublicAudio) {}

    /// Called when a user exits an audio or live channel.
    async fn audio_or_live_channel_member_exit(&self, _ctx: Context, _audio: PublicAudio) {}

    /// Called when a forum thread is created.
    async fn forum_thread_create(&self, _ctx: Context, _thread: Thread) {}

    /// Called when a forum thread is updated.
    async fn forum_thread_update(&self, _ctx: Context, _thread: Thread) {}

    /// Called when a forum thread is deleted.
    async fn forum_thread_delete(&self, _ctx: Context, _thread: Thread) {}

    /// Called when a forum post is created.
    async fn forum_post_create(&self, _ctx: Context, _post: Post) {}

    /// Called when a forum post is deleted.
    async fn forum_post_delete(&self, _ctx: Context, _post: Post) {}

    /// Called when a forum reply is created.
    async fn forum_reply_create(&self, _ctx: Context, _reply: Reply) {}

    /// Called when a forum reply is deleted.
    async fn forum_reply_delete(&self, _ctx: Context, _reply: Reply) {}

    /// Called when a forum publish audit result arrives.
    async fn forum_publish_audit_result(&self, _ctx: Context, _result: ForumAuditResult) {}

    /// Called when an open forum thread is created.
    async fn open_forum_thread_create(&self, _ctx: Context, _thread: OpenThread) {}

    /// Called when an open forum thread is updated.
    async fn open_forum_thread_update(&self, _ctx: Context, _thread: OpenThread) {}

    /// Called when an open forum thread is deleted.
    async fn open_forum_thread_delete(&self, _ctx: Context, _thread: OpenThread) {}

    /// Called when an open forum post is created.
    async fn open_forum_post_create(&self, _ctx: Context, _thread: OpenThread) {}

    /// Called when an open forum post is deleted.
    async fn open_forum_post_delete(&self, _ctx: Context, _thread: OpenThread) {}

    /// Called when an open forum reply is created.
    async fn open_forum_reply_create(&self, _ctx: Context, _thread: OpenThread) {}

    /// Called when an open forum reply is deleted.
    async fn open_forum_reply_delete(&self, _ctx: Context, _thread: OpenThread) {}

    /// Called for any unhandled events.
    async fn unknown_event(&self, _ctx: Context, _event: GatewayEvent) {}

    /// Called when an error occurs during event processing.
    async fn error(&self, _error: BotError) {
        error!("Event handler error: {}", _error);
    }
}
