//! Guild-related data models for the QQ Guild Bot API.
//!
//! This module contains guild types that correspond to the Python botpy implementation.

use crate::models::{HasId, HasName, Pager, Snowflake, Timestamp, channel::Channel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_upper_case_globals)]
pub const DefaultColor: u32 = 4_278_245_297;

fn insert_query_param(query: &mut HashMap<String, String>, key: &str, value: &Option<String>) {
    if let Some(value) = value.as_ref().filter(|value| !value.is_empty()) {
        query.insert(key.to_string(), value.clone());
    }
}

/// Represents a guild (server) in the QQ Guild system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Guild {
    /// The guild's unique ID
    #[serde(default)]
    pub id: Snowflake,
    /// The guild's name
    #[serde(default)]
    pub name: String,
    /// The guild's icon hash
    #[serde(default)]
    pub icon: String,
    /// The ID of the guild owner
    #[serde(default)]
    pub owner_id: Snowflake,
    /// Whether the current user is the owner of this guild
    #[serde(rename = "owner")]
    #[serde(default)]
    pub is_owner: bool,
    /// The number of members in this guild
    #[serde(default)]
    pub member_count: i32,
    /// The maximum number of members for this guild
    #[serde(default)]
    pub max_members: i64,
    /// The guild's description
    #[serde(default)]
    pub description: String,
    /// When the current user joined this guild
    #[serde(default)]
    pub joined_at: Timestamp,
    /// Channels contained in this guild when included by gateway payloads
    #[serde(default)]
    pub channels: Vec<Channel>,
    /// Bound game world/server ID
    #[serde(default)]
    pub union_world_id: String,
    /// Bound game organization/team ID
    #[serde(default)]
    pub union_org_id: String,
    /// Operator user ID
    #[serde(default)]
    pub op_user_id: Snowflake,
}

impl Guild {
    /// Creates a new guild.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new guild from API data.
    pub fn from_data(_api: crate::api::BotApi, id: String, data: serde_json::Value) -> Self {
        let mut guild = serde_json::from_value::<Self>(data).unwrap_or_default();
        if guild.id.is_empty() {
            guild.id = id;
        }
        guild
    }

    /// Gets the guild's icon URL if it has one.
    pub fn icon_url(&self) -> Option<String> {
        (!self.icon.is_empty()).then(|| {
            format!(
                "https://groupprofile.qq.com/groupicon/{}/{}",
                self.id, self.icon
            )
        })
    }

    /// Returns true if the current user owns this guild.
    pub fn is_owned_by_current_user(&self) -> bool {
        self.is_owner
    }

    /// Gets the guild's member count.
    pub fn get_member_count(&self) -> i32 {
        self.member_count
    }

    /// Gets the guild's maximum member count.
    pub fn get_max_members(&self) -> i64 {
        self.max_members
    }

    /// Returns true if the guild has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        self.max_members > 0 && i64::from(self.member_count) >= self.max_members
    }

    /// Gets the guild's display name (same as name for guilds).
    pub fn display_name(&self) -> Option<&str> {
        (!self.name.is_empty()).then_some(self.name.as_str())
    }

    /// Returns true if the guild has a description.
    pub fn has_description(&self) -> bool {
        !self.description.is_empty()
    }
}

impl HasId for Guild {
    fn id(&self) -> Option<&Snowflake> {
        (!self.id.is_empty()).then_some(&self.id)
    }
}

impl HasName for Guild {
    fn name(&self) -> &str {
        &self.name
    }
}

/// Guild roles response wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildRoles {
    /// Guild ID
    #[serde(default)]
    pub guild_id: Snowflake,
    /// List of roles in the guild
    #[serde(default)]
    pub roles: Vec<GuildRole>,
    /// Number of roles
    #[serde(default, rename = "role_num_limit")]
    pub num_limit: String,
}

impl GuildRoles {
    /// Creates a new guild roles wrapper.
    pub fn new(roles: Vec<GuildRole>) -> Self {
        Self {
            guild_id: String::new(),
            roles,
            num_limit: String::new(),
        }
    }
}

/// Represents a role ID.
pub type RoleId = Snowflake;
/// Botgo-compatible role ID alias.
pub type RoleID = RoleId;
/// Default role color used by botgo when creating or updating roles.
pub const DEFAULT_ROLE_COLOR: u32 = 4_278_245_297;

fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

/// Botgo-compatible role update info body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateRoleInfo {
    pub name: String,
    pub color: u32,
    pub hoist: u32,
}

/// Represents a role in a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildRole {
    /// The role's unique ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: Snowflake,
    /// The role's name
    #[serde(default)]
    pub name: String,
    /// The role's color (ARGB hex as decimal)
    #[serde(default)]
    pub color: u32,
    /// Whether this role is displayed separately in the member list
    #[serde(default)]
    pub hoist: u32,
    /// The number of members with this role
    #[serde(default, rename = "number", skip_serializing_if = "is_zero_u32")]
    pub member_count: u32,
    /// The number of online members with this role
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub member_limit: u32,
}

impl GuildRole {
    fn default_color() -> u32 {
        DEFAULT_ROLE_COLOR
    }

    fn hoist_value(&self) -> u32 {
        self.hoist
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
        if role.color == 0 {
            role.color = GuildRole::default_color();
        }
        Self {
            guild_id: guild_id.into(),
            filter: UpdateRoleFilter::default(),
            update: role,
        }
    }
}

/// Result returned from role create/update APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpdateResult {
    #[serde(default)]
    pub role_id: Snowflake,
    #[serde(default)]
    pub guild_id: Snowflake,
    #[serde(default)]
    pub role: Option<GuildRole>,
}

impl GuildRole {
    /// Creates a new role.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if this role is hoisted (displayed separately).
    pub fn is_hoisted(&self) -> bool {
        self.hoist != 0
    }

    /// Converts the role to the numeric hoist value expected by the API.
    pub fn hoist_as_u32(&self) -> u32 {
        self.hoist_value()
    }

    /// Gets the role's color as a hex value.
    pub fn color_hex(&self) -> Option<String> {
        (self.color != 0).then(|| format!("#{:06X}", self.color))
    }

    /// Gets the number of members with this role.
    pub fn member_count(&self) -> u32 {
        self.member_count
    }

    /// Gets the member limit for this role.
    pub fn get_member_limit(&self) -> u32 {
        self.member_limit
    }

    /// Returns true if the role has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        self.member_limit > 0 && self.member_count >= self.member_limit
    }
}

impl HasId for GuildRole {
    fn id(&self) -> Option<&Snowflake> {
        (!self.id.is_empty()).then_some(&self.id)
    }
}

impl HasName for GuildRole {
    fn name(&self) -> &str {
        &self.name
    }
}

/// Represents a role in a guild (legacy type alias).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Role {
    /// The role's unique ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: Snowflake,
    /// The role's name
    #[serde(default)]
    pub name: String,
    /// The role's color (ARGB hex as decimal)
    #[serde(default)]
    pub color: u32,
    /// Whether this role is displayed separately in the member list
    #[serde(default)]
    pub hoist: u32,
    /// The number of members with this role
    #[serde(default, rename = "number", skip_serializing_if = "is_zero_u32")]
    pub member_count: u32,
    /// The number of online members with this role
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub member_limit: u32,
}

impl Role {
    /// Creates a new role.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if this role is hoisted (displayed separately).
    pub fn is_hoisted(&self) -> bool {
        self.hoist != 0
    }

    /// Gets the role's color as a hex value.
    pub fn color_hex(&self) -> Option<String> {
        (self.color != 0).then(|| format!("#{:06X}", self.color))
    }

    /// Gets the number of members with this role.
    pub fn member_count(&self) -> u32 {
        self.member_count
    }

    /// Gets the member limit for this role.
    pub fn get_member_limit(&self) -> u32 {
        self.member_limit
    }

    /// Returns true if the role has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        self.member_limit > 0 && self.member_count >= self.member_limit
    }
}

impl HasId for Role {
    fn id(&self) -> Option<&Snowflake> {
        (!self.id.is_empty()).then_some(&self.id)
    }
}

impl HasName for Role {
    fn name(&self) -> &str {
        &self.name
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
        channel.id = channel_id.into();
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Member {
    /// Guild ID
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The user information
    #[serde(default)]
    pub user: Option<crate::models::User>,
    /// The member's nickname in the guild
    #[serde(default)]
    pub nick: String,
    /// The member's roles in the guild
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// When the member joined the guild
    #[serde(default)]
    pub joined_at: Timestamp,
    /// Operator user ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub op_user_id: Snowflake,
}

impl Member {
    /// Creates a new member.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the member's display name (nickname or username).
    pub fn display_name(&self) -> Option<&str> {
        (!self.nick.is_empty())
            .then_some(self.nick.as_str())
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
        &self.roles
    }

    /// Returns true if the member has a specific role.
    pub fn has_role(&self, role_id: &str) -> bool {
        self.role_ids().iter().any(|id| id == role_id)
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
        assert_eq!(guild.id, "");
        assert_eq!(guild.name, "");
        assert!(!guild.is_owned_by_current_user());
        assert_eq!(guild.get_member_count(), 0);
        assert_eq!(guild.get_max_members(), 0);
    }

    #[test]
    fn test_guild_with_data() {
        let mut guild = Guild::new();
        guild.id = "123456789".to_string();
        guild.name = "Test Guild".to_string();
        guild.is_owner = true;
        guild.member_count = 100;
        guild.max_members = 500;
        guild.description = "A test guild".to_string();

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

        assert_eq!(guild.id, "guild-1");
        assert!(guild.is_owner);
        assert_eq!(guild.channels.len(), 1);
        assert_eq!(guild.channels[0].id, "channel-1");
        assert_eq!(guild.union_world_id, "world-1");
        assert_eq!(guild.union_org_id, "org-1");
        assert_eq!(guild.op_user_id, "operator-1");

        let value = serde_json::to_value(&guild).unwrap();
        assert_eq!(value["owner"], serde_json::json!(true));
        assert!(value.get("is_owner").is_none());
        assert_eq!(value["channels"][0]["id"], serde_json::json!("channel-1"));
        assert_eq!(value["union_world_id"], serde_json::json!("world-1"));
    }

    #[test]
    fn botgo_guild_uses_required_zero_value_fields() {
        let guild: Guild = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(guild.id, "");
        assert_eq!(guild.name, "");
        assert_eq!(guild.icon, "");
        assert_eq!(guild.owner_id, "");
        assert!(!guild.is_owner);
        assert_eq!(guild.member_count, 0);
        assert_eq!(guild.max_members, 0);
        assert_eq!(guild.description, "");
        assert_eq!(guild.joined_at, "");
        assert!(guild.channels.is_empty());
        assert_eq!(guild.union_world_id, "");
        assert_eq!(guild.union_org_id, "");
        assert_eq!(guild.op_user_id, "");
    }

    #[test]
    fn test_member_limit() {
        let mut guild = Guild::new();
        guild.member_count = 500;
        guild.max_members = 500;
        assert!(guild.is_at_member_limit());

        guild.member_count = 499;
        assert!(!guild.is_at_member_limit());

        guild.member_count = 501;
        assert!(guild.is_at_member_limit());
    }

    #[test]
    fn test_icon_url() {
        let mut guild = Guild::new();
        assert!(guild.icon_url().is_none());

        guild.id = "123456789".to_string();
        guild.icon = "abc123".to_string();
        let url = guild.icon_url().unwrap();
        assert!(url.contains("123456789"));
        assert!(url.contains("abc123"));
    }

    #[test]
    fn test_role_creation() {
        let role = Role::new();
        assert_eq!(role.id, "");
        assert_eq!(role.name, "");
        assert!(!role.is_hoisted());
        assert_eq!(role.member_count(), 0);
    }

    #[test]
    fn test_role_with_data() {
        let mut role = Role::new();
        role.id = "role123".to_string();
        role.name = "Admin".to_string();
        role.color = 0xFF0000;
        role.hoist = 1;
        role.member_count = 5;
        role.member_limit = 10;

        assert_eq!(role.id(), Some(&"role123".to_string()));
        assert_eq!(role.name(), "Admin");
        assert_eq!(role.color_hex(), Some("#FF0000".to_string()));
        assert!(role.is_hoisted());
        assert_eq!(role.member_count(), 5);
        assert_eq!(role.get_member_limit(), 10);
        assert!(!role.is_at_member_limit());
    }

    #[test]
    fn botgo_role_keeps_official_json_shape() {
        let role = GuildRole {
            id: "role-1".to_string(),
            name: "Admin".to_string(),
            color: 0xFF0000,
            hoist: 1,
            member_count: 5,
            member_limit: 10,
        };
        let value = serde_json::to_value(&role).unwrap();

        assert_eq!(value["id"], "role-1");
        assert_eq!(value["name"], "Admin");
        assert_eq!(value["color"], 0xFF0000);
        assert_eq!(value["hoist"], 1);
        assert_eq!(value["number"], 5);
        assert_eq!(value["member_limit"], 10);

        let roles = GuildRoles {
            guild_id: "guild-1".to_string(),
            roles: vec![role],
            num_limit: "30".to_string(),
        };
        let value = serde_json::to_value(&roles).unwrap();
        assert_eq!(value["guild_id"], "guild-1");
        assert_eq!(value["role_num_limit"], "30");
        assert!(value.get("num_limit").is_none());
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
        assert_eq!(member.nick, "");
        assert_eq!(member.role_ids().len(), 0);
    }

    #[test]
    fn test_member_with_roles() {
        let mut member = Member::new();
        member.roles = vec!["role1".to_string(), "role2".to_string()];

        assert!(member.has_role("role1"));
        assert!(member.has_role("role2"));
        assert!(!member.has_role("role3"));
        assert_eq!(member.role_ids().len(), 2);
    }

    #[test]
    fn botgo_member_uses_required_zero_value_fields() {
        let member: Member = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(member.guild_id, "");
        assert!(member.user.is_none());
        assert_eq!(member.nick, "");
        assert!(member.roles.is_empty());
        assert_eq!(member.joined_at, "");
        assert_eq!(member.op_user_id, "");

        let value = serde_json::to_value(&member).unwrap();
        assert_eq!(value["guild_id"], "");
        assert_eq!(value["nick"], "");
        assert_eq!(value["roles"], serde_json::json!([]));
        assert_eq!(value["joined_at"], "");
        assert!(value.get("op_user_id").is_none());
    }
}
