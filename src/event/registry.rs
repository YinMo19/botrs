use super::handlers::*;
use super::{DEFAULT_HANDLERS, EventParseFn};
use crate::models::gateway::*;
use std::sync::{LazyLock, RwLock};

static EVENT_PARSE_HANDLERS: LazyLock<RwLock<Vec<(OpCode, EventType, EventParseFn)>>> =
    LazyLock::new(|| RwLock::new(default_event_handlers()));

fn default_event_handlers() -> Vec<(OpCode, EventType, EventParseFn)> {
    [
        (EventGuildCreate, guild_handler as EventParseFn),
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

pub fn register_handler(op_code: OpCode, event_type: impl Into<EventType>, handler: EventParseFn) {
    let event_type = event_type.into();
    let mut handlers = EVENT_PARSE_HANDLERS
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

pub fn parse_and_handle(payload: &mut WSPayload) -> crate::Result<()> {
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

    if let Some(handler) = handler {
        handler(payload, &raw)
    } else if let Some(plain) = DEFAULT_HANDLERS
        .read()
        .expect("default handlers lock poisoned")
        .plain
    {
        plain(payload, &raw)
    } else {
        Ok(())
    }
}
