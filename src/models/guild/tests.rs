use super::*;

#[test]
fn test_guild_creation() {
    let guild = Guild::default();
    assert_eq!(guild.id, "");
    assert_eq!(guild.name, "");
    assert!(!guild.is_owner);
    assert_eq!(guild.member_count, 0);
    assert_eq!(guild.max_members, 0);
}

#[test]
fn test_guild_with_data() {
    let guild = Guild {
        id: "123456789".to_string(),
        name: "Test Guild".to_string(),
        is_owner: true,
        member_count: 100,
        max_members: 500,
        description: "A test guild".to_string(),
        ..Default::default()
    };

    assert_eq!(guild.id, "123456789");
    assert_eq!(guild.name, "Test Guild");
    assert!(guild.is_owner);
    assert_eq!(guild.member_count, 100);
    assert_eq!(guild.max_members, 500);
    assert!(i64::from(guild.member_count) < guild.max_members);
    assert!(!guild.description.is_empty());
}

#[test]
fn guild_fields_use_official_json_names() {
    let guild: Guild = serde_json::from_value(serde_json::json!({
            "id": "guild-1",
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
    }))
    .unwrap();

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
fn guild_deserialization_does_not_use_gateway_event_id_as_guild_id() {
    let guild: Guild = serde_json::from_value(serde_json::json!({
            "name": "Guild"
    }))
    .unwrap();

    assert_eq!(guild.id, "");
}

#[test]
fn guild_uses_required_zero_value_fields() {
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

    let value = serde_json::to_value(&guild).unwrap();
    assert!(value.get("op_user_id").is_none());
}

#[test]
fn test_member_limit() {
    let mut guild = Guild {
        member_count: 500,
        max_members: 500,
        ..Default::default()
    };
    assert!(i64::from(guild.member_count) >= guild.max_members);

    guild.member_count = 499;
    assert!(i64::from(guild.member_count) < guild.max_members);

    guild.member_count = 501;
    assert!(i64::from(guild.member_count) >= guild.max_members);
}

#[test]
fn test_role_creation() {
    let role = GuildRole::default();
    assert_eq!(role.id, "");
    assert_eq!(role.name, "");
    assert_eq!(role.hoist, 0);
    assert_eq!(role.member_count, 0);
}

#[test]
fn test_role_with_data() {
    let role = GuildRole {
        id: "role123".to_string(),
        name: "Admin".to_string(),
        color: 0xFF0000,
        hoist: 1,
        member_count: 5,
        member_limit: 10,
    };

    assert_eq!(role.id, "role123");
    assert_eq!(role.name, "Admin");
    assert_eq!(role.color, 0xFF0000);
    assert_eq!(role.hoist, 1);
    assert_eq!(role.member_count, 5);
    assert_eq!(role.member_limit, 10);
    assert!(role.member_count < role.member_limit);
}

#[test]
fn role_keeps_official_json_shape() {
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
fn update_role_new_matches_request_body() {
    let body = UpdateRole::new(
        "guild-1",
        GuildRole {
            name: "Admin".to_string(),
            color: 0,
            hoist: 1,
            ..Default::default()
        },
    );

    assert_eq!(body.update.color, DEFAULT_ROLE_COLOR);
    assert_eq!(
        serde_json::to_value(&body).unwrap(),
        serde_json::json!({
            "guild_id": "guild-1",
            "filter": {
                "name": 1,
                "color": 1,
                "hoist": 1
            },
            "info": {
                "name": "Admin",
                "color": DEFAULT_ROLE_COLOR,
                "hoist": 1
            }
        })
    );
}

#[test]
fn update_guild_mute_uses_zero_value_omitempty_shape() {
    let empty = serde_json::to_value(UpdateGuildMute::default()).unwrap();
    assert_eq!(empty, serde_json::json!({}));

    let single = UpdateGuildMute::new(Some("1710000000"), None);
    let value = serde_json::to_value(&single).unwrap();
    assert_eq!(single.mute_end_timestamp, "1710000000");
    assert_eq!(single.mute_seconds, "");
    assert!(single.user_ids.is_empty());
    assert_eq!(
        value,
        serde_json::json!({"mute_end_timestamp": "1710000000"})
    );

    let cancel = UpdateGuildMute::cancel_multi(vec!["user-1".to_string()]);
    let value = serde_json::to_value(&cancel).unwrap();
    assert_eq!(
        value,
        serde_json::json!({
            "mute_end_timestamp": "0",
            "mute_seconds": "0",
            "user_ids": ["user-1"]
        })
    );
}

#[test]
fn update_guild_mute_response_omits_empty_user_ids() {
    let empty = serde_json::to_value(UpdateGuildMuteResponse::default()).unwrap();
    assert_eq!(empty, serde_json::json!({}));

    let response = UpdateGuildMuteResponse {
        user_ids: vec!["user-1".to_string()],
    };
    assert_eq!(
        serde_json::to_value(&response).unwrap(),
        serde_json::json!({
            "user_ids": ["user-1"]
        })
    );
}

#[test]
fn pager_query_params_match_official_priority() {
    let members = GuildMembersPager {
        after: Some("user-1".to_string()),
        limit: Some("100".to_string()),
    };
    assert_eq!(
        members.query_params().get("after").map(String::as_str),
        Some("user-1")
    );

    let role_members = GuildRoleMembersPager {
        start_index: Some("next-1".to_string()),
        limit: Some("50".to_string()),
    };
    assert_eq!(
        role_members
            .query_params()
            .get("start_index")
            .map(String::as_str),
        Some("next-1")
    );

    let guilds = GuildPager {
        before: Some("before-1".to_string()),
        after: Some("after-1".to_string()),
        limit: Some("20".to_string()),
    };
    let query = guilds.query_params();
    assert_eq!(query.get("after").map(String::as_str), Some("after-1"));
    assert!(!query.contains_key("before"));
}

#[test]
fn test_member_creation() {
    let member = Member::default();
    assert!(member.user.is_none());
    assert_eq!(member.nick, "");
    assert!(member.roles.is_empty());
}

#[test]
fn test_member_with_roles() {
    let member = Member {
        roles: vec!["role1".to_string(), "role2".to_string()],
        ..Default::default()
    };

    assert!(member.roles.iter().any(|id| id == "role1"));
    assert!(member.roles.iter().any(|id| id == "role2"));
    assert!(!member.roles.iter().any(|id| id == "role3"));
    assert_eq!(member.roles.len(), 2);
}

#[test]
fn member_uses_required_zero_value_fields() {
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

#[test]
fn member_add_role_body_matches_json_shape() {
    let empty = serde_json::to_value(MemberAddRoleBody::default()).unwrap();
    assert_eq!(empty, serde_json::json!({"channel": null}));

    let with_channel = serde_json::to_value(MemberAddRoleBody {
        channel: Some(crate::models::channel::Channel {
            id: "channel-1".to_string(),
            ..Default::default()
        }),
    })
    .unwrap();
    assert_eq!(
        with_channel,
        serde_json::json!({
            "channel": {
                "id": "channel-1",
                "guild_id": ""
            }
        })
    );
}

#[test]
fn member_delete_options_match_query_shape() {
    let options = MemberDeleteOptions {
        add_blacklist: true,
        delete_history_msg_days: 42,
    };

    assert_eq!(options.delete_history_msg_days, 42);
    assert_eq!(
        serde_json::to_value(&options).unwrap(),
        serde_json::json!({
            "add_blacklist": true,
            "delete_history_msg_days": 42
        })
    );

    assert_eq!(normalize_delete_history_msg_days(42), NO_DELETE);
}
