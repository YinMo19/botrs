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
