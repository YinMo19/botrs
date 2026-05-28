use super::*;

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

#[test]
fn c2c_friend_data_uses_required_zero_value_fields() {
    let friend: C2CFriendData = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(friend.openid, "");
    assert_eq!(friend.timestamp, 0);
    assert_eq!(friend.nick, "");
    assert_eq!(friend.avatar, "");

    let value = serde_json::to_value(&friend).unwrap();
    assert_eq!(value["openid"], "");
    assert_eq!(value["timestamp"], 0);
    assert_eq!(value["nick"], "");
    assert_eq!(value["avatar"], "");
}

#[test]
fn manage_event_ids_are_internal_only() {
    let group = GroupManageEvent::new(
        Some("event-1".to_string()),
        &serde_json::json!({
            "timestamp": 1710000000_u64,
            "group_openid": "group-1",
            "op_member_openid": "member-1"
        }),
    );
    let c2c = C2CManageEvent::new(
        Some("event-2".to_string()),
        &serde_json::json!({
            "timestamp": 1710000001_u64,
            "openid": "user-1"
        }),
    );

    assert_eq!(group.event_id.as_deref(), Some("event-1"));
    assert_eq!(c2c.event_id.as_deref(), Some("event-2"));

    let group_value = serde_json::to_value(&group).unwrap();
    let c2c_value = serde_json::to_value(&c2c).unwrap();
    assert!(group_value.get("event_id").is_none());
    assert!(c2c_value.get("event_id").is_none());
}
