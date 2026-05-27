pub(crate) const GATEWAY_BOT: &str = "/gateway/bot";
pub(crate) const USER_ME: &str = "/users/@me";
pub(crate) const USER_ME_GUILDS: &str = "/users/@me/guilds";
pub(crate) const USER_ME_DMS: &str = "/users/@me/dms";
pub(crate) const WEBHOOK_SESSIONS: &str = "/gateway/webhook/sessions";

pub(crate) fn webhook_session(session_id: &str) -> String {
    format!("/gateway/webhook/sessions/{session_id}")
}

pub(crate) fn guild(guild_id: &str) -> String {
    format!("/guilds/{guild_id}")
}

pub(crate) fn guild_members(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/members")
}

pub(crate) fn guild_member(guild_id: &str, user_id: &str) -> String {
    format!("/guilds/{guild_id}/members/{user_id}")
}

pub(crate) fn guild_role_members(guild_id: &str, role_id: &str) -> String {
    format!("/guilds/{guild_id}/roles/{role_id}/members")
}

pub(crate) fn guild_mute(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/mute")
}

pub(crate) fn guild_member_mute(guild_id: &str, user_id: &str) -> String {
    format!("/guilds/{guild_id}/members/{user_id}/mute")
}

pub(crate) fn guild_channels(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/channels")
}

pub(crate) fn channel(channel_id: &str) -> String {
    format!("/channels/{channel_id}")
}

pub(crate) fn voice_channel_members(channel_id: &str) -> String {
    format!("/channels/{channel_id}/voice/members")
}

pub(crate) fn channel_member_permissions(channel_id: &str, user_id: &str) -> String {
    format!("/channels/{channel_id}/members/{user_id}/permissions")
}

pub(crate) fn channel_role_permissions(channel_id: &str, role_id: &str) -> String {
    format!("/channels/{channel_id}/roles/{role_id}/permissions")
}

pub(crate) fn channel_messages(channel_id: &str) -> String {
    format!("/channels/{channel_id}/messages")
}

pub(crate) fn channel_message(channel_id: &str, message_id: &str) -> String {
    format!("/channels/{channel_id}/messages/{message_id}")
}

pub(crate) fn channel_setting_guide(channel_id: &str) -> String {
    format!("/channels/{channel_id}/settingguide")
}

pub(crate) fn group_messages(group_openid: &str) -> String {
    format!("/v2/groups/{group_openid}/messages")
}

pub(crate) fn group_file(group_openid: &str) -> String {
    format!("/v2/groups/{group_openid}/files")
}

pub(crate) fn group_message(group_openid: &str, message_id: &str) -> String {
    format!("/v2/groups/{group_openid}/messages/{message_id}")
}

pub(crate) fn c2c_messages(openid: &str) -> String {
    format!("/v2/users/{openid}/messages")
}

pub(crate) fn c2c_file(openid: &str) -> String {
    format!("/v2/users/{openid}/files")
}

pub(crate) fn c2c_message(openid: &str, message_id: &str) -> String {
    format!("/v2/users/{openid}/messages/{message_id}")
}

pub(crate) fn dms_messages(guild_id: &str) -> String {
    format!("/dms/{guild_id}/messages")
}

pub(crate) fn dms_message(guild_id: &str, message_id: &str) -> String {
    format!("/dms/{guild_id}/messages/{message_id}")
}

pub(crate) fn dms_setting_guide(guild_id: &str) -> String {
    format!("/dms/{guild_id}/settingguide")
}

pub(crate) fn channel_audio(channel_id: &str) -> String {
    format!("/channels/{channel_id}/audio")
}

pub(crate) fn channel_mic(channel_id: &str) -> String {
    format!("/channels/{channel_id}/mic")
}

pub(crate) fn channel_threads(channel_id: &str) -> String {
    format!("/channels/{channel_id}/threads")
}

pub(crate) fn channel_thread(channel_id: &str, thread_id: &str) -> String {
    format!("/channels/{channel_id}/threads/{thread_id}")
}

pub(crate) fn guild_roles(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/roles")
}

pub(crate) fn guild_role(guild_id: &str, role_id: &str) -> String {
    format!("/guilds/{guild_id}/roles/{role_id}")
}

pub(crate) fn guild_member_role(guild_id: &str, user_id: &str, role_id: &str) -> String {
    format!("/guilds/{guild_id}/members/{user_id}/roles/{role_id}")
}

pub(crate) fn channel_announces(channel_id: &str) -> String {
    format!("/channels/{channel_id}/announces")
}

pub(crate) fn channel_announce(channel_id: &str, message_id: &str) -> String {
    format!("/channels/{channel_id}/announces/{message_id}")
}

pub(crate) fn channel_announces_all(channel_id: &str) -> String {
    format!("/channels/{channel_id}/announces/all")
}

pub(crate) fn guild_announces(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/announces")
}

pub(crate) fn guild_announce(guild_id: &str, message_id: &str) -> String {
    format!("/guilds/{guild_id}/announces/{message_id}")
}

pub(crate) fn guild_announces_all(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/announces/all")
}

pub(crate) fn channel_schedules(channel_id: &str) -> String {
    format!("/channels/{channel_id}/schedules")
}

pub(crate) fn channel_schedule(channel_id: &str, schedule_id: &str) -> String {
    format!("/channels/{channel_id}/schedules/{schedule_id}")
}

pub(crate) fn api_permission(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/api_permission")
}

pub(crate) fn api_permission_demand(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/api_permission/demand")
}

pub(crate) fn channel_pins(channel_id: &str) -> String {
    format!("/channels/{channel_id}/pins")
}

pub(crate) fn channel_pin(channel_id: &str, message_id: &str) -> String {
    format!("/channels/{channel_id}/pins/{message_id}")
}

pub(crate) fn channel_pins_all(channel_id: &str) -> String {
    format!("/channels/{channel_id}/pins/all")
}

pub(crate) fn message_reaction(
    channel_id: &str,
    message_id: &str,
    emoji_type: impl std::fmt::Display,
    emoji_id: impl std::fmt::Display,
) -> String {
    format!("/channels/{channel_id}/messages/{message_id}/reactions/{emoji_type}/{emoji_id}")
}

pub(crate) fn interaction(interaction_id: &str) -> String {
    format!("/interactions/{interaction_id}")
}

pub(crate) fn guild_message_setting(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/message/setting")
}
