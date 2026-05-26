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
fn test_remind_type_description() {
    assert_eq!(RemindType::None.description(), "No reminder");
    assert_eq!(
        RemindType::Before30Minutes.description(),
        "30 minutes before"
    );
    assert_eq!(RemindType::Before1Day.description(), "1 day before");
}

#[test]
fn test_remind_type_minutes_before() {
    assert_eq!(RemindType::None.minutes_before(), None);
    assert_eq!(RemindType::OnStart.minutes_before(), None);
    assert_eq!(RemindType::Before5Minutes.minutes_before(), Some(5));
    assert_eq!(RemindType::Before1Hour.minutes_before(), Some(60));
    assert_eq!(RemindType::Before1Day.minutes_before(), Some(24 * 60));
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
    assert!(schedule.has_reminder());
    assert!(schedule.has_jump_channel());
}

#[test]
fn test_schedule_with_description() {
    let schedule = Schedule::new(
        "Daily Standup",
        "1640995200",
        "1640996400",
        None,
        RemindType::Before5Minutes,
    )
    .with_description("Daily team standup meeting");

    assert_eq!(
        schedule.description,
        "Daily team standup meeting".to_string()
    );
}

#[test]
fn test_schedule_duration() {
    let schedule = Schedule::new(
        "Test Event",
        "1640995200", // Start
        "1640998800", // End (1 hour later)
        None,
        RemindType::None,
    );

    assert_eq!(schedule.duration_seconds(), Some(3600)); // 1 hour = 3600 seconds
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

    assert!(!schedule.has_reminder());
    assert_eq!(schedule.reminder_description(), "No reminder");
}

#[test]
fn test_schedule_display() {
    let schedule = Schedule::new(
        "Test Meeting",
        "1640995200",
        "1640998800",
        Some("channel456".to_string()),
        RemindType::Before30Minutes,
    );

    let display = format!("{}", schedule);
    assert!(display.contains("Test Meeting"));
    assert!(display.contains("1640995200"));
    assert!(display.contains("30 minutes before"));
}

#[test]
fn test_schedule_timestamp_parsing() {
    let schedule = Schedule::new(
        "Parse Test",
        "1640995200",
        "invalid_timestamp",
        None,
        RemindType::None,
    );

    assert!(schedule.start_timestamp_parsed().is_ok());
    assert!(schedule.end_timestamp_parsed().is_err());
    assert_eq!(schedule.duration_seconds(), None);
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
    assert!(!schedule.has_jump_channel());
    assert!(!schedule.has_reminder());

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
    let wrapper = ScheduleWrapper::empty();
    let value = serde_json::to_value(&wrapper).unwrap();

    assert!(value.as_object().unwrap().is_empty());
}
