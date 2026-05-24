//! Guild-related data models for the QQ Guild Bot API.
//!
//! This module contains guild types that correspond to the Python botpy implementation.

use crate::models::{HasId, HasName, Pager, Snowflake, Timestamp, channel::Channel};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

#[allow(non_upper_case_globals)]
pub const DefaultColor: u32 = 4_278_245_297;

fn insert_query_param(query: &mut HashMap<String, String>, key: &str, value: &Option<String>) {
    if let Some(value) = value.as_ref().filter(|value| !value.is_empty()) {
        query.insert(key.to_string(), value.clone());
    }
}

mod role_hoist_serde {
    use super::*;

    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_some(&u32::from(*value)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<serde_json::Value>::deserialize(deserializer)?;
        Ok(value.and_then(|value| match value {
            serde_json::Value::Bool(value) => Some(value),
            serde_json::Value::Number(value) => value.as_u64().map(|value| value != 0),
            _ => None,
        }))
    }
}

/// Represents a guild (server) in the QQ Guild system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Guild {
    /// The guild's unique ID
    pub id: Option<Snowflake>,
    /// The guild's name
    pub name: Option<String>,
    /// The guild's icon hash
    pub icon: Option<String>,
    /// The ID of the guild owner
    pub owner_id: Option<Snowflake>,
    /// Whether the current user is the owner of this guild
    #[serde(rename = "owner")]
    pub is_owner: Option<bool>,
    /// The number of members in this guild
    pub member_count: Option<u32>,
    /// The maximum number of members for this guild
    pub max_members: Option<u32>,
    /// The guild's description
    pub description: Option<String>,
    /// When the current user joined this guild
    pub joined_at: Option<Timestamp>,
    /// Channels contained in this guild when included by gateway payloads
    #[serde(default)]
    pub channels: Vec<Channel>,
    /// Bound game world/server ID
    pub union_world_id: Option<String>,
    /// Bound game organization/team ID
    pub union_org_id: Option<String>,
    /// Operator user ID
    pub op_user_id: Option<Snowflake>,
}

impl Guild {
    /// Creates a new guild.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            icon: None,
            owner_id: None,
            is_owner: None,
            member_count: None,
            max_members: None,
            description: None,
            joined_at: None,
            channels: Vec::new(),
            union_world_id: None,
            union_org_id: None,
            op_user_id: None,
        }
    }

    /// Creates a new guild from API data.
    pub fn from_data(api: crate::api::BotApi, id: String, data: serde_json::Value) -> Self {
        Self {
            id: Some(id),
            name: data.get("name").and_then(|v| v.as_str()).map(String::from),
            icon: data.get("icon").and_then(|v| v.as_str()).map(String::from),
            owner_id: data
                .get("owner_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            is_owner: data
                .get("owner")
                .or_else(|| data.get("is_owner"))
                .and_then(|v| v.as_bool()),
            member_count: data
                .get("member_count")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
            max_members: data
                .get("max_members")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32),
            description: data
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from),
            joined_at: data
                .get("joined_at")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            channels: data
                .get("channels")
                .and_then(|v| v.as_array())
                .map(|channels| {
                    channels
                        .iter()
                        .cloned()
                        .map(|channel| {
                            let channel_id = channel
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string();
                            Channel::from_data(api.clone(), channel_id, channel)
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            union_world_id: data
                .get("union_world_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            union_org_id: data
                .get("union_org_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            op_user_id: data
                .get("op_user_id")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }

    /// Gets the guild's icon URL if it has one.
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            format!(
                "https://groupprofile.qq.com/groupicon/{}/{}",
                self.id.as_ref().unwrap_or(&String::new()),
                hash
            )
        })
    }

    /// Returns true if the current user owns this guild.
    pub fn is_owned_by_current_user(&self) -> bool {
        self.is_owner.unwrap_or(false)
    }

    /// Gets the guild's member count.
    pub fn get_member_count(&self) -> u32 {
        self.member_count.unwrap_or(0)
    }

    /// Gets the guild's maximum member count.
    pub fn get_max_members(&self) -> u32 {
        self.max_members.unwrap_or(0)
    }

    /// Returns true if the guild has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        match (self.member_count, self.max_members) {
            (Some(current), Some(max)) => current >= max,
            _ => false,
        }
    }

    /// Gets the guild's display name (same as name for guilds).
    pub fn display_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns true if the guild has a description.
    pub fn has_description(&self) -> bool {
        self.description
            .as_ref()
            .is_some_and(|desc| !desc.is_empty())
    }
}

impl Default for Guild {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Guild {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

impl HasName for Guild {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

/// Guild roles response wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildRoles {
    /// Guild ID
    pub guild_id: Option<Snowflake>,
    /// List of roles in the guild
    pub roles: Vec<GuildRole>,
    /// Number of roles
    pub role_num_limit: Option<String>,
}

impl GuildRoles {
    /// Creates a new guild roles wrapper.
    pub fn new(roles: Vec<GuildRole>) -> Self {
        Self {
            guild_id: None,
            roles,
            role_num_limit: None,
        }
    }
}

/// Represents a role ID.
pub type RoleId = Snowflake;
/// Botgo-compatible role ID alias.
pub type RoleID = RoleId;
/// Default role color used by botgo when creating or updating roles.
pub const DEFAULT_ROLE_COLOR: u32 = 4_278_245_297;

/// Botgo-compatible role update info body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateRoleInfo {
    pub name: String,
    pub color: u32,
    pub hoist: u32,
}

/// Represents a role in a guild.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildRole {
    /// The role's unique ID
    pub id: Option<Snowflake>,
    /// The role's name
    pub name: Option<String>,
    /// The role's color (ARGB hex as decimal)
    pub color: Option<u32>,
    /// Whether this role is displayed separately in the member list
    #[serde(with = "role_hoist_serde", default)]
    pub hoist: Option<bool>,
    /// The number of members with this role
    pub number: Option<u32>,
    /// The number of online members with this role
    pub member_limit: Option<u32>,
}

impl GuildRole {
    fn default_color() -> u32 {
        DEFAULT_ROLE_COLOR
    }

    fn hoist_value(&self) -> u32 {
        self.hoist.map(u32::from).unwrap_or(0)
    }
}

/// Filter identifying which role fields are being updated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRoleFilter {
    pub name: u32,
    pub color: u32,
    pub hoist: u32,
}

impl Default for UpdateRoleFilter {
    fn default() -> Self {
        Self {
            name: 1,
            color: 1,
            hoist: 1,
        }
    }
}

/// Botgo-compatible role update body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRole {
    pub guild_id: String,
    pub filter: UpdateRoleFilter,
    #[serde(rename = "info")]
    pub update: GuildRole,
}

impl UpdateRole {
    /// Creates a role update body with botgo-compatible defaults.
    pub fn new(guild_id: impl Into<String>, mut role: GuildRole) -> Self {
        if role.color.unwrap_or(0) == 0 {
            role.color = Some(GuildRole::default_color());
        }
        role.hoist = Some(role.is_hoisted());
        Self {
            guild_id: guild_id.into(),
            filter: UpdateRoleFilter::default(),
            update: role,
        }
    }
}

/// Result returned from role create/update APIs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateResult {
    pub role_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub role: Option<GuildRole>,
}

impl GuildRole {
    /// Creates a new role.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            color: None,
            hoist: None,
            number: None,
            member_limit: None,
        }
    }

    /// Returns true if this role is hoisted (displayed separately).
    pub fn is_hoisted(&self) -> bool {
        self.hoist.unwrap_or(false)
    }

    /// Converts the role to the numeric hoist value expected by the API.
    pub fn hoist_as_u32(&self) -> u32 {
        self.hoist_value()
    }

    /// Gets the role's color as a hex value.
    pub fn color_hex(&self) -> Option<String> {
        self.color.map(|c| format!("#{c:06X}"))
    }

    /// Gets the number of members with this role.
    pub fn member_count(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    /// Gets the member limit for this role.
    pub fn get_member_limit(&self) -> u32 {
        self.member_limit.unwrap_or(0)
    }

    /// Returns true if the role has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        match (self.number, self.member_limit) {
            (Some(current), Some(limit)) => current >= limit,
            _ => false,
        }
    }
}

impl Default for GuildRole {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for GuildRole {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

impl HasName for GuildRole {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

/// Represents a role in a guild (legacy type alias).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    /// The role's unique ID
    pub id: Option<Snowflake>,
    /// The role's name
    pub name: Option<String>,
    /// The role's color (ARGB hex as decimal)
    pub color: Option<u32>,
    /// Whether this role is displayed separately in the member list
    pub hoist: Option<bool>,
    /// The number of members with this role
    pub number: Option<u32>,
    /// The number of online members with this role
    pub member_limit: Option<u32>,
}

impl Role {
    /// Creates a new role.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            color: None,
            hoist: None,
            number: None,
            member_limit: None,
        }
    }

    /// Returns true if this role is hoisted (displayed separately).
    pub fn is_hoisted(&self) -> bool {
        self.hoist.unwrap_or(false)
    }

    /// Gets the role's color as a hex value.
    pub fn color_hex(&self) -> Option<String> {
        self.color.map(|c| format!("#{c:06X}"))
    }

    /// Gets the number of members with this role.
    pub fn member_count(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    /// Gets the member limit for this role.
    pub fn get_member_limit(&self) -> u32 {
        self.member_limit.unwrap_or(0)
    }

    /// Returns true if the role has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        match (self.number, self.member_limit) {
            (Some(current), Some(limit)) => current >= limit,
            _ => false,
        }
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Role {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

impl HasName for Role {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

// Type alias for backward compatibility
pub type Roles = Vec<Role>;

/// Pager for guild member list requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildMembersPager {
    /// Read members after this user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl GuildMembersPager {
    /// Creates a new guild members pager.
    pub fn new(after: impl Into<String>, limit: impl ToString) -> Self {
        Self {
            after: Some(after.into()),
            limit: Some(limit.to_string()),
        }
    }

    /// Converts the pager to botgo-compatible query parameters.
    pub fn query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        insert_query_param(&mut query, "after", &self.after);
        query
    }

    /// Botgo-compatible query parameter accessor.
    #[allow(non_snake_case)]
    pub fn QueryParams(&self) -> HashMap<String, String> {
        self.query_params()
    }
}

impl Pager for GuildMembersPager {
    fn query_params(&self) -> HashMap<String, String> {
        GuildMembersPager::query_params(self)
    }
}

/// Pager for guild role member list requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildRoleMembersPager {
    /// Start index from the previous response's `next` value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<String>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl GuildRoleMembersPager {
    /// Creates a new guild role members pager.
    pub fn new(start_index: impl Into<String>, limit: impl ToString) -> Self {
        Self {
            start_index: Some(start_index.into()),
            limit: Some(limit.to_string()),
        }
    }

    /// Converts the pager to botgo-compatible query parameters.
    pub fn query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        insert_query_param(&mut query, "start_index", &self.start_index);
        query
    }

    /// Botgo-compatible query parameter accessor.
    #[allow(non_snake_case)]
    pub fn QueryParams(&self) -> HashMap<String, String> {
        self.query_params()
    }
}

impl Pager for GuildRoleMembersPager {
    fn query_params(&self) -> HashMap<String, String> {
        GuildRoleMembersPager::query_params(self)
    }
}

/// Pager for current-user guild list requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildPager {
    /// Read guilds before this guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Read guilds after this guild ID. Takes precedence over `before`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl GuildPager {
    /// Creates an empty guild pager.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn with_before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn with_limit(mut self, limit: impl ToString) -> Self {
        self.limit = Some(limit.to_string());
        self
    }

    /// Converts the pager to botgo-compatible query parameters.
    pub fn query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        insert_query_param(&mut query, "after", &self.after);
        if !query.contains_key("after") {
            insert_query_param(&mut query, "before", &self.before);
        }
        query
    }

    /// Botgo-compatible query parameter accessor.
    #[allow(non_snake_case)]
    pub fn QueryParams(&self) -> HashMap<String, String> {
        self.query_params()
    }
}

impl Pager for GuildPager {
    fn query_params(&self) -> HashMap<String, String> {
        GuildPager::query_params(self)
    }
}

/// Response returned by the guild role members endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildRoleMembers {
    /// Role members in the current page
    #[serde(default)]
    pub data: Vec<Member>,
    /// Cursor for the next page
    #[serde(default)]
    pub next: String,
}

/// Body used when adding or deleting a member role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberAddRoleBody {
    /// Channel object for channel administrator roles.
    pub channel: Option<Channel>,
}

impl MemberAddRoleBody {
    /// Creates a body without a channel.
    pub fn new() -> Self {
        Self { channel: None }
    }

    /// Creates a body for a channel-specific role.
    pub fn with_channel_id(channel_id: impl Into<String>) -> Self {
        let mut channel = Channel::new();
        channel.id = Some(channel_id.into());
        Self {
            channel: Some(channel),
        }
    }
}

/// Supported history deletion windows when removing a guild member.
pub type DeleteHistoryMsgDay = i32;

pub const NO_DELETE: DeleteHistoryMsgDay = 0;
pub const DELETE_THREE_DAYS: DeleteHistoryMsgDay = 3;
pub const DELETE_SEVEN_DAYS: DeleteHistoryMsgDay = 7;
pub const DELETE_FIFTEEN_DAYS: DeleteHistoryMsgDay = 15;
pub const DELETE_THIRTY_DAYS: DeleteHistoryMsgDay = 30;
pub const DELETE_ALL: DeleteHistoryMsgDay = -1;
#[allow(non_upper_case_globals)]
pub const NoDelete: DeleteHistoryMsgDay = NO_DELETE;
#[allow(non_upper_case_globals)]
pub const DeleteThreeDays: DeleteHistoryMsgDay = DELETE_THREE_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteSevenDays: DeleteHistoryMsgDay = DELETE_SEVEN_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteFifteenDays: DeleteHistoryMsgDay = DELETE_FIFTEEN_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteThirtyDays: DeleteHistoryMsgDay = DELETE_THIRTY_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteAll: DeleteHistoryMsgDay = DELETE_ALL;

pub type MemberDeleteOpts = MemberDeleteOptions;

/// Additional options for deleting a guild member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberDeleteOptions {
    /// Whether to add the member to the guild blacklist
    pub add_blacklist: bool,
    /// How many days of history to retract
    pub delete_history_msg_days: DeleteHistoryMsgDay,
}

impl MemberDeleteOptions {
    /// Creates delete options with official defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether the member should also be added to the blacklist.
    pub fn with_add_blacklist(mut self, add_blacklist: bool) -> Self {
        self.add_blacklist = add_blacklist;
        self
    }

    /// Sets the history deletion window.
    pub fn with_delete_history_msg_days(mut self, days: DeleteHistoryMsgDay) -> Self {
        self.delete_history_msg_days = normalize_delete_history_msg_days(days);
        self
    }
}

impl Default for MemberDeleteOptions {
    fn default() -> Self {
        Self {
            add_blacklist: false,
            delete_history_msg_days: NO_DELETE,
        }
    }
}

pub type MemberDeleteOption = Box<dyn FnOnce(&mut MemberDeleteOptions) + Send>;

#[allow(non_snake_case)]
pub fn WithAddBlackList(add_blacklist: bool) -> MemberDeleteOption {
    Box::new(move |options| {
        options.add_blacklist = add_blacklist;
    })
}

#[allow(non_snake_case)]
pub fn WithDeleteHistoryMsg(days: DeleteHistoryMsgDay) -> MemberDeleteOption {
    Box::new(move |options| {
        options.delete_history_msg_days = normalize_delete_history_msg_days(days);
    })
}

/// Normalizes history deletion days to the official supported values.
pub fn normalize_delete_history_msg_days(days: DeleteHistoryMsgDay) -> DeleteHistoryMsgDay {
    match days {
        DELETE_THREE_DAYS | DELETE_SEVEN_DAYS | DELETE_FIFTEEN_DAYS | DELETE_THIRTY_DAYS
        | DELETE_ALL => days,
        _ => NO_DELETE,
    }
}

/// Body for guild mute endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateGuildMute {
    /// Mute end timestamp in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_end_timestamp: Option<String>,
    /// Mute duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_seconds: Option<String>,
    /// User IDs for batch mute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl UpdateGuildMute {
    /// Creates a mute request body.
    pub fn new(mute_end_timestamp: Option<&str>, mute_seconds: Option<&str>) -> Self {
        Self {
            mute_end_timestamp: mute_end_timestamp.map(String::from),
            mute_seconds: mute_seconds.map(String::from),
            user_ids: None,
        }
    }

    /// Creates a batch mute request body.
    pub fn new_multi(
        user_ids: Vec<String>,
        mute_end_timestamp: Option<&str>,
        mute_seconds: Option<&str>,
    ) -> Self {
        Self {
            mute_end_timestamp: mute_end_timestamp.map(String::from),
            mute_seconds: mute_seconds.map(String::from),
            user_ids: Some(user_ids),
        }
    }

    /// Creates a request body that cancels mute.
    pub fn cancel() -> Self {
        Self {
            mute_end_timestamp: Some("0".to_string()),
            mute_seconds: Some("0".to_string()),
            user_ids: None,
        }
    }

    /// Creates a request body that cancels mute for multiple users.
    pub fn cancel_multi(user_ids: Vec<String>) -> Self {
        Self {
            mute_end_timestamp: Some("0".to_string()),
            mute_seconds: Some("0".to_string()),
            user_ids: Some(user_ids),
        }
    }
}

/// Response for batch guild mute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateGuildMuteResponse {
    /// Successfully muted user IDs
    #[serde(default)]
    pub user_ids: Vec<String>,
}

/// Represents a member of a guild.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    /// Guild ID
    pub guild_id: Option<Snowflake>,
    /// The user information
    pub user: Option<crate::models::User>,
    /// The member's nickname in the guild
    pub nick: Option<String>,
    /// The member's roles in the guild
    pub roles: Option<Vec<Snowflake>>,
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
    /// Operator user ID
    pub op_user_id: Option<Snowflake>,
}

impl Member {
    /// Creates a new member.
    pub fn new() -> Self {
        Self {
            guild_id: None,
            user: None,
            nick: None,
            roles: None,
            joined_at: None,
            op_user_id: None,
        }
    }

    /// Gets the member's display name (nickname or username).
    pub fn display_name(&self) -> Option<&str> {
        self.nick
            .as_deref()
            .or_else(|| self.user.as_ref().map(|u| u.username.as_str()))
    }

    /// Gets the member's username.
    pub fn username(&self) -> Option<&str> {
        self.user.as_ref().map(|u| u.username.as_str())
    }

    /// Gets the member's user ID.
    pub fn user_id(&self) -> Option<&Snowflake> {
        self.user.as_ref().map(|u| &u.id)
    }

    /// Returns true if the member is a bot.
    pub fn is_bot(&self) -> bool {
        self.user.as_ref().is_some_and(|u| u.is_bot())
    }

    /// Gets the member's roles.
    pub fn role_ids(&self) -> &[Snowflake] {
        self.roles.as_deref().unwrap_or(&[])
    }

    /// Returns true if the member has a specific role.
    pub fn has_role(&self, role_id: &str) -> bool {
        self.role_ids().iter().any(|id| id == role_id)
    }
}

impl Default for Member {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Member {
    fn id(&self) -> Option<&Snowflake> {
        self.user_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guild_creation() {
        let guild = Guild::new();
        assert!(guild.id.is_none());
        assert!(guild.name.is_none());
        assert!(!guild.is_owned_by_current_user());
        assert_eq!(guild.get_member_count(), 0);
        assert_eq!(guild.get_max_members(), 0);
    }

    #[test]
    fn test_guild_with_data() {
        let mut guild = Guild::new();
        guild.id = Some("123456789".to_string());
        guild.name = Some("Test Guild".to_string());
        guild.is_owner = Some(true);
        guild.member_count = Some(100);
        guild.max_members = Some(500);
        guild.description = Some("A test guild".to_string());

        assert_eq!(guild.id(), Some(&"123456789".to_string()));
        assert_eq!(guild.name(), "Test Guild");
        assert!(guild.is_owned_by_current_user());
        assert_eq!(guild.get_member_count(), 100);
        assert_eq!(guild.get_max_members(), 500);
        assert!(!guild.is_at_member_limit());
        assert!(guild.has_description());
        assert_eq!(guild.display_name(), Some("Test Guild"));
    }

    #[test]
    fn botgo_guild_fields_use_official_json_names() {
        let guild = Guild::from_data(
            crate::api::BotApi::new(crate::http::HttpClient::new(30, false).unwrap()),
            "guild-1".to_string(),
            serde_json::json!({
                "name": "Guild",
                "owner": true,
                "channels": [
                    {
                        "id": "channel-1",
                        "guild_id": "guild-1",
                        "name": "general",
                        "type": 0
                    }
                ],
                "union_world_id": "world-1",
                "union_org_id": "org-1",
                "op_user_id": "operator-1"
            }),
        );

        assert_eq!(guild.id.as_deref(), Some("guild-1"));
        assert_eq!(guild.is_owner, Some(true));
        assert_eq!(guild.channels.len(), 1);
        assert_eq!(guild.channels[0].id.as_deref(), Some("channel-1"));
        assert_eq!(guild.union_world_id.as_deref(), Some("world-1"));
        assert_eq!(guild.union_org_id.as_deref(), Some("org-1"));
        assert_eq!(guild.op_user_id.as_deref(), Some("operator-1"));

        let value = serde_json::to_value(&guild).unwrap();
        assert_eq!(value["owner"], serde_json::json!(true));
        assert!(value.get("is_owner").is_none());
        assert_eq!(value["channels"][0]["id"], serde_json::json!("channel-1"));
        assert_eq!(value["union_world_id"], serde_json::json!("world-1"));
    }

    #[test]
    fn test_member_limit() {
        let mut guild = Guild::new();
        guild.member_count = Some(500);
        guild.max_members = Some(500);
        assert!(guild.is_at_member_limit());

        guild.member_count = Some(499);
        assert!(!guild.is_at_member_limit());

        guild.member_count = Some(501);
        assert!(guild.is_at_member_limit());
    }

    #[test]
    fn test_icon_url() {
        let mut guild = Guild::new();
        assert!(guild.icon_url().is_none());

        guild.id = Some("123456789".to_string());
        guild.icon = Some("abc123".to_string());
        let url = guild.icon_url().unwrap();
        assert!(url.contains("123456789"));
        assert!(url.contains("abc123"));
    }

    #[test]
    fn test_role_creation() {
        let role = Role::new();
        assert!(role.id.is_none());
        assert!(role.name.is_none());
        assert!(!role.is_hoisted());
        assert_eq!(role.member_count(), 0);
    }

    #[test]
    fn test_role_with_data() {
        let mut role = Role::new();
        role.id = Some("role123".to_string());
        role.name = Some("Admin".to_string());
        role.color = Some(0xFF0000);
        role.hoist = Some(true);
        role.number = Some(5);
        role.member_limit = Some(10);

        assert_eq!(role.id(), Some(&"role123".to_string()));
        assert_eq!(role.name(), "Admin");
        assert_eq!(role.color_hex(), Some("#FF0000".to_string()));
        assert!(role.is_hoisted());
        assert_eq!(role.member_count(), 5);
        assert_eq!(role.get_member_limit(), 10);
        assert!(!role.is_at_member_limit());
    }

    #[test]
    fn botgo_pager_query_params_match_official_priority() {
        let members = GuildMembersPager::new("user-1", 100);
        assert_eq!(
            members.QueryParams().get("after").map(String::as_str),
            Some("user-1")
        );

        let role_members = GuildRoleMembersPager::new("next-1", 50);
        assert_eq!(
            role_members
                .QueryParams()
                .get("start_index")
                .map(String::as_str),
            Some("next-1")
        );

        let guilds = GuildPager::new()
            .with_before("before-1")
            .with_after("after-1")
            .with_limit(20);
        let query = guilds.QueryParams();
        assert_eq!(query.get("after").map(String::as_str), Some("after-1"));
        assert!(!query.contains_key("before"));
    }

    #[test]
    fn test_member_creation() {
        let member = Member::new();
        assert!(member.user.is_none());
        assert!(member.nick.is_none());
        assert_eq!(member.role_ids().len(), 0);
    }

    #[test]
    fn test_member_with_roles() {
        let mut member = Member::new();
        member.roles = Some(vec!["role1".to_string(), "role2".to_string()]);

        assert!(member.has_role("role1"));
        assert!(member.has_role("role2"));
        assert!(!member.has_role("role3"));
        assert_eq!(member.role_ids().len(), 2);
    }
}
