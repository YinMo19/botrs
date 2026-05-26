use super::{DefaultHandlers, PayloadData};
use crate::models::gateway::*;

macro_rules! typed_handler {
    ($fn_name:ident, $field:ident, $data:ty) => {
        pub(super) fn $fn_name(payload: &mut WSPayload, message: &[u8]) -> crate::Result<()> {
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
