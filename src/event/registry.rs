use super::handlers::*;
use crate::models::gateway::*;
use std::sync::{LazyLock, RwLock};

type EventParseFn = fn(&mut WSPayload, &[u8]) -> crate::Result<()>;

static EVENT_PARSE_HANDLERS: LazyLock<RwLock<Vec<(OpCode, EventType, EventParseFn)>>> =
    LazyLock::new(|| RwLock::new(default_event_handlers()));

fn default_event_handlers() -> Vec<(OpCode, EventType, EventParseFn)> {
    [
        (EVENT_GUILD_CREATE, guild_handler as EventParseFn),
        (EVENT_GUILD_UPDATE, guild_handler),
        (EVENT_GUILD_DELETE, guild_handler),
        (EVENT_CHANNEL_CREATE, channel_handler),
        (EVENT_CHANNEL_UPDATE, channel_handler),
        (EVENT_CHANNEL_DELETE, channel_handler),
        (EVENT_GUILD_MEMBER_ADD, guild_member_handler),
        (EVENT_GUILD_MEMBER_UPDATE, guild_member_handler),
        (EVENT_GUILD_MEMBER_REMOVE, guild_member_handler),
        (EVENT_MESSAGE_CREATE, message_handler),
        (EVENT_MESSAGE_DELETE, message_delete_handler),
        (EVENT_MESSAGE_REACTION_ADD, message_reaction_handler),
        (EVENT_MESSAGE_REACTION_REMOVE, message_reaction_handler),
        (EVENT_AT_MESSAGE_CREATE, at_message_handler),
        (EVENT_PUBLIC_MESSAGE_DELETE, public_message_delete_handler),
        (EVENT_DIRECT_MESSAGE_CREATE, direct_message_handler),
        (EVENT_DIRECT_MESSAGE_DELETE, direct_message_delete_handler),
        (EVENT_AUDIO_START, audio_handler),
        (EVENT_AUDIO_FINISH, audio_handler),
        (EVENT_AUDIO_ON_MIC, audio_handler),
        (EVENT_AUDIO_OFF_MIC, audio_handler),
        (EVENT_MESSAGE_AUDIT_PASS, message_audit_handler),
        (EVENT_MESSAGE_AUDIT_REJECT, message_audit_handler),
        (EVENT_FORUM_THREAD_CREATE, thread_handler),
        (EVENT_FORUM_THREAD_UPDATE, thread_handler),
        (EVENT_FORUM_THREAD_DELETE, thread_handler),
        (EVENT_FORUM_POST_CREATE, post_handler),
        (EVENT_FORUM_POST_DELETE, post_handler),
        (EVENT_FORUM_REPLY_CREATE, reply_handler),
        (EVENT_FORUM_REPLY_DELETE, reply_handler),
        (EVENT_FORUM_AUDIT_RESULT, forum_audit_handler),
        (EVENT_INTERACTION_CREATE, interaction_handler),
        (EVENT_GROUP_AT_MESSAGE_CREATE, group_at_message_handler),
        (EVENT_C2C_MESSAGE_CREATE, c2c_message_handler),
        (EVENT_SUBSCRIBE_MESSAGE_STATUS, subscribe_msg_status_handler),
        (EVENT_C2C_FRIEND_ADD, c2c_friend_handler),
        (EVENT_C2C_FRIEND_DEL, c2c_friend_handler),
        (EVENT_ENTER_AIO, enter_aio_handler),
    ]
    .into_iter()
    .map(|(event, handler)| (WS_DISPATCH_EVENT, event.to_string(), handler))
    .collect()
}

pub(crate) fn parse_and_handle(payload: &mut WSPayload) -> crate::Result<()> {
    let raw = payload
        .raw_message
        .clone()
        .unwrap_or_else(|| serde_json::to_vec(payload).unwrap_or_default());
    let handler = {
        let handlers = EVENT_PARSE_HANDLERS
            .read()
            .expect("event handler map lock poisoned");
        payload.base.event_type.as_ref().and_then(|event_type| {
            handlers
                .iter()
                .find(|(op, event, _)| *op == payload.base.op_code && event.as_str() == event_type)
                .map(|(_, _, handler)| *handler)
        })
    };

    handler.map_or(Ok(()), |handler| handler(payload, &raw))
}
