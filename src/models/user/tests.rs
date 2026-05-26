use super::*;

#[test]
fn test_user_creation() {
    let user = User::new("123456789", "TestUser");
    assert_eq!(user.id, "123456789");
    assert_eq!(user.username, "TestUser");
    assert_eq!(user.avatar, "");
    assert_eq!(user.union_openid, "");
    assert_eq!(user.union_user_account, "");
    assert!(!user.is_bot());
    assert!(user.is_human());
}

#[test]
fn user_uses_required_zero_value_fields() {
    let user: User = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(user.id, "");
    assert_eq!(user.username, "");
    assert_eq!(user.avatar, "");
    assert!(!user.bot);
    assert_eq!(user.union_openid, "");
    assert_eq!(user.union_user_account, "");
    assert!(user.avatar_url().is_none());
}

#[test]
fn user_keeps_official_json_shape() {
    let user = User {
        id: "user-1".to_string(),
        username: "alice".to_string(),
        avatar: "avatar-key".to_string(),
        bot: true,
        union_openid: "union-openid".to_string(),
        union_user_account: "union-account".to_string(),
    };
    let value = serde_json::to_value(&user).unwrap();

    assert_eq!(value["id"], "user-1");
    assert_eq!(value["username"], "alice");
    assert_eq!(value["avatar"], "avatar-key");
    assert_eq!(value["bot"], true);
    assert_eq!(value["union_openid"], "union-openid");
    assert_eq!(value["union_user_account"], "union-account");
    assert!(user.avatar_url().is_some());
}

#[test]
fn user_from_bot_info_preserves_union_fields() {
    let user = User::from(crate::models::api::BotInfo {
        id: "bot-1".to_string(),
        username: "bot".to_string(),
        avatar: "avatar-key".to_string(),
        bot: true,
        union_openid: "UNION_OPENID_XXXXXX".to_string(),
        union_user_account: "UNION_ACCOUNT_XXXXXX".to_string(),
        share_url: "https://example.com/share".to_string(),
        welcome_msg: "hello".to_string(),
    });

    assert_eq!(user.id, "bot-1");
    assert_eq!(user.username, "bot");
    assert_eq!(user.avatar, "avatar-key");
    assert!(user.bot);
    assert_eq!(user.union_openid, "UNION_OPENID_XXXXXX");
    assert_eq!(user.union_user_account, "UNION_ACCOUNT_XXXXXX");
}

#[test]
fn test_user_mention() {
    let user = User::new("123456789", "TestUser");
    assert_eq!(user.mention(), "<@!123456789>");
}

#[test]
fn test_member_display_name() {
    let user = User::new("123456789", "TestUser");
    let mut member = Member::new(user, "2024-01-01T00:00:00Z".to_string());

    // Without nickname, should return username
    assert_eq!(member.display_name(), "TestUser");

    // With nickname, should return nickname
    member.nick = Some("Nickname".to_string());
    assert_eq!(member.display_name(), "Nickname");
}

#[test]
fn test_member_roles() {
    let user = User::new("123456789", "TestUser");
    let mut member = Member::new(user, "2024-01-01T00:00:00Z".to_string());

    member.roles = vec!["role1".to_string(), "role2".to_string()];

    assert!(member.has_role(&"role1".to_string()));
    assert!(!member.has_role(&"role3".to_string()));

    assert!(member.has_any_role(&["role1".to_string(), "role3".to_string()]));
    assert!(member.has_all_roles(&["role1".to_string(), "role2".to_string()]));
    assert!(!member.has_all_roles(&["role1".to_string(), "role3".to_string()]));
}

#[test]
fn test_role_creation() {
    let role = Role::new("123456789", "TestRole");
    assert_eq!(role.id, "123456789");
    assert_eq!(role.name, "TestRole");
    assert_eq!(role.mention(), "<@&123456789>");
}

#[test]
fn test_role_color() {
    let mut role = Role::new("123456789", "TestRole");
    role.color = 0xFF5733; // Orange color

    let (r, g, b) = role.rgb();
    assert_eq!(r, 255);
    assert_eq!(g, 87);
    assert_eq!(b, 51);

    assert_eq!(role.hex_color(), "#FF5733");
}
