use super::{
    Ark, ArkKv, ArkObj, ArkObjKv, C2CMessageParams, DirectMessage, Embed, GroupMessageParams,
    Keyboard, KeyboardButton, KeyboardButtonAction, KeyboardButtonPermission,
    KeyboardButtonRenderData, KeyboardContent, KeyboardModal, KeyboardRow, KeyboardStyle,
    KeyboardSubscribeData, KeyboardTemplateId, MarkdownParam, MarkdownPayload, MarkdownStyle,
    Media, MediaInfo, Message, MessageAttachment, MessageAudit, MessageCreateType, MessageMember,
    MessageParams, MessageReference, MessageScene, MessageToCreate, MessageUser, MessagesPager,
    Reference, SettingGuideParams, Stream,
};

#[test]
fn c2c_message_accepts_minimal_gateway_payload() {
    let mut message: super::C2CMessage = serde_json::from_value(serde_json::json!({
        "author": {
            "bot": false,
            "id": "OPENID_XXXXXX",
            "union_openid": "UNION_OPENID_XXXXXX",
            "user_openid": "USER_OPENID_XXXXXX"
        },
        "content": "ping",
        "id": "ROBOT1.0_MESSAGE_ID_XXXXXX",
        "message_type": 0,
        "msg_seq": 0,
        "source": "default",
        "timestamp": "2026-05-27T00:47:07+08:00"
    }))
    .unwrap();
    message.event_id = Some("event-1".to_string());

    assert_eq!(message.id, "ROBOT1.0_MESSAGE_ID_XXXXXX");
    assert_eq!(message.event_id.as_deref(), Some("event-1"));
    assert!(message.mentions.is_empty());
    assert!(message.attachments.is_empty());

    let value = serde_json::to_value(&message).unwrap();
    assert!(value.get("event_id").is_none());
}

#[test]
fn message_params_convert_to_create_payload_shape() {
    let params = MessageParams {
        content: Some("hello".to_string()),
        image: Some("https://example.com/image.png".to_string()),
        msg_id: Some("message-1".to_string()),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(MessageToCreate::from(params)).unwrap(),
        serde_json::json!({
            "content": "hello",
            "image": "https://example.com/image.png",
            "msg_id": "message-1"
        })
    );

    let params = GroupMessageParams {
        msg_type: 7,
        content: Some("media".to_string()),
        media: Some(Media {
            file_info: Some("file-info".to_string()),
            ttl: Some(60),
            ..Default::default()
        }),
        msg_seq: Some(42),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(MessageToCreate::from(params)).unwrap(),
        serde_json::json!({
            "content": "media",
            "msg_type": 7,
            "media": {
                "file_info": "file-info"
            },
            "msg_seq": 42
        })
    );
}

#[test]
fn message_params_include_extended_send_fields() {
    let params = MessageParams {
        content: Some("hello".to_string()),
        subscribe_id: Some("subscribe-1".to_string()),
        stream: Some(Stream {
            state: Some(1),
            id: Some("stream-1".to_string()),
            index: Some(2),
            reset: Some(false),
        }),
        feature_id: Some(42),
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_value(MessageToCreate::from(params)).unwrap(),
        serde_json::json!({
            "content": "hello",
            "subscribe_id": "subscribe-1",
            "stream": {
                "state": 1,
                "id": "stream-1",
                "index": 2
            },
            "feature_id": 42
        })
    );
}

#[test]
fn message_pager_builds_expected_query_params() {
    let query = MessagesPager::around("message-1", Some(5)).to_query_params();

    assert_eq!(query.get("around").map(String::as_str), Some("message-1"));
    assert_eq!(query.get("limit").map(String::as_str), Some("5"));
}

#[test]
fn setting_guide_params_build_channel_mentions_and_dm_target() {
    let channel = SettingGuideParams::for_users(["user-1", "user-2"]);
    assert_eq!(
        serde_json::to_value(channel).unwrap(),
        serde_json::json!({
            "content": "<@user-1><@user-2>"
        })
    );

    let dm = SettingGuideParams::for_guild("guild-1");
    assert_eq!(
        serde_json::to_value(dm).unwrap(),
        serde_json::json!({
            "setting_guide": {
                "guild_id": "guild-1"
            }
        })
    );
}

#[test]
fn open_message_text_params_omit_unspecified_msg_seq() {
    let group = GroupMessageParams::new_text("hello");
    let group_value = serde_json::to_value(MessageToCreate::from(group)).unwrap();
    assert!(group_value.get("msg_seq").is_none());

    let c2c = C2CMessageParams::new_text("hello");
    let c2c_value = serde_json::to_value(MessageToCreate::from(c2c)).unwrap();
    assert!(c2c_value.get("msg_seq").is_none());
}

#[test]
fn markdown_params_set_expected_message_type() {
    let channel = MessageParams::new_markdown("# hello");
    let channel_value = serde_json::to_value(MessageToCreate::from(channel)).unwrap();
    assert_eq!(channel_value["msg_type"], serde_json::json!(2));
    assert_eq!(
        channel_value["markdown"]["content"],
        serde_json::json!("# hello")
    );

    let group = GroupMessageParams::new_markdown("# group");
    let group_value = serde_json::to_value(MessageToCreate::from(group)).unwrap();
    assert_eq!(group_value["msg_type"], serde_json::json!(2));
    assert_eq!(
        group_value["markdown"]["content"],
        serde_json::json!("# group")
    );

    let c2c = C2CMessageParams::new_markdown("# c2c");
    let c2c_value = serde_json::to_value(MessageToCreate::from(c2c)).unwrap();
    assert_eq!(c2c_value["msg_type"], serde_json::json!(2));
    assert_eq!(c2c_value["markdown"]["content"], serde_json::json!("# c2c"));
}

#[test]
fn open_message_manual_params_omit_unspecified_msg_seq() {
    let group = GroupMessageParams {
        msg_type: 0,
        content: Some("hello".to_string()),
        msg_id: Some("message-1".to_string()),
        ..Default::default()
    };
    let group_value = serde_json::to_value(MessageToCreate::from(group)).unwrap();
    assert!(group_value.get("msg_seq").is_none());

    let c2c = C2CMessageParams {
        msg_type: 0,
        content: Some("hello".to_string()),
        msg_id: Some("message-1".to_string()),
        ..Default::default()
    };
    let c2c_value = serde_json::to_value(MessageToCreate::from(c2c)).unwrap();
    assert!(c2c_value.get("msg_seq").is_none());
}

#[test]
fn media_keeps_upload_response_file_uuid() {
    let media: Media = serde_json::from_value(serde_json::json!({
        "file_uuid": "FILE_UUID_XXXXXX",
        "file_info": "FILE_INFO_XXXXXX",
        "ttl": 3600
    }))
    .unwrap();

    assert_eq!(media.file_uuid.as_deref(), Some("FILE_UUID_XXXXXX"));
    assert_eq!(media.file_info.as_deref(), Some("FILE_INFO_XXXXXX"));
    assert_eq!(media.ttl, Some(3600));

    let request_media = MediaInfo::from(media);
    assert_eq!(
        serde_json::to_value(&request_media).unwrap(),
        serde_json::json!({
            "file_info": "FILE_INFO_XXXXXX"
        })
    );
}

#[test]
fn message_create_omits_go_zero_values() {
    let message = MessageToCreate {
        content: Some(String::new()),
        msg_type: Some(MessageCreateType::Text),
        image: Some(String::new()),
        msg_id: Some(String::new()),
        event_id: Some(String::new()),
        msg_seq: Some(0),
        media: Some(MediaInfo {
            file_info: Some(String::new()),
        }),
        ark: Some(Ark {
            template_id: Some(0),
            kv: Some(Vec::new()),
        }),
        embed: Some(Embed::default()),
        ..Default::default()
    };

    let value = serde_json::to_value(&message).unwrap();
    for key in [
        "content", "msg_type", "image", "msg_id", "event_id", "msg_seq",
    ] {
        assert!(value.get(key).is_none(), "{key} should be omitted");
    }
    assert_eq!(value["media"], serde_json::json!({}));
    assert_eq!(value["ark"], serde_json::json!({}));
    assert_eq!(
        value["embed"],
        serde_json::json!({
            "prompt": "",
            "thumbnail": {
                "url": ""
            }
        })
    );
}

#[test]
fn message_create_keeps_non_zero_omitempty_values() {
    let message = MessageToCreate {
        content: Some("hello".to_string()),
        msg_type: Some(MessageCreateType::Markdown),
        image: Some("https://example.com/image.png".to_string()),
        msg_id: Some("msg-1".to_string()),
        event_id: Some("event-1".to_string()),
        msg_seq: Some(1),
        media: Some(MediaInfo {
            file_info: Some("file-info".to_string()),
        }),
        ark: Some(Ark {
            template_id: Some(23),
            kv: Some(vec![ArkKv {
                key: Some("key".to_string()),
                value: Some("value".to_string()),
                obj: Some(vec![ArkObj {
                    obj_kv: Some(vec![ArkObjKv {
                        key: Some("nested-key".to_string()),
                        value: Some("nested-value".to_string()),
                    }]),
                }]),
            }]),
        }),
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_value(&message).unwrap(),
        serde_json::json!({
            "content": "hello",
            "msg_type": 2,
            "image": "https://example.com/image.png",
            "msg_id": "msg-1",
            "event_id": "event-1",
            "msg_seq": 1,
            "media": {
                "file_info": "file-info"
            },
            "ark": {
                "template_id": 23,
                "kv": [{
                    "key": "key",
                    "value": "value",
                    "obj": [{
                        "obj_kv": [{
                            "key": "nested-key",
                            "value": "nested-value"
                        }]
                    }]
                }]
            },
        })
    );
}

#[test]
fn embed_keeps_required_zero_value_fields() {
    let embed = Embed::default();
    assert_eq!(
        serde_json::to_value(&embed).unwrap(),
        serde_json::json!({
            "prompt": "",
            "thumbnail": {
                "url": ""
            }
        })
    );
}

#[test]
fn keyboard_action_keeps_official_zero_value_shape() {
    let action = KeyboardButtonAction::default();

    assert_eq!(
        serde_json::to_value(&action).unwrap(),
        serde_json::json!({
            "enter": false,
            "subscribe_data": {}
        })
    );
}

#[test]
fn keyboard_omits_go_zero_values() {
    let keyboard = Keyboard {
        id: Some(String::new()),
        content: Some(KeyboardContent {
            rows: Some(vec![KeyboardRow {
                buttons: Some(vec![KeyboardButton {
                    id: Some(String::new()),
                    render_data: Some(KeyboardButtonRenderData {
                        label: Some(String::new()),
                        visited_label: Some(String::new()),
                        style: Some(0),
                    }),
                    action: Some(KeyboardButtonAction {
                        action_type: Some(0),
                        click_limit: Some(0),
                        data: Some(String::new()),
                        at_bot_show_channel_list: Some(false),
                        permission: Some(KeyboardButtonPermission {
                            permission_type: Some(0),
                            specify_role_ids: Some(Vec::new()),
                            specify_user_ids: Some(Vec::new()),
                        }),
                        modal: Some(KeyboardModal {
                            content: Some(String::new()),
                            confirm_text: Some(String::new()),
                            cancel_text: Some(String::new()),
                        }),
                        subscribe_data: KeyboardSubscribeData {
                            template_ids: Some(vec![KeyboardTemplateId {
                                template_id: Some(0),
                                custom_template_id: Some(String::new()),
                            }]),
                        },
                        ..Default::default()
                    }),
                    group_id: Some(String::new()),
                }]),
            }]),
            style: Some(KeyboardStyle {
                font_size: Some(String::new()),
            }),
        }),
    };

    assert_eq!(
        serde_json::to_value(&keyboard).unwrap(),
        serde_json::json!({
            "content": {
                "rows": [{
                    "buttons": [{
                        "render_data": {},
                        "action": {
                            "permission": {},
                            "enter": false,
                            "subscribe_data": {
                                "template_ids": [{}]
                            },
                            "modal": {}
                        }
                    }]
                }],
                "style": {}
            }
        })
    );
}

#[test]
fn keyboard_keeps_non_zero_values() {
    let action = KeyboardButtonAction {
        action_type: Some(4),
        click_limit: Some(1),
        data: Some("payload".to_string()),
        enter: true,
        at_bot_show_channel_list: Some(true),
        subscribe_data: KeyboardSubscribeData {
            template_ids: Some(vec![KeyboardTemplateId {
                template_id: Some(1),
                custom_template_id: None,
            }]),
        },
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_value(&action).unwrap(),
        serde_json::json!({
            "type": 4,
            "click_limit": 1,
            "data": "payload",
            "enter": true,
            "at_bot_show_channel_list": true,
            "subscribe_data": {
                "template_ids": [{
                    "template_id": 1
                }]
            }
        })
    );
}

#[test]
fn markdown_payload_keeps_official_zero_value_shape() {
    let markdown = MarkdownPayload {
        style: Some(MarkdownStyle::default()),
        params: Some(vec![MarkdownParam {
            key: None,
            values: None,
        }]),
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_value(&markdown).unwrap(),
        serde_json::json!({
            "template_id": 0,
            "custom_template_id": "",
            "params": [{
                "key": "",
                "values": []
            }],
            "content": "",
            "style": {
                "main_font_size": "",
                "layout": ""
            },
            "process_msg": ""
        })
    );
}

#[test]
fn reference_keeps_official_zero_value_shape() {
    let reference = Reference {
        message_id: Some("message-1".to_string()),
        ignore_get_message_error: None,
    };

    assert_eq!(
        serde_json::to_value(&reference).unwrap(),
        serde_json::json!({
            "message_id": "message-1",
            "ignore_get_message_error": false
        })
    );
}

#[test]
fn test_message_creation() {
    let message = Message::default();
    assert_eq!(message.id, "");
    assert_eq!(message.content, "");
    assert!(message.attachments.is_empty());
    assert!(message.mentions.is_empty());
}

#[test]
fn direct_message_is_session_dto() {
    let session: DirectMessage = serde_json::from_value(serde_json::json!({
        "guild_id": "guild-1",
        "channel_id": "channel-1",
        "create_time": "2024-01-02T03:04:05+08:00",
        "content": "ignored"
    }))
    .unwrap();

    assert_eq!(session.guild_id, "guild-1");
    assert_eq!(session.channel_id, "channel-1");
    assert_eq!(session.create_time, "2024-01-02T03:04:05+08:00");

    let value = serde_json::to_value(&session).unwrap();
    assert_eq!(value["guild_id"], serde_json::json!("guild-1"));
    assert_eq!(value["channel_id"], serde_json::json!("channel-1"));
    assert_eq!(
        value["create_time"],
        serde_json::json!("2024-01-02T03:04:05+08:00")
    );
    assert!(value.get("content").is_none());
}

#[test]
fn direct_message_session_rejects_missing_required_fields() {
    assert!(serde_json::from_value::<DirectMessage>(serde_json::json!({})).is_err());
}

#[test]
fn test_message_with_content() {
    let message = Message {
        content: "Hello, world!".to_string(),
        ..Default::default()
    };
    assert!(!message.content.is_empty());
}

#[test]
fn message_event_id_is_internal_only() {
    let mut message: Message = serde_json::from_value(serde_json::json!({
            "id": "message-1",
            "content": "hello",
            "channel_id": "channel-1",
            "guild_id": "guild-1",
            "author": {
                "id": "user-1",
                "username": "user",
                "bot": false
            },
            "seq_in_channel": "1",
            "timestamp": "2024-01-01T00:00:00+08:00"
    }))
    .unwrap();
    message.event_id = Some("event-1".to_string());

    assert_eq!(message.event_id.as_deref(), Some("event-1"));
    let value = serde_json::to_value(&message).unwrap();
    assert!(value.get("event_id").is_none());
}

#[test]
fn message_reference_keeps_ignore_error_flag() {
    let reference: MessageReference = serde_json::from_value(serde_json::json!({
        "message_id": "message-1",
        "ignore_get_message_error": true
    }))
    .unwrap();

    assert_eq!(reference.message_id, "message-1");
    assert!(reference.ignore_get_message_error);

    let value = serde_json::to_value(&reference).unwrap();
    assert_eq!(value["message_id"], serde_json::json!("message-1"));
    assert_eq!(value["ignore_get_message_error"], serde_json::json!(true));
}

#[test]
fn message_reference_defaults_missing_ignore_error_flag() {
    let reference: MessageReference = serde_json::from_value(serde_json::json!({
        "message_id": "message-1"
    }))
    .unwrap();

    assert_eq!(reference.message_id, "message-1");
    assert!(!reference.ignore_get_message_error);
}

#[test]
fn message_reference_rejects_missing_message_id() {
    assert!(
        serde_json::from_value::<MessageReference>(serde_json::json!({
            "ignore_get_message_error": true
        }))
        .is_err()
    );
}

#[test]
fn message_events_reject_missing_required_identity_fields() {
    assert!(serde_json::from_value::<Message>(serde_json::json!({})).is_err());
    assert!(serde_json::from_value::<super::GroupMessage>(serde_json::json!({})).is_err());
    assert!(serde_json::from_value::<super::C2CMessage>(serde_json::json!({})).is_err());
}

#[test]
fn message_member_accepts_partial_author_member_payload() {
    let member: MessageMember = serde_json::from_value(serde_json::json!({
        "joined_at": "2024-01-01T00:00:00+08:00"
    }))
    .unwrap();

    assert_eq!(member.nick, "");
    assert!(member.roles.is_empty());
    assert_eq!(member.joined_at, "2024-01-01T00:00:00+08:00");
}

#[test]
fn message_scene_accepts_callback_without_source() {
    let scene: MessageScene = serde_json::from_value(serde_json::json!({
        "callback_data": "payload"
    }))
    .unwrap();

    assert_eq!(scene.source, "");
    assert_eq!(scene.callback_data, "payload");
    assert!(scene.ext.is_empty());
    assert_eq!(
        serde_json::to_value(&scene).unwrap(),
        serde_json::json!({
            "callback_data": "payload"
        })
    );
}

#[test]
fn message_audit_keeps_channel_sequence() {
    let mut audit: MessageAudit = serde_json::from_value(serde_json::json!({
            "audit_id": "audit-1",
            "message_id": "message-1",
            "guild_id": "guild-1",
            "channel_id": "channel-1",
            "audit_time": "2024-01-02T03:04:05+08:00",
            "create_time": "2024-01-02T03:03:00+08:00",
            "seq_in_channel": "42"
    }))
    .unwrap();
    audit.event_id = Some("event-1".to_string());

    assert_eq!(audit.seq_in_channel, "42");
    assert_eq!(audit.audit_time, "2024-01-02T03:04:05+08:00");
    assert_eq!(audit.create_time, "2024-01-02T03:03:00+08:00");

    let value = serde_json::to_value(&audit).unwrap();
    assert_eq!(value["seq_in_channel"], serde_json::json!("42"));
    assert!(value.get("event_id").is_none());
}

#[test]
fn message_audit_rejects_missing_required_fields() {
    assert!(serde_json::from_value::<MessageAudit>(serde_json::json!({})).is_err());
}

#[test]
fn embed_keeps_prompt_field() {
    let embed = Embed {
        title: Some("title".to_string()),
        prompt: "summary".to_string(),
        ..Default::default()
    };

    let value = serde_json::to_value(&embed).unwrap();
    assert_eq!(value["prompt"], serde_json::json!("summary"));
    let parsed: Embed = serde_json::from_value(value).unwrap();
    assert_eq!(parsed.prompt, "summary");
}

#[test]
fn message_attachment_value_fields_serialize_in_official_shape() {
    let attachment = MessageAttachment::default();

    assert_eq!(attachment.id, None);
    assert_eq!(attachment.filename, "");
    assert_eq!(attachment.content_type, "");
    assert_eq!(attachment.content, "");
    assert_eq!(attachment.size, 0);
    assert_eq!(attachment.url, "");
    assert_eq!(attachment.width, 0);
    assert_eq!(attachment.height, 0);

    assert_eq!(
        serde_json::to_value(&attachment).unwrap(),
        serde_json::json!({})
    );
}

#[test]
fn message_attachment_rejects_missing_required_fields() {
    assert!(serde_json::from_value::<MessageAttachment>(serde_json::json!({})).is_err());
}

#[test]
fn message_attachment_defaults_missing_dimensions_for_non_image() {
    let attachment: MessageAttachment = serde_json::from_value(serde_json::json!({
        "content_type": "audio/silk",
        "filename": "voice.silk",
        "size": 1024,
        "url": "https://multimedia.nt.qq.com.cn/download?appid=1407"
    }))
    .unwrap();

    assert_eq!(attachment.content_type, "audio/silk");
    assert_eq!(attachment.filename, "voice.silk");
    assert_eq!(attachment.size, 1024);
    assert_eq!(attachment.width, 0);
    assert_eq!(attachment.height, 0);

    assert_eq!(
        serde_json::to_value(&attachment).unwrap(),
        serde_json::json!({
            "filename": "voice.silk",
            "content_type": "audio/silk",
            "size": 1024,
            "url": "https://multimedia.nt.qq.com.cn/download?appid=1407"
        })
    );
}

#[test]
fn message_attachment_keeps_open_message_shape() {
    let attachment: MessageAttachment = serde_json::from_value(serde_json::json!({
        "content": "",
        "content_type": "image/png",
        "filename": "91FE1A7D6BEE23893635173599CE58DF.png",
        "height": 512,
        "size": 67372,
        "url": "https://multimedia.nt.qq.com.cn/download?appid=1407",
        "width": 754
    }))
    .unwrap();

    assert_eq!(attachment.id, None);
    assert_eq!(attachment.content, "");
    assert_eq!(attachment.content_type, "image/png");
    assert_eq!(attachment.filename, "91FE1A7D6BEE23893635173599CE58DF.png");
    assert_eq!(attachment.height, 512);
    assert_eq!(attachment.size, 67372);
    assert_eq!(
        attachment.url,
        "https://multimedia.nt.qq.com.cn/download?appid=1407"
    );
    assert_eq!(attachment.width, 754);

    assert_eq!(
        serde_json::to_value(&attachment).unwrap(),
        serde_json::json!({
            "filename": "91FE1A7D6BEE23893635173599CE58DF.png",
            "content_type": "image/png",
            "size": 67372,
            "url": "https://multimedia.nt.qq.com.cn/download?appid=1407",
            "width": 754,
            "height": 512
        })
    );
}

#[test]
fn test_bot_detection() {
    let mut message = Message {
        author: MessageUser {
            id: "123".to_string(),
            username: "Bot".to_string(),
            bot: true,
            ..Default::default()
        },
        ..Default::default()
    };

    assert!(message.author.bot);

    message.author.bot = false;
    assert!(!message.author.bot);
}
