use super::*;

#[test]
fn test_channel_creation() {
    let channel = Channel::default();
    assert!(channel.id.is_empty());
    assert!(channel.name.is_empty());
    assert_eq!(channel.private_type, PrivateType::Public);
}

#[test]
fn test_channel_types() {
    let mut channel = Channel {
        channel_type: ChannelType::Text,
        ..Default::default()
    };
    assert_eq!(channel.channel_type, ChannelType::Text);

    channel.channel_type = ChannelType::Voice;
    assert_eq!(channel.channel_type, ChannelType::Voice);

    channel.channel_type = ChannelType::Category;
    assert_eq!(channel.channel_type, ChannelType::Category);
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
    let mut channel = Channel {
        private_type: PrivateType::Public,
        ..Default::default()
    };
    assert_eq!(channel.private_type, PrivateType::Public);

    channel.private_type = PrivateType::OnlyAdmin;
    assert_eq!(channel.private_type, PrivateType::OnlyAdmin);

    channel.private_type = PrivateType::AdminAndMember;
    assert_eq!(channel.private_type, PrivateType::AdminAndMember);
}

#[test]
fn test_speak_permissions() {
    let mut channel = Channel {
        speak_permission: SpeakPermission::Public,
        ..Default::default()
    };
    assert_eq!(channel.speak_permission, SpeakPermission::Public);

    channel.speak_permission = SpeakPermission::AdminAndMember;
    assert_eq!(channel.speak_permission, SpeakPermission::AdminAndMember);
}

#[test]
fn test_channel_permissions() {
    let perms = ChannelPermissions {
        user_id: "user123".to_string(),
        ..Default::default()
    };
    assert_eq!(ChannelPermissions::default().user_id, "");
    assert_eq!(perms.user_id, "user123");

    let role_perms = ChannelRolesPermissions {
        role_id: "role123".to_string(),
        ..Default::default()
    };
    assert_eq!(role_perms.role_id, "role123");
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
fn channel_deserialization_does_not_use_gateway_event_id_as_channel_id() {
    let channel: Channel = serde_json::from_value(serde_json::json!({
            "guild_id": "guild-1",
            "name": "general"
    }))
    .unwrap();

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
