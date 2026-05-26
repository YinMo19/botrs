use super::*;

#[test]
fn test_manage_event_type_from_str() {
    assert_eq!(
        "group_add_robot".parse::<ManageEventType>(),
        Ok(ManageEventType::GroupAddRobot)
    );
    assert_eq!(
        "friend_add".parse::<ManageEventType>(),
        Ok(ManageEventType::FriendAdd)
    );
    assert_eq!("invalid".parse::<ManageEventType>(), Err(()));
}

#[test]
fn test_manage_event_type_as_str() {
    assert_eq!(ManageEventType::GroupAddRobot.as_str(), "group_add_robot");
    assert_eq!(ManageEventType::FriendAdd.as_str(), "friend_add");
}

#[test]
fn test_is_group_event() {
    assert!(ManageEventType::GroupAddRobot.is_group_event());
    assert!(!ManageEventType::FriendAdd.is_group_event());
}

#[test]
fn test_is_c2c_event() {
    assert!(ManageEventType::FriendAdd.is_c2c_event());
    assert!(!ManageEventType::GroupAddRobot.is_c2c_event());
}

#[test]
fn enter_aio_uses_zero_value_omitempty_shape() {
    let event = EnterAioEvent::new(
        Some("event-1".to_string()),
        &serde_json::json!({
            "user_openid": "user-1",
            "from_source": "profile"
        }),
    );

    assert_eq!(event.user_openid, "user-1");
    assert_eq!(event.from_source, "profile");
    assert_eq!(event.event_id.as_deref(), Some("event-1"));

    let value = serde_json::to_value(&event).unwrap();
    assert_eq!(
        value,
        serde_json::json!({
            "user_openid": "user-1",
            "from_source": "profile"
        })
    );

    let empty = serde_json::to_value(EnterAioEvent::default()).unwrap();
    assert_eq!(empty, serde_json::json!({}));
}

#[test]
fn subscribe_message_status_uses_required_zero_value_fields() {
    let event = SubscribeMessageStatusData::new(
        Some("event-1".to_string()),
        &serde_json::json!({
            "group_openid": "group-1",
            "openid": "user-1",
            "result": [{
                "template_id": 1,
                "custom_template_id": "custom-1",
                "op": 2,
                "subscribe_id": "sub-1",
                "update_ts": 1710000000
            }]
        }),
    );

    assert_eq!(event.group_openid, "group-1");
    assert_eq!(event.openid, "user-1");
    assert_eq!(event.event_id.as_deref(), Some("event-1"));
    assert_eq!(event.result[0].template_id, 1);
    assert_eq!(event.result[0].custom_template_id, "custom-1");
    assert_eq!(event.result[0].op, 2);
    assert_eq!(event.result[0].subscribe_id, "sub-1");
    assert_eq!(event.result[0].update_ts, 1_710_000_000);

    let value = serde_json::to_value(&event).unwrap();
    assert_eq!(value["group_openid"], "group-1");
    assert_eq!(value["openid"], "user-1");
    assert_eq!(value["result"][0]["template_id"], 1);
    assert_eq!(value["result"][0]["custom_template_id"], "custom-1");
    assert_eq!(value["result"][0]["op"], 2);
    assert_eq!(value["result"][0]["subscribe_id"], "sub-1");
    assert_eq!(value["result"][0]["update_ts"], 1_710_000_000_u64);
    assert!(value.get("event_id").is_none());

    let empty = serde_json::to_value(SubscribeMessageStatusData::default()).unwrap();
    assert_eq!(empty["group_openid"], "");
    assert_eq!(empty["openid"], "");
    assert_eq!(empty["result"], serde_json::json!([]));
}
