use super::*;
use crate::intents::Intent;
use crate::models::gateway::*;

pub trait RegisterableHandler {
    fn register(self) -> Intent;
}

pub fn RegisterHandlers<H: RegisterableHandler>(handlers: impl IntoIterator<Item = H>) -> Intent {
    handlers
        .into_iter()
        .fold(crate::intents::IntentNone, |intent, handler| {
            intent | handler.register()
        })
}

macro_rules! registerable {
    ($ty:ident, $field:ident, [$($event:expr),+]) => {
        impl RegisterableHandler for $ty {
            fn register(self) -> Intent {
                DefaultHandlers
                    .write()
                    .expect("default handlers lock poisoned")
                    .$field = Some(self);
                crate::models::gateway::EventToIntent([$($event),+])
            }
        }
    };
}

impl RegisterableHandler for ReadyHandler {
    fn register(self) -> Intent {
        DefaultHandlers
            .write()
            .expect("default handlers lock poisoned")
            .ready = Some(self);
        crate::intents::IntentNone
    }
}

impl RegisterableHandler for ErrorNotifyHandler {
    fn register(self) -> Intent {
        DefaultHandlers
            .write()
            .expect("default handlers lock poisoned")
            .error_notify = Some(self);
        crate::intents::IntentNone
    }
}

impl RegisterableHandler for PlainEventHandler {
    fn register(self) -> Intent {
        DefaultHandlers
            .write()
            .expect("default handlers lock poisoned")
            .plain = Some(self);
        crate::intents::IntentNone
    }
}

registerable!(
    GuildEventHandler,
    guild,
    [EventGuildCreate, EventGuildDelete, EventGuildUpdate]
);
registerable!(
    GuildMemberEventHandler,
    guild_member,
    [
        EventGuildMemberAdd,
        EventGuildMemberRemove,
        EventGuildMemberUpdate
    ]
);
registerable!(
    ChannelEventHandler,
    channel,
    [EventChannelCreate, EventChannelDelete, EventChannelUpdate]
);
registerable!(MessageEventHandler, message, [EventMessageCreate]);
registerable!(ATMessageEventHandler, at_message, [EventAtMessageCreate]);
registerable!(
    DirectMessageEventHandler,
    direct_message,
    [EventDirectMessageCreate]
);
registerable!(
    MessageDeleteEventHandler,
    message_delete,
    [EventMessageDelete]
);
registerable!(
    PublicMessageDeleteEventHandler,
    public_message_delete,
    [EventPublicMessageDelete]
);
registerable!(
    DirectMessageDeleteEventHandler,
    direct_message_delete,
    [EventDirectMessageDelete]
);
registerable!(
    MessageReactionEventHandler,
    message_reaction,
    [EventMessageReactionAdd, EventMessageReactionRemove]
);
registerable!(
    MessageAuditEventHandler,
    message_audit,
    [EventMessageAuditPass, EventMessageAuditReject]
);
registerable!(
    AudioEventHandler,
    audio,
    [
        EventAudioStart,
        EventAudioFinish,
        EventAudioOnMic,
        EventAudioOffMic
    ]
);
registerable!(
    ThreadEventHandler,
    thread,
    [
        EventForumThreadCreate,
        EventForumThreadUpdate,
        EventForumThreadDelete
    ]
);
registerable!(
    PostEventHandler,
    post,
    [EventForumPostCreate, EventForumPostDelete]
);
registerable!(
    ReplyEventHandler,
    reply,
    [EventForumReplyCreate, EventForumReplyDelete]
);
registerable!(ForumAuditEventHandler, forum_audit, [EventForumAuditResult]);
registerable!(
    InteractionEventHandler,
    interaction,
    [EventInteractionCreate]
);
registerable!(
    GroupATMessageEventHandler,
    group_at_message,
    [EventGroupAtMessageCreate]
);
registerable!(C2CMessageEventHandler, c2c_message, [EventC2CMessageCreate]);
registerable!(
    SubscribeMsgStatusEventHandler,
    subscribe_msg_status,
    [EventSubscribeMsgStatus]
);
registerable!(C2CFriendEventHandler, c2c_friend, [EventC2CFriendAdd]);
registerable!(EnterAIOEventHandler, enter_aio, [EventEnterAIO]);
