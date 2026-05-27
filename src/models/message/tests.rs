#[cfg(test)]
mod tests {
    use super::super::{
        ACTION_TYPE_SUBSCRIBE, ActionButton, Ark, ArkKv, ArkObj, ArkObjKv, C2CMessageParams,
        DirectMessage, Embed, GroupMessageParams, InputNotify, Keyboard, KeyboardButton,
        KeyboardButtonAction, KeyboardButtonPermission, KeyboardButtonRenderData, KeyboardContent,
        KeyboardModal, KeyboardRow, KeyboardStyle, KeyboardSubscribeData, KeyboardTemplateId,
        MarkdownParam, MarkdownPayload, MarkdownStyle, Media, MediaInfo, Message,
        MessageAttachment, MessageAudit, MessageCreateType, MessagePagerType, MessageParams,
        MessageReference, MessageToCreate, MessageUser, MessagesPager, Reference, Stream, emoji,
        etl_input, mention_all_user, mention_channel, mention_user, parse_command,
    };

    #[test]
    fn c2c_message_accepts_minimal_gateway_payload() {
        let message = super::super::C2CMessage::from_data(
            "event-1".to_string(),
            serde_json::json!({
                "author": {
                    "bot": false,
                    "id": "OPENID_XXXXXX",
                    "union_openid": "UNION_OPENID_XXXXXX",
                    "user_openid": "USER_OPENID_XXXXXX"
                },
                "content": "ping",
                "id": "ROBOT1.0_MESSAGE_ID_XXXXXX",
                "msg_seq": 0,
                "source": "default",
                "timestamp": "2026-05-27T00:47:07+08:00"
            }),
        );

        assert_eq!(message.id.as_deref(), Some("ROBOT1.0_MESSAGE_ID_XXXXXX"));
        assert_eq!(message.event_id.as_deref(), Some("event-1"));
        assert!(message.mentions.is_empty());
        assert!(message.attachments.is_empty());

        let value = serde_json::to_value(&message).unwrap();
        assert!(value.get("event_id").is_none());
    }

    #[test]
    fn test_message_helpers() {
        assert_eq!(mention_user("123"), "<@123>");
        assert_eq!(mention_all_user(), "@everyone");
        assert_eq!(mention_channel("456"), "<#456>");
        assert_eq!(emoji(1), "<emoji:1>");
        assert_eq!(etl_input("<@!123>  ping value"), "ping value");
        assert_eq!(
            etl_input("\u{00a0}<@!123> ping  value\u{00a0}"),
            "ping  value"
        );
        assert_eq!(etl_input("<@123> ping"), "<@123> ping");
        assert_eq!(etl_input("<@!abc> ping"), "<@!abc> ping");

        let command = parse_command("<@!123>  /ping value");
        assert_eq!(command.cmd, "/ping");
        assert_eq!(command.content, "value");

        let command = parse_command("<@!123> /ping value");
        assert_eq!(command.cmd, "/ping");
        assert_eq!(command.content, "value");

        let command = parse_command("/ping\tvalue");
        assert_eq!(command.cmd, "/ping\tvalue");
        assert_eq!(command.content, "");
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
    fn open_message_text_params_omit_unspecified_msg_seq() {
        let group = GroupMessageParams::new_text("hello");
        let group_value = serde_json::to_value(MessageToCreate::from(group)).unwrap();
        assert!(group_value.get("msg_seq").is_none());

        let c2c = C2CMessageParams::new_text("hello");
        let c2c_value = serde_json::to_value(MessageToCreate::from(c2c)).unwrap();
        assert!(c2c_value.get("msg_seq").is_none());
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
            subscribe_id: Some(String::new()),
            feature_id: Some(0),
            input_notify: Some(InputNotify {
                input_type: Some(0),
                input_second: Some(0),
            }),
            media: Some(MediaInfo {
                file_info: Some(String::new()),
            }),
            action_button: Some(ActionButton {
                template_id: Some(0),
                callback_data: Some(String::new()),
                feedback: Some(false),
                tts: Some(false),
                re_generate: Some(false),
                stop_generate: Some(false),
            }),
            stream: Some(Stream {
                state: Some(0),
                id: Some(String::new()),
                index: Some(0),
                reset: Some(false),
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
            "content",
            "msg_type",
            "image",
            "msg_id",
            "event_id",
            "msg_seq",
            "subscribe_id",
            "feature_id",
        ] {
            assert!(value.get(key).is_none(), "{key} should be omitted");
        }
        assert_eq!(value["input_notify"], serde_json::json!({}));
        assert_eq!(value["media"], serde_json::json!({}));
        assert_eq!(value["action_button"], serde_json::json!({}));
        assert_eq!(value["stream"], serde_json::json!({}));
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
            subscribe_id: Some("sub-1".to_string()),
            feature_id: Some(7),
            input_notify: Some(InputNotify {
                input_type: Some(1),
                input_second: Some(3),
            }),
            media: Some(MediaInfo {
                file_info: Some("file-info".to_string()),
            }),
            action_button: Some(ActionButton {
                template_id: Some(2),
                callback_data: Some("callback".to_string()),
                feedback: Some(true),
                tts: Some(true),
                re_generate: Some(true),
                stop_generate: Some(true),
            }),
            stream: Some(Stream {
                state: Some(1),
                id: Some("stream-1".to_string()),
                index: Some(1),
                reset: Some(true),
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
                "subscribe_id": "sub-1",
                "input_notify": {
                    "input_type": 1,
                    "input_second": 3
                },
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
                "action_button": {
                    "template_id": 2,
                    "callback_data": "callback",
                    "feedback": true,
                    "tts": true,
                    "re_generate": true,
                    "stop_generate": true
                },
                "stream": {
                    "state": 1,
                    "id": "stream-1",
                    "index": 1,
                    "reset": true
                },
                "feature_id": 7
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
            action_type: Some(ACTION_TYPE_SUBSCRIBE),
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
    fn messages_pager_query_params() {
        let pager = MessagesPager::new(Some(MessagePagerType::Before), Some("msg-1"), Some(20));
        let query = pager.query_params();

        assert_eq!(query.get("before").map(String::as_str), Some("msg-1"));
        assert_eq!(query.get("limit").map(String::as_str), Some("20"));
    }

    #[test]
    fn messages_pager_omits_empty_query_params() {
        let pager = MessagesPager::new(Some(MessagePagerType::Before), Some(""), Some(""));
        assert!(pager.query_params().is_empty());
    }

    #[test]
    fn test_message_creation() {
        let message = Message::new();
        assert!(message.id.is_none());
        assert!(message.content.is_none());
        assert!(!message.has_content());
        assert!(!message.has_attachments());
        assert!(!message.has_mentions());
    }

    #[test]
    fn direct_message_is_session_dto() {
        let session = DirectMessage::from_data(serde_json::json!({
            "guild_id": "guild-1",
            "channel_id": "channel-1",
            "create_time": "2024-01-02T03:04:05+08:00",
            "content": "ignored"
        }));

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
    fn direct_message_session_uses_required_zero_value_fields() {
        let session: DirectMessage = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(session.guild_id, "");
        assert_eq!(session.channel_id, "");
        assert_eq!(session.create_time, "");

        let value = serde_json::to_value(&session).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "guild_id": "",
                "channel_id": "",
                "create_time": ""
            })
        );
    }

    #[test]
    fn test_message_with_content() {
        let mut message = Message::new();
        message.content = Some("Hello, world!".to_string());
        assert!(message.has_content());
    }

    #[test]
    fn message_event_id_is_internal_only() {
        let message = Message::from_data(
            "event-1".to_string(),
            serde_json::json!({
                "id": "message-1",
                "content": "hello"
            }),
        );

        assert_eq!(message.event_id.as_deref(), Some("event-1"));
        let value = serde_json::to_value(&message).unwrap();
        assert!(value.get("event_id").is_none());
    }

    #[test]
    fn message_reference_keeps_ignore_error_flag() {
        let reference = MessageReference::from_data(serde_json::json!({
            "message_id": "message-1",
            "ignore_get_message_error": true
        }));

        assert_eq!(reference.message_id.as_deref(), Some("message-1"));
        assert_eq!(reference.ignore_get_message_error, Some(true));

        let value = serde_json::to_value(&reference).unwrap();
        assert_eq!(value["message_id"], serde_json::json!("message-1"));
        assert_eq!(value["ignore_get_message_error"], serde_json::json!(true));
    }

    #[test]
    fn message_audit_keeps_channel_sequence() {
        let audit = MessageAudit::from_data(
            "event-1".to_string(),
            serde_json::json!({
                "audit_id": "audit-1",
                "message_id": "message-1",
                "guild_id": "guild-1",
                "channel_id": "channel-1",
                "seq_in_channel": "42"
            }),
        );

        assert_eq!(audit.seq_in_channel, "42");
        assert_eq!(audit.audit_time, "");
        assert_eq!(audit.create_time, "");

        let value = serde_json::to_value(&audit).unwrap();
        assert_eq!(value["seq_in_channel"], serde_json::json!("42"));
        assert!(value.get("event_id").is_none());
    }

    #[test]
    fn message_audit_uses_required_zero_value_fields() {
        let audit: MessageAudit = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(audit.audit_id, "");
        assert_eq!(audit.message_id, "");
        assert_eq!(audit.guild_id, "");
        assert_eq!(audit.channel_id, "");
        assert_eq!(audit.audit_time, "");
        assert_eq!(audit.create_time, "");
        assert_eq!(audit.seq_in_channel, "");

        let value = serde_json::to_value(&audit).unwrap();
        assert_eq!(value["audit_id"], "");
        assert_eq!(value["message_id"], "");
        assert_eq!(value["guild_id"], "");
        assert_eq!(value["channel_id"], "");
        assert_eq!(value["audit_time"], "");
        assert_eq!(value["create_time"], "");
        assert_eq!(value["seq_in_channel"], "");
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
    fn test_message_attachment_types() {
        let mut attachment = MessageAttachment {
            id: Some("123".to_string()),
            filename: Some("image.png".to_string()),
            content_type: Some("image/png".to_string()),
            size: Some(1024),
            url: Some("https://example.com/image.png".to_string()),
            width: Some(800),
            height: Some(600),
        };

        assert!(attachment.is_image());
        assert!(!attachment.is_video());
        assert!(!attachment.is_audio());

        attachment.content_type = Some("video/mp4".to_string());
        assert!(!attachment.is_image());
        assert!(attachment.is_video());
        assert!(!attachment.is_audio());
    }

    #[test]
    fn message_attachment_omits_go_zero_values() {
        let attachment = MessageAttachment {
            id: Some(String::new()),
            filename: Some(String::new()),
            content_type: Some(String::new()),
            size: Some(0),
            url: Some(String::new()),
            width: Some(0),
            height: Some(0),
        };

        assert_eq!(
            serde_json::to_value(&attachment).unwrap(),
            serde_json::json!({})
        );

        let attachment = MessageAttachment {
            id: Some("attachment-1".to_string()),
            filename: Some("image.png".to_string()),
            content_type: Some("image/png".to_string()),
            size: Some(128),
            url: Some("https://example.com/image.png".to_string()),
            width: Some(64),
            height: Some(32),
        };

        assert_eq!(
            serde_json::to_value(&attachment).unwrap(),
            serde_json::json!({
                "id": "attachment-1",
                "filename": "image.png",
                "content_type": "image/png",
                "size": 128,
                "url": "https://example.com/image.png",
                "width": 64,
                "height": 32
            })
        );
    }

    #[test]
    fn test_bot_detection() {
        let mut message = Message::new();
        message.author = Some(MessageUser {
            id: Some("123".to_string()),
            username: Some("Bot".to_string()),
            bot: Some(true),
            avatar: None,
        });

        assert!(message.is_from_bot());

        message.author.as_mut().unwrap().bot = Some(false);
        assert!(!message.is_from_bot());
    }
}
