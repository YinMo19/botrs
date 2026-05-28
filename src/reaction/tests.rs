use super::*;
use crate::models::emoji::{Emoji, EmojiType};

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
    let emoji = Emoji {
        id: "emoji123".to_string(),
        emoji_type: EmojiType::System,
        ..Default::default()
    };
    assert_eq!(emoji.id, "emoji123");
    assert_eq!(emoji.emoji_type, EmojiType::System);
}

#[test]
fn test_reaction_target_creation() {
    let target = ReactionTarget {
        id: "target123".to_string(),
        target_type: ReactionTargetType::Message,
    };
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
    let reaction = MessageReaction {
        user_id: "user-1".to_string(),
        channel_id: "channel-1".to_string(),
        guild_id: "guild-1".to_string(),
        target: ReactionTarget {
            id: "message-1".to_string(),
            target_type: ReactionTargetType::Message,
        },
        emoji: Emoji {
            id: "43".to_string(),
            emoji_type: EmojiType::System,
            ..Default::default()
        },
    };
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
    assert_eq!(reaction.emoji.emoji_type, EmojiType::Unknown(0));

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
    let reaction = Reaction::from_message_reaction(
        Some("event-1".to_string()),
        MessageReaction {
            user_id: "user-1".to_string(),
            channel_id: "channel-1".to_string(),
            guild_id: "guild-1".to_string(),
            target: ReactionTarget {
                id: "message-1".to_string(),
                target_type: ReactionTargetType::Message,
            },
            emoji: Emoji {
                id: "43".to_string(),
                emoji_type: EmojiType::System,
                ..Default::default()
            },
        },
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
