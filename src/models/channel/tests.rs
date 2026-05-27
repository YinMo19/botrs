use super::*;

#[test]
fn test_channel_creation() {
    let channel = Channel::new();
    assert!(channel.id.is_empty());
    assert!(channel.name.is_empty());
    assert!(channel.is_public()); // Default should be public
}

#[test]
fn test_channel_types() {
    let mut channel = Channel::new();

    channel.channel_type = ChannelType::Text;
    assert!(channel.is_text());
    assert!(!channel.is_voice());

    channel.channel_type = ChannelType::Voice;
    assert!(channel.is_voice());
    assert!(!channel.is_text());

    channel.channel_type = ChannelType::Category;
    assert!(channel.is_group());
}

#[test]
fn test_channel_type_conversion() {
    assert_eq!(ChannelType::from(0), ChannelType::Text);
    assert_eq!(u32::from(ChannelType::Text), 0);

    assert_eq!(ChannelType::from(10005), ChannelType::Live);
    assert_eq!(u32::from(ChannelType::Live), 10005);

    assert_eq!(ChannelType::from(99999), ChannelType::Unknown(99999));
    assert_eq!(u32::from(ChannelType::Unknown(99999)), 99999);
}

#[test]
fn test_private_types() {
    let mut channel = Channel::new();

    channel.private_type = PrivateType::Public;
    assert!(channel.is_public());
    assert!(!channel.is_admin_only());

    channel.private_type = PrivateType::OnlyAdmin;
    assert!(!channel.is_public());
    assert!(channel.is_admin_only());

    channel.private_type = PrivateType::AdminAndMember;
    assert!(channel.is_specified_users_only());
}

#[test]
fn test_speak_permissions() {
    let mut channel = Channel::new();

    channel.speak_permission = SpeakPermission::Public;
    assert!(channel.everyone_can_speak());
    assert!(!channel.admin_only_speak());

    channel.speak_permission = SpeakPermission::AdminAndMember;
    assert!(!channel.everyone_can_speak());
    assert!(channel.admin_only_speak());
}

#[test]
fn test_channel_mention() {
    let mut channel = Channel::new();
    channel.id = "123456789".to_string();
    assert_eq!(channel.mention(), "<#123456789>");
}

#[test]
fn test_channel_permissions() {
    let mut perms = ChannelPermissions::new();
    assert!(!perms.is_user_permission());

    perms.user_id = "user123".to_string();
    assert!(perms.is_user_permission());

    let role_perms = ChannelRolesPermissions {
        role_id: "role123".to_string(),
        ..Default::default()
    };
    assert!(role_perms.is_role_permission());
}

#[test]
fn channel_uses_zero_values_for_missing_fields() {
    let channel: Channel = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(channel.id, "");
    assert_eq!(channel.guild_id, "");
    assert_eq!(channel.name, "");
    assert_eq!(channel.channel_type, ChannelType::Text);
    assert_eq!(channel.sub_type, ChannelSubType::Chat);
    assert_eq!(channel.private_type, PrivateType::Public);
    assert_eq!(channel.speak_permission, SpeakPermission::Invalid);
    assert!(channel.private_user_ids.is_empty());

    let value = serde_json::to_value(&channel).unwrap();
    assert_eq!(value["id"], serde_json::json!(""));
    assert_eq!(value["guild_id"], serde_json::json!(""));
    assert!(value.get("name").is_none());
    assert!(value.get("type").is_none());
    assert!(value.get("sub_type").is_none());
    assert!(value.get("position").is_none());
    assert!(value.get("private_user_ids").is_none());
}

#[test]
fn channel_decodes_large_type_values() {
    let channel: Channel = serde_json::from_value(serde_json::json!({
        "id": "channel-1",
        "guild_id": "guild-1",
        "name": "live",
        "type": 10005,
        "sub_type": 3,
        "private_type": 2,
        "speak_permission": 1,
        "private_user_ids": ["user-1"],
        "permissions": "2048"
    }))
    .unwrap();

    assert_eq!(channel.id, "channel-1");
    assert_eq!(channel.guild_id, "guild-1");
    assert_eq!(channel.channel_type, ChannelType::Live);
    assert_eq!(channel.sub_type, ChannelSubType::TeamGame);
    assert_eq!(channel.private_type, PrivateType::AdminAndMember);
    assert_eq!(channel.speak_permission, SpeakPermission::Public);
    assert_eq!(channel.private_user_ids, ["user-1"]);
    assert_eq!(channel.permissions, "2048");
}

#[test]
fn channel_from_data_does_not_use_gateway_event_id_as_channel_id() {
    let channel = Channel::from_data(
        "event-1".to_string(),
        serde_json::json!({
            "guild_id": "guild-1",
            "name": "general"
        }),
    );

    assert_eq!(channel.id, "");
    assert_eq!(channel.guild_id, "guild-1");
}

#[test]
fn channel_permissions_are_separate_dtos() {
    let user_permissions: ChannelPermissions = serde_json::from_value(serde_json::json!({
        "channel_id": "channel-1",
        "user_id": "user-1",
        "permissions": "1024"
    }))
    .unwrap();
    let role_permissions: ChannelRolesPermissions = serde_json::from_value(serde_json::json!({
        "channel_id": "channel-1",
        "role_id": "role-1",
        "permissions": "2048"
    }))
    .unwrap();

    assert_eq!(user_permissions.user_id, "user-1");
    assert_eq!(user_permissions.permissions, "1024");
    assert_eq!(role_permissions.role_id, "role-1");
    assert_eq!(role_permissions.permissions, "2048");
}

#[test]
fn channel_value_object_omits_go_zero_values() {
    let value = ChannelValueObject {
        name: Some(String::new()),
        channel_type: Some(ChannelType::Text),
        position: Some(0),
        parent_id: Some(String::new()),
        owner_id: Some(String::new()),
        sub_type: Some(ChannelSubType::Chat),
        private_type: Some(PrivateType::Public),
        private_user_ids: Some(Vec::new()),
        speak_permission: Some(SpeakPermission::Invalid),
        application_id: Some(String::new()),
        permissions: Some(String::new()),
        op_user_id: Some(String::new()),
    };

    assert_eq!(serde_json::to_value(&value).unwrap(), serde_json::json!({}));
}

#[test]
fn channel_value_object_keeps_non_zero_values() {
    let value = ChannelValueObject {
        name: Some("name".to_string()),
        channel_type: Some(ChannelType::Voice),
        position: Some(1),
        parent_id: Some("parent".to_string()),
        owner_id: Some("owner".to_string()),
        sub_type: Some(ChannelSubType::Notice),
        private_type: Some(PrivateType::AdminAndMember),
        private_user_ids: Some(vec!["user".to_string()]),
        speak_permission: Some(SpeakPermission::Public),
        application_id: Some("app".to_string()),
        permissions: Some("1".to_string()),
        op_user_id: Some("op".to_string()),
    };

    assert_eq!(
        serde_json::to_value(&value).unwrap(),
        serde_json::json!({
            "name": "name",
            "type": 2,
            "position": 1,
            "parent_id": "parent",
            "owner_id": "owner",
            "sub_type": 1,
            "private_type": 2,
            "private_user_ids": ["user"],
            "speak_permission": 1,
            "application_id": "app",
            "permissions": "1",
            "op_user_id": "op"
        })
    );
}

#[test]
fn channel_permissions_omit_empty_fields() {
    let user_permissions = ChannelPermissions::default();
    let role_permissions = ChannelRolesPermissions::default();
    let update_permissions = UpdateChannelPermissions {
        add: Some(String::new()),
        remove: Some(String::new()),
    };

    assert_eq!(
        serde_json::to_value(&user_permissions).unwrap(),
        serde_json::json!({})
    );
    assert_eq!(
        serde_json::to_value(&role_permissions).unwrap(),
        serde_json::json!({})
    );
    assert_eq!(
        serde_json::to_value(&update_permissions).unwrap(),
        serde_json::json!({})
    );
}

#[test]
fn channel_permissions_validate_only_non_empty_values() {
    let update_permissions = UpdateChannelPermissions {
        add: Some(String::new()),
        remove: Some(String::new()),
    };
    assert!(update_permissions.validate().is_ok());

    let update_permissions = UpdateChannelPermissions::new(Some("7"), Some("0"));
    assert!(update_permissions.validate().is_ok());

    let update_permissions = UpdateChannelPermissions::new(Some("not-a-number"), None::<&str>);
    assert!(update_permissions.validate().is_err());
}
