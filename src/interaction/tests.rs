use super::*;
use crate::BotApi;

#[test]
fn test_interaction_type() {
    assert_eq!(InteractionType::Ping as u8, 1);
    assert_eq!(InteractionType::ApplicationCommand as u8, 2);
    assert_eq!(InteractionType::HttpProxy as u8, 10);
    assert_eq!(InteractionType::InlineKeyboard as u8, 11);
}

#[test]
fn test_interaction_data_type() {
    assert_eq!(InteractionDataType::ChatInputSearch as u8, 9);
    assert_eq!(InteractionDataType::HttpProxy as u8, 10);
    assert_eq!(InteractionDataType::InlineKeyboardButtonClick as u8, 11);
    assert_eq!(InteractionDataType::CallbackCommandClick as u8, 12);
    assert_eq!(InteractionDataType::MessageFeedbackClick as u8, 13);
    assert_eq!(InteractionDataType::ClearSessionClick as u8, 14);
}

#[test]
fn test_interaction_type_from() {
    assert_eq!(InteractionType::from(1), InteractionType::Ping);
    assert_eq!(
        InteractionType::from(2),
        InteractionType::ApplicationCommand
    );
    assert_eq!(InteractionType::from(10), InteractionType::HttpProxy);
    assert_eq!(InteractionType::from(11), InteractionType::InlineKeyboard);
}

#[test]
fn test_interaction_data_type_from() {
    assert_eq!(
        InteractionDataType::from(9),
        InteractionDataType::ChatInputSearch
    );
    assert_eq!(
        InteractionDataType::from(10),
        InteractionDataType::HttpProxy
    );
    assert_eq!(
        InteractionDataType::from(11),
        InteractionDataType::InlineKeyboardButtonClick
    );
    assert_eq!(
        InteractionDataType::from(12),
        InteractionDataType::CallbackCommandClick
    );
    assert_eq!(
        InteractionDataType::from(13),
        InteractionDataType::MessageFeedbackClick
    );
    assert_eq!(
        InteractionDataType::from(14),
        InteractionDataType::ClearSessionClick
    );
}

#[test]
fn interaction_types_serialize_as_numeric_wire_values() {
    assert_eq!(
        serde_json::to_value(InteractionType::ApplicationCommand).unwrap(),
        serde_json::json!(2)
    );
    assert_eq!(
        serde_json::from_value::<InteractionType>(serde_json::json!(11)).unwrap(),
        InteractionType::InlineKeyboard
    );
    assert_eq!(
        serde_json::to_value(InteractionDataType::ChatInputSearch).unwrap(),
        serde_json::json!(9)
    );
    assert_eq!(
        serde_json::from_value::<InteractionDataType>(serde_json::json!(14)).unwrap(),
        InteractionDataType::ClearSessionClick
    );
}

#[test]
fn interaction_payload_uses_expected_type_fields() {
    let interaction = Interaction::new(
        BotApi::new(crate::http::HttpClient::new(30, false).unwrap()),
        Some("event-1".to_string()),
        &serde_json::json!({
            "id": "interaction-1",
            "application_id": "app-1",
            "type": 2,
            "data": {
                "name": "search",
                "type": 9,
                "resolved": {
                    "keyword": "botrs"
                }
            },
            "version": 1
        }),
    );

    let value = serde_json::to_value(&interaction).unwrap();
    assert_eq!(value["type"], serde_json::json!(2));
    assert_eq!(value["data"]["type"], serde_json::json!(9));
    assert_eq!(interaction.event_id.as_deref(), Some("event-1"));
    assert!(value.get("event_id").is_none());
    assert!(value.get("interaction_type").is_none());
    assert!(value["data"].get("data_type").is_none());
}

#[test]
fn resolved_uses_required_zero_value_fields() {
    let resolved: Resolved = serde_json::from_value(serde_json::json!({
        "button_id": "btn-1",
        "checked": 1
    }))
    .unwrap();

    assert_eq!(resolved.keyword, "");
    assert_eq!(resolved.user_id, "");
    assert_eq!(resolved.request, "");
    assert_eq!(resolved.message_id, "");
    assert_eq!(resolved.member_nick, "");
    assert_eq!(resolved.button_data, "");
    assert_eq!(resolved.button_id, "btn-1");
    assert_eq!(resolved.feature_id, "");
    assert_eq!(resolved.feedback_opt, "");
    assert_eq!(resolved.checked, 1);

    let value = serde_json::to_value(Resolved::default()).unwrap();
    assert_eq!(value["keyword"], "");
    assert_eq!(value["button_id"], "");
    assert_eq!(value["checked"], 0);
}

#[test]
fn search_dtos_keep_official_json_shape() {
    let resolved = SearchInputResolved {
        keyword: "botrs".to_string(),
    };
    let resolved_value = serde_json::to_value(&resolved).unwrap();
    assert_eq!(resolved_value["keyword"], "botrs");

    let empty_resolved = serde_json::to_value(SearchInputResolved::default()).unwrap();
    assert!(empty_resolved.get("keyword").is_none());

    let response = SearchRsp {
        layouts: vec![SearchLayout {
            layout_type: LayoutTypeImageText,
            action_type: ActionTypeSendARK,
            title: "docs".to_string(),
            records: vec![SearchRecord {
                cover: "https://example.com/cover.png".to_string(),
                title: "BotRS".to_string(),
                tips: "Rust SDK".to_string(),
                url: "https://example.com".to_string(),
            }],
        }],
    };
    let value = serde_json::to_value(&response).unwrap();

    assert_eq!(value["layouts"][0]["LayoutType"], 0);
    assert_eq!(value["layouts"][0]["ActionType"], 0);
    assert_eq!(value["layouts"][0]["Title"], "docs");
    assert_eq!(
        value["layouts"][0]["Records"][0]["cover"],
        "https://example.com/cover.png"
    );
    assert_eq!(value["layouts"][0]["Records"][0]["title"], "BotRS");
    assert_eq!(value["layouts"][0]["Records"][0]["tips"], "Rust SDK");
    assert_eq!(
        value["layouts"][0]["Records"][0]["url"],
        "https://example.com"
    );
    assert!(value["layouts"][0].get("layout_type").is_none());
    assert!(value["layouts"][0].get("action_type").is_none());
}
