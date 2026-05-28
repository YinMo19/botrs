pub(crate) const GATEWAY_BOT: &str = "/gateway/bot";
pub(crate) const USER_ME: &str = "/users/@me";
pub(crate) const USER_ME_DMS: &str = "/users/@me/dms";

pub(crate) fn channel_messages(channel_id: &str) -> String {
    format!("/channels/{channel_id}/messages")
}

pub(crate) fn channel_message(channel_id: &str, message_id: &str) -> String {
    format!("/channels/{channel_id}/messages/{message_id}")
}

pub(crate) fn group_messages(group_openid: &str) -> String {
    format!("/v2/groups/{group_openid}/messages")
}

pub(crate) fn group_file(group_openid: &str) -> String {
    format!("/v2/groups/{group_openid}/files")
}

pub(crate) fn c2c_messages(openid: &str) -> String {
    format!("/v2/users/{openid}/messages")
}

pub(crate) fn c2c_file(openid: &str) -> String {
    format!("/v2/users/{openid}/files")
}

pub(crate) fn dms_messages(guild_id: &str) -> String {
    format!("/dms/{guild_id}/messages")
}

pub(crate) fn guild_announces(guild_id: &str) -> String {
    format!("/guilds/{guild_id}/announces")
}

pub(crate) fn guild_announce(guild_id: &str, message_id: &str) -> String {
    format!("/guilds/{guild_id}/announces/{message_id}")
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

pub(crate) fn message_reaction(
    channel_id: &str,
    message_id: &str,
    emoji_type: impl std::fmt::Display,
    emoji_id: impl std::fmt::Display,
) -> String {
    format!("/channels/{channel_id}/messages/{message_id}/reactions/{emoji_type}/{emoji_id}")
}
