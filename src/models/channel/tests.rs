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
