use super::ConnectionState;
use crate::audio::{Audio, PublicAudio};
use crate::forum::{OpenThread, Thread};
use crate::interaction::Interaction;
use crate::manage::{C2CManageEvent, GroupManageEvent};
use crate::models::{api::AudioAction, channel::Channel, guild::Guild, message::*, user::Member};
use crate::reaction::Reaction;
use serde_json::Value;

pub(super) fn parse_ready(
    _state: &ConnectionState,
    _payload: &Value,
) -> Option<(&'static str, Value)> {
    Some(("ready", Value::Null))
}

pub(super) fn parse_resumed(
    _state: &ConnectionState,
    _payload: &Value,
) -> Option<(&'static str, Value)> {
    Some(("resumed", Value::Null))
}

pub(super) fn parse_guild_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let guild_id = payload.get("id").and_then(|v| v.as_str())?;
    let guild_data = payload.get("d")?;
    let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
    Some(("guild_create", serde_json::to_value(guild).ok()?))
}

pub(super) fn parse_guild_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let guild_id = payload.get("id").and_then(|v| v.as_str())?;
    let guild_data = payload.get("d")?;
    let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
    Some(("guild_update", serde_json::to_value(guild).ok()?))
}

pub(super) fn parse_guild_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let guild_id = payload.get("id").and_then(|v| v.as_str())?;
    let guild_data = payload.get("d")?;
    let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
    Some(("guild_delete", serde_json::to_value(guild).ok()?))
}

pub(super) fn parse_channel_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let channel_id = payload.get("id").and_then(|v| v.as_str())?;
    let channel_data = payload.get("d")?;
    let channel = Channel::from_data(
        state.api.clone(),
        channel_id.to_string(),
        channel_data.clone(),
    );
    Some(("channel_create", serde_json::to_value(channel).ok()?))
}

pub(super) fn parse_channel_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let channel_id = payload.get("id").and_then(|v| v.as_str())?;
    let channel_data = payload.get("d")?;
    let channel = Channel::from_data(
        state.api.clone(),
        channel_id.to_string(),
        channel_data.clone(),
    );
    Some(("channel_update", serde_json::to_value(channel).ok()?))
}

pub(super) fn parse_channel_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let channel_id = payload.get("id").and_then(|v| v.as_str())?;
    let channel_data = payload.get("d")?;
    let channel = Channel::from_data(
        state.api.clone(),
        channel_id.to_string(),
        channel_data.clone(),
    );
    Some(("channel_delete", serde_json::to_value(channel).ok()?))
}

pub(super) fn parse_guild_member_add(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let _member_id = payload.get("id").and_then(|v| v.as_str())?;
    let member_data = payload.get("d")?;
    let member = Member::from_data(member_data.clone());
    Some(("guild_member_add", serde_json::to_value(member).ok()?))
}

pub(super) fn parse_guild_member_update(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let _member_id = payload.get("id").and_then(|v| v.as_str())?;
    let member_data = payload.get("d")?;
    let member = Member::from_data(member_data.clone());
    Some(("guild_member_update", serde_json::to_value(member).ok()?))
}

pub(super) fn parse_guild_member_remove(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let _member_id = payload.get("id").and_then(|v| v.as_str())?;
    let member_data = payload.get("d")?;
    let member = Member::from_data(member_data.clone());
    Some(("guild_member_remove", serde_json::to_value(member).ok()?))
}

pub(super) fn parse_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = Message::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_create", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_message_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = crate::models::message::MessageDelete::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_delete", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_at_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = Message::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("at_message_create", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_public_message_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = crate::models::message::MessageDelete::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("public_message_delete", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_direct_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = Message::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("direct_message_create", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_direct_message_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = crate::models::message::MessageDelete::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("direct_message_delete", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_message_reaction_add(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reaction_id = payload.get("id").and_then(|v| v.as_str())?;
    let reaction_data = payload.get("d")?;
    let reaction = Reaction::new(
        state.api.clone(),
        Some(reaction_id.to_string()),
        reaction_data,
    )
    .ok()?;
    Some(("message_reaction_add", serde_json::to_value(reaction).ok()?))
}

pub(super) fn parse_message_reaction_remove(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reaction_id = payload.get("id").and_then(|v| v.as_str())?;
    let reaction_data = payload.get("d")?;
    let reaction = Reaction::new(
        state.api.clone(),
        Some(reaction_id.to_string()),
        reaction_data,
    )
    .ok()?;
    Some((
        "message_reaction_remove",
        serde_json::to_value(reaction).ok()?,
    ))
}

pub(super) fn parse_interaction_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let interaction_id = payload.get("id").and_then(|v| v.as_str())?;
    let interaction_data = payload.get("d")?;
    let interaction = Interaction::new(
        state.api.clone(),
        Some(interaction_id.to_string()),
        interaction_data,
    );
    Some((
        "interaction_create",
        serde_json::to_value(interaction).ok()?,
    ))
}

pub(super) fn parse_audio_start(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("audio_start", serde_json::to_value(audio).ok()?))
}

pub(super) fn parse_audio_finish(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("audio_finish", serde_json::to_value(audio).ok()?))
}

pub(super) fn parse_on_mic(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("on_mic", serde_json::to_value(audio).ok()?))
}

pub(super) fn parse_off_mic(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_id = payload.get("id").and_then(|v| v.as_str())?;
    let audio_data = payload.get("d")?;
    let audio_action = AudioAction::from_value(audio_data);
    let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
    Some(("off_mic", serde_json::to_value(audio).ok()?))
}

pub(super) fn parse_audio_or_live_channel_member_enter(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_data = payload.get("d")?;
    let public_audio = PublicAudio::new(state.api.clone(), audio_data.clone());
    Some((
        "audio_or_live_channel_member_enter",
        serde_json::to_value(public_audio).ok()?,
    ))
}

pub(super) fn parse_audio_or_live_channel_member_exit(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audio_data = payload.get("d")?;
    let public_audio = PublicAudio::new(state.api.clone(), audio_data.clone());
    Some((
        "audio_or_live_channel_member_exit",
        serde_json::to_value(public_audio).ok()?,
    ))
}

pub(super) fn parse_forum_thread_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_id = payload.get("id").and_then(|v| v.as_str())?;
    let thread_data = payload.get("d")?;
    let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
    Some(("forum_thread_create", serde_json::to_value(thread).ok()?))
}

pub(super) fn parse_forum_thread_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_id = payload.get("id").and_then(|v| v.as_str())?;
    let thread_data = payload.get("d")?;
    let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
    Some(("forum_thread_update", serde_json::to_value(thread).ok()?))
}

pub(super) fn parse_forum_thread_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_id = payload.get("id").and_then(|v| v.as_str())?;
    let thread_data = payload.get("d")?;
    let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
    Some(("forum_thread_delete", serde_json::to_value(thread).ok()?))
}

pub(super) fn parse_forum_post_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("forum_post_create", post_data.clone()))
}

pub(super) fn parse_forum_post_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("forum_post_delete", post_data.clone()))
}

pub(super) fn parse_forum_reply_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("forum_reply_create", reply_data.clone()))
}

pub(super) fn parse_forum_reply_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("forum_reply_delete", reply_data.clone()))
}

pub(super) fn parse_forum_publish_audit_result(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let audit_data = payload.get("d")?;
    Some(("forum_publish_audit_result", audit_data.clone()))
}

pub(super) fn parse_open_forum_thread_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_data = payload.get("d")?;
    let thread = OpenThread::new(state.api.clone(), thread_data);
    Some((
        "open_forum_thread_create",
        serde_json::to_value(thread).ok()?,
    ))
}

pub(super) fn parse_open_forum_thread_update(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_data = payload.get("d")?;
    let thread = OpenThread::new(state.api.clone(), thread_data);
    Some((
        "open_forum_thread_update",
        serde_json::to_value(thread).ok()?,
    ))
}

pub(super) fn parse_open_forum_thread_delete(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let thread_data = payload.get("d")?;
    let thread = OpenThread::new(state.api.clone(), thread_data);
    Some((
        "open_forum_thread_delete",
        serde_json::to_value(thread).ok()?,
    ))
}

pub(super) fn parse_open_forum_post_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("open_forum_post_create", post_data.clone()))
}

pub(super) fn parse_open_forum_post_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let post_data = payload.get("d")?;
    Some(("open_forum_post_delete", post_data.clone()))
}

pub(super) fn parse_open_forum_reply_create(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("open_forum_reply_create", reply_data.clone()))
}

pub(super) fn parse_open_forum_reply_delete(
    _state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let reply_data = payload.get("d")?;
    Some(("open_forum_reply_delete", reply_data.clone()))
}

pub(super) fn parse_group_at_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = GroupMessage::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some((
        "group_at_message_create",
        serde_json::to_value(message).ok()?,
    ))
}

pub(super) fn parse_c2c_message_create(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = C2CMessage::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("c2c_message_create", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_group_add_robot(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_add_robot", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_group_del_robot(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_del_robot", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_group_msg_reject(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_msg_reject", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_group_msg_receive(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("group_msg_receive", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_friend_add(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("friend_add", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_friend_del(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("friend_del", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_c2c_msg_reject(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("c2c_msg_reject", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_c2c_msg_receive(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let event_id = payload.get("id").and_then(|v| v.as_str())?;
    let event_data = payload.get("d")?;
    let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), event_data);
    Some(("c2c_msg_receive", serde_json::to_value(event).ok()?))
}

pub(super) fn parse_message_audit_pass(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = MessageAudit::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_audit_pass", serde_json::to_value(message).ok()?))
}

pub(super) fn parse_message_audit_reject(
    state: &ConnectionState,
    payload: &Value,
) -> Option<(&'static str, Value)> {
    let message_id = payload.get("id").and_then(|v| v.as_str())?;
    let message_data = payload.get("d")?;
    let message = MessageAudit::from_data(
        state.api.clone(),
        message_id.to_string(),
        message_data.clone(),
    );
    Some(("message_audit_reject", serde_json::to_value(message).ok()?))
}
