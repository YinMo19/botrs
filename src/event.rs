//! Botgo-style event registration and dispatch helpers.

#![allow(non_snake_case, non_upper_case_globals)]

use std::sync::{LazyLock, RwLock};

use serde::de::DeserializeOwned;

use crate::api::BotApi;
use crate::http::HttpClient;
use crate::intents::Intent;
use crate::models::gateway::*;

pub type EventParseFunc = fn(&mut WSPayload, &[u8]) -> crate::Result<()>;
pub type ReadyHandler = fn(&mut WSPayload, &mut WSReadyData);
pub type ErrorNotifyHandler = fn(crate::error::Err);
pub type PlainEventHandler = fn(&mut WSPayload, &[u8]) -> crate::Result<()>;

macro_rules! handler_type {
    ($name:ident, $data:ty) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub fn(&mut WSPayload, &mut $data) -> crate::Result<()>);
    };
}

handler_type!(GuildEventHandler, WSGuildData);
handler_type!(GuildMemberEventHandler, WSGuildMemberData);
handler_type!(ChannelEventHandler, WSChannelData);
handler_type!(MessageEventHandler, WSMessageData);
handler_type!(MessageDeleteEventHandler, WSMessageDeleteData);
handler_type!(PublicMessageDeleteEventHandler, WSPublicMessageDeleteData);
handler_type!(DirectMessageDeleteEventHandler, WSDirectMessageDeleteData);
handler_type!(MessageReactionEventHandler, WSMessageReactionData);
handler_type!(ATMessageEventHandler, WSATMessageData);
handler_type!(DirectMessageEventHandler, WSDirectMessageData);
handler_type!(AudioEventHandler, WSAudioData);
handler_type!(MessageAuditEventHandler, WSMessageAuditData);
handler_type!(ThreadEventHandler, WSThreadData);
handler_type!(PostEventHandler, WSPostData);
handler_type!(ReplyEventHandler, WSReplyData);
handler_type!(ForumAuditEventHandler, WSForumAuditData);
handler_type!(InteractionEventHandler, WSInteractionData);
handler_type!(GroupATMessageEventHandler, WSGroupATMessageData);
handler_type!(C2CMessageEventHandler, WSC2CMessageData);
handler_type!(C2CFriendEventHandler, WSC2CFriendData);
handler_type!(SubscribeMsgStatusEventHandler, WSSubscribeMsgStatus);
handler_type!(EnterAIOEventHandler, WSEnterAIOData);

#[derive(Default, Clone)]
pub struct HandlerRegistry {
    pub ready: Option<ReadyHandler>,
    pub error_notify: Option<ErrorNotifyHandler>,
    pub plain: Option<PlainEventHandler>,
    pub guild: Option<GuildEventHandler>,
    pub guild_member: Option<GuildMemberEventHandler>,
    pub channel: Option<ChannelEventHandler>,
    pub message: Option<MessageEventHandler>,
    pub message_reaction: Option<MessageReactionEventHandler>,
    pub at_message: Option<ATMessageEventHandler>,
    pub direct_message: Option<DirectMessageEventHandler>,
    pub message_audit: Option<MessageAuditEventHandler>,
    pub message_delete: Option<MessageDeleteEventHandler>,
    pub public_message_delete: Option<PublicMessageDeleteEventHandler>,
    pub direct_message_delete: Option<DirectMessageDeleteEventHandler>,
    pub audio: Option<AudioEventHandler>,
    pub thread: Option<ThreadEventHandler>,
    pub post: Option<PostEventHandler>,
    pub reply: Option<ReplyEventHandler>,
    pub forum_audit: Option<ForumAuditEventHandler>,
    pub interaction: Option<InteractionEventHandler>,
    pub group_at_message: Option<GroupATMessageEventHandler>,
    pub c2c_message: Option<C2CMessageEventHandler>,
    pub subscribe_msg_status: Option<SubscribeMsgStatusEventHandler>,
    pub c2c_friend: Option<C2CFriendEventHandler>,
    pub enter_aio: Option<EnterAIOEventHandler>,
}

pub static DefaultHandlers: LazyLock<RwLock<HandlerRegistry>> =
    LazyLock::new(|| RwLock::new(HandlerRegistry::default()));

static EVENT_PARSE_FUNC_MAP: LazyLock<RwLock<Vec<(OpCode, EventType, EventParseFunc)>>> =
    LazyLock::new(|| RwLock::new(default_event_handlers()));

fn default_event_handlers() -> Vec<(OpCode, EventType, EventParseFunc)> {
    [
        (EventGuildCreate, guild_handler as EventParseFunc),
        (EventGuildUpdate, guild_handler),
        (EventGuildDelete, guild_handler),
        (EventChannelCreate, channel_handler),
        (EventChannelUpdate, channel_handler),
        (EventChannelDelete, channel_handler),
        (EventGuildMemberAdd, guild_member_handler),
        (EventGuildMemberUpdate, guild_member_handler),
        (EventGuildMemberRemove, guild_member_handler),
        (EventMessageCreate, message_handler),
        (EventMessageDelete, message_delete_handler),
        (EventMessageReactionAdd, message_reaction_handler),
        (EventMessageReactionRemove, message_reaction_handler),
        (EventAtMessageCreate, at_message_handler),
        (EventPublicMessageDelete, public_message_delete_handler),
        (EventDirectMessageCreate, direct_message_handler),
        (EventDirectMessageDelete, direct_message_delete_handler),
        (EventAudioStart, audio_handler),
        (EventAudioFinish, audio_handler),
        (EventAudioOnMic, audio_handler),
        (EventAudioOffMic, audio_handler),
        (EventMessageAuditPass, message_audit_handler),
        (EventMessageAuditReject, message_audit_handler),
        (EventForumThreadCreate, thread_handler),
        (EventForumThreadUpdate, thread_handler),
        (EventForumThreadDelete, thread_handler),
        (EventForumPostCreate, post_handler),
        (EventForumPostDelete, post_handler),
        (EventForumReplyCreate, reply_handler),
        (EventForumReplyDelete, reply_handler),
        (EventForumAuditResult, forum_audit_handler),
        (EventInteractionCreate, interaction_handler),
        (EventGroupAtMessageCreate, group_at_message_handler),
        (EventC2CMessageCreate, c2c_message_handler),
        (EventSubscribeMsgStatus, subscribe_msg_status_handler),
        (EventC2CFriendAdd, c2c_friend_handler),
        (EventC2CFriendDel, c2c_friend_handler),
        (EventEnterAIO, enter_aio_handler),
    ]
    .into_iter()
    .map(|(event, handler)| (WSDispatchEvent, event.to_string(), handler))
    .collect()
}

pub fn RegisterHandler(op_code: OpCode, event_type: impl Into<EventType>, handler: EventParseFunc) {
    let event_type = event_type.into();
    let mut handlers = EVENT_PARSE_FUNC_MAP
        .write()
        .expect("event handler map lock poisoned");
    if let Some((_, _, existing)) = handlers
        .iter_mut()
        .find(|(op, event, _)| *op == op_code && *event == event_type)
    {
        *existing = handler;
    } else {
        handlers.push((op_code, event_type, handler));
    }
}

pub fn ParseAndHandle(payload: &mut WSPayload) -> crate::Result<()> {
    let raw = payload
        .raw_message
        .clone()
        .unwrap_or_else(|| serde_json::to_vec(payload).unwrap_or_default());
    let handler = {
        let handlers = EVENT_PARSE_FUNC_MAP
            .read()
            .expect("event handler map lock poisoned");
        payload.base.event_type.as_ref().and_then(|event_type| {
            handlers
                .iter()
                .find(|(op, event, _)| *op == payload.base.op_code && event.as_str() == event_type)
                .map(|(_, _, handler)| *handler)
        })
    };

    if let Some(handler) = handler {
        handler(payload, &raw)
    } else if let Some(plain) = DefaultHandlers
        .read()
        .expect("default handlers lock poisoned")
        .plain
    {
        plain(payload, &raw)
    } else {
        Ok(())
    }
}

pub fn ParseData<T: DeserializeOwned>(message: &[u8]) -> crate::Result<T> {
    let value: serde_json::Value = serde_json::from_slice(message)?;
    serde_json::from_value(value.get("d").cloned().unwrap_or(serde_json::Value::Null))
        .map_err(Into::into)
}

pub trait PayloadData: Sized {
    fn parse_from_payload(payload: &WSPayload, message: &[u8]) -> crate::Result<Self>;
}

impl<T> PayloadData for T
where
    T: DeserializeOwned,
{
    fn parse_from_payload(_payload: &WSPayload, message: &[u8]) -> crate::Result<Self> {
        ParseData(message)
    }
}

fn payload_data(message: &[u8]) -> crate::Result<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_slice(message)?;
    Ok(value.get("d").cloned().unwrap_or(serde_json::Value::Null))
}

fn payload_event_id(payload: &WSPayload) -> Option<String> {
    payload.base.event_id.clone()
}

fn event_api() -> BotApi {
    BotApi::new(HttpClient::new(crate::DEFAULT_TIMEOUT, false).expect("valid default api client"))
}

macro_rules! impl_constructed_payload_data {
    ($ty:ty, $ctor:expr) => {
        impl PayloadData for $ty {
            fn parse_from_payload(payload: &WSPayload, message: &[u8]) -> crate::Result<Self> {
                let data = payload_data(message)?;
                Ok($ctor(payload, data))
            }
        }
    };
}

impl PayloadData for crate::reaction::Reaction {
    fn parse_from_payload(payload: &WSPayload, message: &[u8]) -> crate::Result<Self> {
        let data = ParseData(message)?;
        Ok(crate::reaction::Reaction::from_message_reaction(
            event_api(),
            payload_event_id(payload),
            data,
        ))
    }
}
impl_constructed_payload_data!(
    crate::forum::Thread,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Thread::new(event_api(), payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::forum::Post,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Post::new(event_api(), payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::forum::Reply,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Reply::new(event_api(), payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::interaction::Interaction,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::interaction::Interaction::new(event_api(), payload_event_id(payload), &data)
    }
);
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

macro_rules! typed_handler {
    ($fn_name:ident, $field:ident, $data:ty) => {
        fn $fn_name(payload: &mut WSPayload, message: &[u8]) -> crate::Result<()> {
            let mut data: $data = <$data as PayloadData>::parse_from_payload(payload, message)?;
            if let Some(handler) = DefaultHandlers
                .read()
                .expect("default handlers lock poisoned")
                .$field
            {
                handler.0(payload, &mut data)?;
            }
            Ok(())
        }
    };
}

typed_handler!(guild_handler, guild, WSGuildData);
typed_handler!(guild_member_handler, guild_member, WSGuildMemberData);
typed_handler!(channel_handler, channel, WSChannelData);
typed_handler!(message_handler, message, WSMessageData);
typed_handler!(message_delete_handler, message_delete, WSMessageDeleteData);
typed_handler!(
    message_reaction_handler,
    message_reaction,
    WSMessageReactionData
);
typed_handler!(at_message_handler, at_message, WSATMessageData);
typed_handler!(
    public_message_delete_handler,
    public_message_delete,
    WSPublicMessageDeleteData
);
typed_handler!(direct_message_handler, direct_message, WSDirectMessageData);
typed_handler!(
    direct_message_delete_handler,
    direct_message_delete,
    WSDirectMessageDeleteData
);
typed_handler!(audio_handler, audio, WSAudioData);
typed_handler!(thread_handler, thread, WSThreadData);
typed_handler!(post_handler, post, WSPostData);
typed_handler!(reply_handler, reply, WSReplyData);
typed_handler!(forum_audit_handler, forum_audit, WSForumAuditData);
typed_handler!(message_audit_handler, message_audit, WSMessageAuditData);
typed_handler!(interaction_handler, interaction, WSInteractionData);
typed_handler!(
    group_at_message_handler,
    group_at_message,
    WSGroupATMessageData
);
typed_handler!(c2c_message_handler, c2c_message, WSC2CMessageData);
typed_handler!(
    subscribe_msg_status_handler,
    subscribe_msg_status,
    WSSubscribeMsgStatus
);
typed_handler!(c2c_friend_handler, c2c_friend, WSC2CFriendData);
typed_handler!(enter_aio_handler, enter_aio, WSEnterAIOData);

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static MESSAGE_COUNT: AtomicUsize = AtomicUsize::new(0);

    fn message_handler(_: &mut WSPayload, _: &mut WSMessageData) -> crate::Result<()> {
        MESSAGE_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    #[test]
    fn register_handlers_returns_intents() {
        let intent = RegisterHandlers([MessageEventHandler(message_handler)]);
        assert_eq!(
            intent & crate::intents::IntentGuildMessages,
            crate::intents::IntentGuildMessages
        );
    }

    #[test]
    fn parse_and_handle_dispatches_typed_handler() {
        RegisterHandlers([MessageEventHandler(message_handler)]);
        let body = br#"{"op":0,"t":"MESSAGE_CREATE","d":{"id":"1","content":"hello"}}"#;
        let mut payload = WSPayload {
            base: WSPayloadBase {
                op_code: WSDispatchEvent,
                seq: None,
                event_type: Some(EventMessageCreate.to_string()),
                event_id: None,
            },
            data: None,
            raw_message: Some(body.to_vec()),
            session: None,
        };

        ParseAndHandle(&mut payload).unwrap();
        assert!(MESSAGE_COUNT.load(Ordering::Relaxed) > 0);
    }

    #[test]
    fn parse_data_reads_c2c_friend_dto_like_botgo() {
        let body = br#"{"op":0,"t":"FRIEND_ADD","d":{"openid":"u1","timestamp":123,"nick":"n","avatar":"a"}}"#;
        let data: WSC2CFriendData = ParseData(body).unwrap();

        assert_eq!(data.openid, "u1");
        assert_eq!(data.timestamp, 123);
        assert_eq!(data.nick, "n");
        assert_eq!(data.avatar, "a");
    }
}
