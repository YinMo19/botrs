use super::prelude::*;
use std::future::Future;

macro_rules! event_handlers {
    ($( $(#[$doc:meta])* $variant:ident($session:ty) => $method:ident ),+ $(,)?) => {
        /// A decoded event and its event-scoped API session.
        #[non_exhaustive]
        pub enum Event {
            $( $(#[$doc])* $variant($session), )+
        }

        impl Event {
            /// Calls the corresponding individual callback on a handler.
            pub async fn dispatch<H: EventHandler + ?Sized>(self, handler: &H) {
                match self {
                    $(Self::$variant(session) => handler.$method(session).await,)+
                }
            }
        }

        /// Handles events from WebSocket or Webhook delivery.
        #[async_trait::async_trait]
        pub trait EventHandler: Send + Sync {
            /// Override to use `?` in business logic, or implement individual callbacks.
            async fn handle(&self, event: Event) -> Result<()> {
                event.dispatch(self).await;
                Ok(())
            }

            $( $(#[$doc])* async fn $method(&self, _session: $session) {} )+

            /// Reports decoding or handler errors.
            async fn error(&self, error: BotError) {
                error!("Event handler error: {}", error);
            }
        }
    };
}

event_handlers! {
    /// Called when the bot is ready and connected.
    Ready(ReadySession) => ready,
    /// Called when the bot session has resumed.
    Resumed(ResumeSession) => resumed,
    /// Called when a message is created (including @ mentions).
    MessageCreate(ChannelReplySession) => message_create,
    /// Called when a direct message is created.
    DirectMessageCreate(DirectReplySession) => direct_message_create,
    /// Called when a direct message is deleted.
    DirectMessageDelete(MessageDeleteSession) => direct_message_delete,
    /// Called when a group message is created.
    GroupMessageCreate(GroupReplySession) => group_message_create,
    /// Called when a C2C message is created.
    C2CMessageCreate(C2CReplySession) => c2c_message_create,
    /// Called when a message is deleted.
    MessageDelete(MessageDeleteSession) => message_delete,
    /// Called when a public guild message is deleted.
    PublicMessageDelete(MessageDeleteSession) => public_message_delete,
    /// Called when a reaction is added to a message.
    MessageReactionAdd(ReactionSession) => message_reaction_add,
    /// Called when a reaction is removed from a message.
    MessageReactionRemove(ReactionSession) => message_reaction_remove,
    /// Called when an interaction event is created.
    InteractionCreate(InteractionSession) => interaction_create,
    /// Called when audio starts.
    AudioStart(AudioSession) => audio_start,
    /// Called when audio finishes.
    AudioFinish(AudioSession) => audio_finish,
    /// Called when microphone is turned on.
    OnMic(AudioSession) => on_mic,
    /// Called when microphone is turned off.
    OffMic(AudioSession) => off_mic,
    /// Called when a guild is created (bot joins).
    GuildCreate(GuildSession) => guild_create,
    /// Called when a guild is updated.
    GuildUpdate(GuildSession) => guild_update,
    /// Called when a guild is deleted (bot leaves).
    GuildDelete(GuildSession) => guild_delete,
    /// Called when a channel is created.
    ChannelCreate(ChannelSession) => channel_create,
    /// Called when a channel is updated.
    ChannelUpdate(ChannelSession) => channel_update,
    /// Called when a channel is deleted.
    ChannelDelete(ChannelSession) => channel_delete,
    /// Called when a guild member is added.
    GuildMemberAdd(MemberSession) => guild_member_add,
    /// Called when a guild member is updated.
    GuildMemberUpdate(MemberSession) => guild_member_update,
    /// Called when a guild member is removed.
    GuildMemberRemove(MemberSession) => guild_member_remove,
    /// Called when a message audit passes.
    MessageAuditPass(MessageAuditSession) => message_audit_pass,
    /// Called when a message audit is rejected.
    MessageAuditReject(MessageAuditSession) => message_audit_reject,
    /// Called when a friend is added.
    FriendAdd(C2CManageSession) => friend_add,
    /// Called when a friend is deleted.
    FriendDel(C2CManageSession) => friend_del,
    /// Called when C2C message is rejected.
    C2CMsgReject(C2CManageSession) => c2c_msg_reject,
    /// Called when C2C message is received.
    C2CMsgReceive(C2CManageSession) => c2c_msg_receive,
    /// Called when subscribe message authorization status changes.
    SubscribeMessageStatus(SubscribeMessageStatusSession) => subscribe_message_status,
    /// Called when a user enters AIO.
    EnterAio(EnterAioSession) => enter_aio,
    /// Called when robot is added to group.
    GroupAddRobot(GroupManageSession) => group_add_robot,
    /// Called when robot is deleted from group.
    GroupDelRobot(GroupManageSession) => group_del_robot,
    /// Called when group message is rejected.
    GroupMsgReject(GroupManageSession) => group_msg_reject,
    /// Called when group message is received.
    GroupMsgReceive(GroupManageSession) => group_msg_receive,
    /// Called when a user enters an audio or live channel.
    AudioOrLiveChannelMemberEnter(PublicAudioSession) => audio_or_live_channel_member_enter,
    /// Called when a user exits an audio or live channel.
    AudioOrLiveChannelMemberExit(PublicAudioSession) => audio_or_live_channel_member_exit,
    /// Called when a forum thread is created.
    ForumThreadCreate(ThreadSession) => forum_thread_create,
    /// Called when a forum thread is updated.
    ForumThreadUpdate(ThreadSession) => forum_thread_update,
    /// Called when a forum thread is deleted.
    ForumThreadDelete(ThreadSession) => forum_thread_delete,
    /// Called when a forum post is created.
    ForumPostCreate(PostSession) => forum_post_create,
    /// Called when a forum post is deleted.
    ForumPostDelete(PostSession) => forum_post_delete,
    /// Called when a forum reply is created.
    ForumReplyCreate(ForumReplySession) => forum_reply_create,
    /// Called when a forum reply is deleted.
    ForumReplyDelete(ForumReplySession) => forum_reply_delete,
    /// Called when a forum publish audit result arrives.
    ForumPublishAuditResult(ForumAuditSession) => forum_publish_audit_result,
    /// Called when an open forum thread is created.
    OpenForumThreadCreate(OpenForumSession) => open_forum_thread_create,
    /// Called when an open forum thread is updated.
    OpenForumThreadUpdate(OpenForumSession) => open_forum_thread_update,
    /// Called when an open forum thread is deleted.
    OpenForumThreadDelete(OpenForumSession) => open_forum_thread_delete,
    /// Called when an open forum post is created.
    OpenForumPostCreate(OpenForumSession) => open_forum_post_create,
    /// Called when an open forum post is deleted.
    OpenForumPostDelete(OpenForumSession) => open_forum_post_delete,
    /// Called when an open forum reply is created.
    OpenForumReplyCreate(OpenForumSession) => open_forum_reply_create,
    /// Called when an open forum reply is deleted.
    OpenForumReplyDelete(OpenForumSession) => open_forum_reply_delete,
    /// Called for any unhandled events.
    UnknownEvent(UnknownEventSession) => unknown_event,
}

#[async_trait::async_trait]
impl<F, Fut> EventHandler for F
where
    F: Fn(Event) -> Fut + Send + Sync,
    Fut: Future<Output = Result<()>> + Send,
{
    async fn handle(&self, event: Event) -> Result<()> {
        self(event).await
    }
}
