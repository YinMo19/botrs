use super::{DEFAULT_HANDLERS, PayloadData};
use crate::forum::{ForumAuditResult, Post, Reply, Thread};
use crate::interaction::Interaction;
use crate::manage::{C2CFriendData, EnterAioEvent, SubscribeMessageStatusData};
use crate::models::{
    api::AudioAction,
    channel::Channel,
    gateway::WSPayload,
    guild::{Guild, Member},
    message::{Message, MessageAudit, MessageDelete},
};
use crate::reaction::MessageReaction;

macro_rules! typed_handler {
    ($fn_name:ident, $field:ident, $data:ty) => {
        pub(super) fn $fn_name(payload: &mut WSPayload, message: &[u8]) -> crate::Result<()> {
            let mut data: $data = <$data as PayloadData>::parse_from_payload(payload, message)?;
            if let Some(handler) = DEFAULT_HANDLERS
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

typed_handler!(guild_handler, guild, Guild);
typed_handler!(guild_member_handler, guild_member, Member);
typed_handler!(channel_handler, channel, Channel);
typed_handler!(message_handler, message, Message);
typed_handler!(message_delete_handler, message_delete, MessageDelete);
typed_handler!(message_reaction_handler, message_reaction, MessageReaction);
typed_handler!(at_message_handler, at_message, Message);
typed_handler!(
    public_message_delete_handler,
    public_message_delete,
    MessageDelete
);
typed_handler!(direct_message_handler, direct_message, Message);
typed_handler!(
    direct_message_delete_handler,
    direct_message_delete,
    MessageDelete
);
typed_handler!(audio_handler, audio, AudioAction);
typed_handler!(thread_handler, thread, Thread);
typed_handler!(post_handler, post, Post);
typed_handler!(reply_handler, reply, Reply);
typed_handler!(forum_audit_handler, forum_audit, ForumAuditResult);
typed_handler!(message_audit_handler, message_audit, MessageAudit);
typed_handler!(interaction_handler, interaction, Interaction);
typed_handler!(group_at_message_handler, group_at_message, Message);
typed_handler!(c2c_message_handler, c2c_message, Message);
typed_handler!(
    subscribe_msg_status_handler,
    subscribe_msg_status,
    SubscribeMessageStatusData
);
typed_handler!(c2c_friend_handler, c2c_friend, C2CFriendData);
typed_handler!(enter_aio_handler, enter_aio, EnterAioEvent);
