use super::*;
use crate::intents::Intent;
use crate::models::gateway::*;

pub trait RegisterableHandler {
    fn register(self) -> Intent;
}

pub fn register_handlers<H: RegisterableHandler>(handlers: impl IntoIterator<Item = H>) -> Intent {
    handlers
        .into_iter()
        .fold(0, |intent, handler| intent | handler.register())
}

macro_rules! registerable {
    ($ty:ident, $field:ident, [$($event:expr),+]) => {
        impl RegisterableHandler for $ty {
            fn register(self) -> Intent {
                DEFAULT_HANDLERS
                    .write()
                    .expect("default handlers lock poisoned")
                    .$field = Some(self);
                crate::models::gateway::event_to_intent([$($event),+])
            }
        }
    };
}

impl RegisterableHandler for ReadyHandler {
    fn register(self) -> Intent {
        DEFAULT_HANDLERS
            .write()
            .expect("default handlers lock poisoned")
            .ready = Some(self);
        0
    }
}

impl RegisterableHandler for ErrorNotifyHandler {
    fn register(self) -> Intent {
        DEFAULT_HANDLERS
            .write()
            .expect("default handlers lock poisoned")
            .error_notify = Some(self);
        0
    }
}

impl RegisterableHandler for PlainEventHandler {
    fn register(self) -> Intent {
        DEFAULT_HANDLERS
            .write()
            .expect("default handlers lock poisoned")
            .plain = Some(self);
        0
    }
}

registerable!(
    GuildEventHandler,
    guild,
    [EVENT_GUILD_CREATE, EVENT_GUILD_DELETE, EVENT_GUILD_UPDATE]
);
registerable!(
    GuildMemberEventHandler,
    guild_member,
    [
        EVENT_GUILD_MEMBER_ADD,
        EVENT_GUILD_MEMBER_REMOVE,
        EVENT_GUILD_MEMBER_UPDATE
    ]
);
registerable!(
    ChannelEventHandler,
    channel,
    [
        EVENT_CHANNEL_CREATE,
        EVENT_CHANNEL_DELETE,
        EVENT_CHANNEL_UPDATE
    ]
);
registerable!(MessageEventHandler, message, [EVENT_MESSAGE_CREATE]);
registerable!(ATMessageEventHandler, at_message, [EVENT_AT_MESSAGE_CREATE]);
registerable!(
    DirectMessageEventHandler,
    direct_message,
    [EVENT_DIRECT_MESSAGE_CREATE]
);
registerable!(
    MessageDeleteEventHandler,
    message_delete,
    [EVENT_MESSAGE_DELETE]
);
registerable!(
    PublicMessageDeleteEventHandler,
    public_message_delete,
    [EVENT_PUBLIC_MESSAGE_DELETE]
);
registerable!(
    DirectMessageDeleteEventHandler,
    direct_message_delete,
    [EVENT_DIRECT_MESSAGE_DELETE]
);
registerable!(
    MessageReactionEventHandler,
    message_reaction,
    [EVENT_MESSAGE_REACTION_ADD, EVENT_MESSAGE_REACTION_REMOVE]
);
registerable!(
    MessageAuditEventHandler,
    message_audit,
    [EVENT_MESSAGE_AUDIT_PASS, EVENT_MESSAGE_AUDIT_REJECT]
);
registerable!(
    AudioEventHandler,
    audio,
    [
        EVENT_AUDIO_START,
        EVENT_AUDIO_FINISH,
        EVENT_AUDIO_ON_MIC,
        EVENT_AUDIO_OFF_MIC
    ]
);
registerable!(
    ThreadEventHandler,
    thread,
    [
        EVENT_FORUM_THREAD_CREATE,
        EVENT_FORUM_THREAD_UPDATE,
        EVENT_FORUM_THREAD_DELETE
    ]
);
registerable!(
    PostEventHandler,
    post,
    [EVENT_FORUM_POST_CREATE, EVENT_FORUM_POST_DELETE]
);
registerable!(
    ReplyEventHandler,
    reply,
    [EVENT_FORUM_REPLY_CREATE, EVENT_FORUM_REPLY_DELETE]
);
registerable!(
    ForumAuditEventHandler,
    forum_audit,
    [EVENT_FORUM_AUDIT_RESULT]
);
registerable!(
    InteractionEventHandler,
    interaction,
    [EVENT_INTERACTION_CREATE]
);
registerable!(
    GroupATMessageEventHandler,
    group_at_message,
    [EVENT_GROUP_AT_MESSAGE_CREATE]
);
registerable!(
    C2CMessageEventHandler,
    c2c_message,
    [EVENT_C2C_MESSAGE_CREATE]
);
registerable!(
    SubscribeMsgStatusEventHandler,
    subscribe_msg_status,
    [EVENT_SUBSCRIBE_MESSAGE_STATUS]
);
registerable!(C2CFriendEventHandler, c2c_friend, [EVENT_C2C_FRIEND_ADD]);
registerable!(EnterAIOEventHandler, enter_aio, [EVENT_ENTER_AIO]);
