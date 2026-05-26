use super::*;
use crate::BotApi;

#[test]
fn test_reaction_target_type() {
    assert_eq!(i32::from(ReactionTargetType::Message), 0);
    assert_eq!(i32::from(ReactionTargetType::Post), 1);
    assert_eq!(i32::from(ReactionTargetType::Comment), 2);
    assert_eq!(i32::from(ReactionTargetType::Reply), 3);
}

#[test]
fn test_reaction_target_type_from() {
    assert_eq!(ReactionTargetType::from(0), ReactionTargetType::Message);
    assert_eq!(ReactionTargetType::from(1), ReactionTargetType::Post);
    assert_eq!(ReactionTargetType::from(2), ReactionTargetType::Comment);
    assert_eq!(ReactionTargetType::from(3), ReactionTargetType::Reply);
    assert_eq!(ReactionTargetType::from(99), ReactionTargetType::Message); // Default fallback
}

#[test]
fn test_emoji_creation() {
    let emoji = Emoji::new("emoji123", 1);
    assert_eq!(emoji.id, "emoji123");
    assert_eq!(emoji.emoji_type, 1);
}

#[test]
fn test_reaction_target_creation() {
    let target = ReactionTarget::new("target123", ReactionTargetType::Message);
    assert_eq!(target.id, "target123");
    assert_eq!(target.target_type, ReactionTargetType::Message);
}

#[test]
fn test_reaction_user_creation() {
    let data = serde_json::json!({
        "id": "user123",
        "username": "testuser",
        "avatar": "https://example.com/avatar.png"
    });
    let user: ReactionUser = serde_json::from_value(data).unwrap();
    assert_eq!(user.id, "user123");
    assert_eq!(user.username, "testuser");
    assert_eq!(user.avatar, "https://example.com/avatar.png");
}

#[test]
fn message_reaction_keeps_official_dto_shape() {
    let reaction = MessageReaction::new(
        "user-1",
        "channel-1",
        "guild-1",
        ReactionTarget::new("message-1", ReactionTargetType::Message),
        Emoji::new("43", 1),
    );
    let value = serde_json::to_value(&reaction).unwrap();

    assert_eq!(value["user_id"], "user-1");
    assert_eq!(value["channel_id"], "channel-1");
    assert_eq!(value["guild_id"], "guild-1");
    assert_eq!(value["target"]["id"], "message-1");
    assert_eq!(value["target"]["type"], 0);
    assert_eq!(value["emoji"]["id"], "43");
    assert_eq!(value["emoji"]["type"], 1);
    assert!(value.get("event_id").is_none());
}

#[test]
fn reaction_event_id_is_internal_only() {
    let http = crate::http::HttpClient::new(30, false).unwrap();
    let api = BotApi::new(http);
    let reaction = Reaction::from_message_reaction(
        api,
        Some("event-1".to_string()),
        MessageReaction::new(
            "user-1",
            "channel-1",
            "guild-1",
            ReactionTarget::new("message-1", ReactionTargetType::Message),
            Emoji::new("43", 1),
        ),
    );

    assert_eq!(reaction.event_id.as_deref(), Some("event-1"));
    let value = serde_json::to_value(&reaction).unwrap();
    assert!(value.get("event_id").is_none());
}

#[test]
fn reaction_pager_query_params() {
    let pager = MessageReactionPager::new(Some("cursor-1"), Some(20));
    let query = pager.QueryParams();

    assert_eq!(query.get("cookie").map(String::as_str), Some("cursor-1"));
    assert_eq!(query.get("limit").map(String::as_str), Some("20"));
}
