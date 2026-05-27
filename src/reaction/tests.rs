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
    assert_eq!(
        ReactionTargetType::from(99),
        ReactionTargetType::Unknown(99)
    );
    assert_eq!(i32::from(ReactionTargetType::Unknown(99)), 99);
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
    let user: crate::models::User = serde_json::from_value(data).unwrap();
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
fn message_reaction_uses_required_zero_value_fields() {
    let reaction: MessageReaction = serde_json::from_value(serde_json::json!({})).unwrap();

    assert_eq!(reaction.user_id, "");
    assert_eq!(reaction.channel_id, "");
    assert_eq!(reaction.guild_id, "");
    assert_eq!(reaction.target.id, "");
    assert_eq!(reaction.target.target_type, ReactionTargetType::Message);
    assert_eq!(reaction.emoji.id, "");
    assert_eq!(reaction.emoji.emoji_type, 0);

    let value = serde_json::to_value(&reaction).unwrap();
    assert_eq!(value["user_id"], "");
    assert_eq!(value["channel_id"], "");
    assert_eq!(value["guild_id"], "");
    assert_eq!(value["target"]["id"], "");
    assert_eq!(value["target"]["type"], 0);
    assert_eq!(value["emoji"]["id"], "");
    assert_eq!(value["emoji"]["type"], 0);
}

#[test]
fn reaction_event_id_is_internal_only() {
    let http = crate::http::HttpClient::new(30, false).unwrap();
    let api = BotApi::new_for_test(http);
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
fn reaction_users_omits_empty_response_fields() {
    let empty = serde_json::to_value(ReactionUsers::default()).unwrap();
    assert_eq!(empty, serde_json::json!({}));

    let users: ReactionUsers = serde_json::from_value(serde_json::json!({
        "cookie": "cursor-1",
        "is_end": true,
        "users": [{
            "id": "user-1",
            "username": "tester",
            "avatar": "https://example.com/avatar.png"
        }]
    }))
    .unwrap();

    assert_eq!(users.cookie.as_deref(), Some("cursor-1"));
    assert!(users.is_end);
    assert_eq!(users.users.len(), 1);

    let value = serde_json::to_value(&users).unwrap();
    assert_eq!(value["cookie"], "cursor-1");
    assert_eq!(value["is_end"], true);
    assert_eq!(value["users"][0]["id"], "user-1");
}

#[test]
fn reaction_pager_query_params() {
    let pager = MessageReactionPager::new(Some("cursor-1"), Some(20));
    let query = pager.query_params();

    assert_eq!(query.get("cookie").map(String::as_str), Some("cursor-1"));
    assert_eq!(query.get("limit").map(String::as_str), Some("20"));
}

#[test]
fn reaction_pager_omits_empty_query_params() {
    let pager = MessageReactionPager::new(Some(""), Some(""));
    assert!(pager.query_params().is_empty());
}
