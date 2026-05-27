use super::*;

#[test]
fn test_remind_type_conversion() {
    assert_eq!(RemindType::from(0), RemindType::None);
    assert_eq!(RemindType::from(1), RemindType::OnStart);
    assert_eq!(RemindType::from(5), RemindType::Before1Hour);
    assert_eq!(u8::from(RemindType::Before15Minutes), 3);
    assert_eq!(u8::from(RemindType::Before1Day), 7);

    assert_eq!(RemindType::from(99), RemindType::Unknown(99));
    assert_eq!(u8::from(RemindType::Unknown(99)), 99);
}

#[test]
fn test_schedule_creation() {
    let schedule = Schedule::new(
        "Team Meeting",
        "1640995200",
        "1640998800",
        Some("channel123".to_string()),
        RemindType::Before15Minutes,
    );

    assert_eq!(schedule.name, "Team Meeting");
    assert_eq!(schedule.start_timestamp, "1640995200");
    assert_eq!(schedule.end_timestamp, "1640998800");
    assert_eq!(schedule.jump_channel_id, "channel123");
    assert_eq!(schedule.remind_type, "3");
}

#[test]
fn test_schedule_no_reminder() {
    let schedule = Schedule::new(
        "No Reminder Event",
        "1640995200",
        "1640998800",
        None,
        RemindType::None,
    );

    assert_eq!(schedule.remind_type, "0");
}

#[test]
fn schedule_uses_required_zero_value_fields() {
    let schedule: Schedule = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(schedule.id, "");
    assert_eq!(schedule.name, "");
    assert_eq!(schedule.description, "");
    assert_eq!(schedule.start_timestamp, "");
    assert_eq!(schedule.end_timestamp, "");
    assert_eq!(schedule.jump_channel_id, "");
    assert_eq!(schedule.remind_type, "");
    assert!(schedule.creator.is_none());

    let value = serde_json::to_value(&schedule).unwrap();
    assert!(value.as_object().unwrap().is_empty());
}

#[test]
fn schedule_keeps_official_json_shape() {
    let schedule = Schedule {
        id: "schedule-1".to_string(),
        name: "meeting".to_string(),
        description: "planning".to_string(),
        start_timestamp: "1640995200".to_string(),
        end_timestamp: "1640998800".to_string(),
        jump_channel_id: "channel-1".to_string(),
        remind_type: "3".to_string(),
        creator: None,
    };
    let value = serde_json::to_value(&schedule).unwrap();

    assert_eq!(value["id"], "schedule-1");
    assert_eq!(value["name"], "meeting");
    assert_eq!(value["description"], "planning");
    assert_eq!(value["start_timestamp"], "1640995200");
    assert_eq!(value["end_timestamp"], "1640998800");
    assert_eq!(value["jump_channel_id"], "channel-1");
    assert_eq!(value["remind_type"], "3");
}

#[test]
fn schedule_wrapper_allows_empty_zero_value_body() {
    let wrapper = ScheduleWrapper::default();
    let value = serde_json::to_value(&wrapper).unwrap();

    assert!(value.as_object().unwrap().is_empty());
}
