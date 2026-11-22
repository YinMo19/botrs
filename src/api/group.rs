use super::{BotApi, resource};
use crate::{BotError, Result, models::group::GroupBotState};
use reqwest::Method;
use serde::{Deserialize, Serialize};

impl BotApi {
    /// Reads the receive mode, proactive messaging permission and role in one group.
    /// QQ restricts this endpoint by application (platform code 11253).
    pub async fn get_group_bot_state(&self, group_openid: &str) -> Result<GroupBotState> {
        let path = resource::group_bot_state(group_openid);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Removes 1–20 members from one QQ group in a single request.
    ///
    /// Supply group-scoped `member_openid` values. QQ requires the remove-members
    /// permission in both the web management console and the group's settings.
    /// The platform returns 40103011 or 40103012 respectively when disabled.
    pub async fn remove_group_members(
        &self,
        group_openid: &str,
        member_openids: &[impl AsRef<str>],
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Request<'a> {
            member_openids: Vec<&'a str>,
        }
        #[derive(Deserialize)]
        struct Response {
            remove_members_result: String,
        }

        if !(1..=20).contains(&member_openids.len()) {
            return Err(BotError::invalid_data(
                "expected 1 to 20 group member OpenIDs",
            ));
        }
        let member_openids: Vec<_> = member_openids.iter().map(AsRef::as_ref).collect();
        if group_openid.trim().is_empty() || member_openids.iter().any(|id| id.trim().is_empty()) {
            return Err(BotError::invalid_data(
                "group and member OpenIDs must be non-empty",
            ));
        }
        let path = resource::group_batch_remove_members(group_openid);
        let response: Response = self
            .request_json(
                Method::POST,
                &path,
                None::<&()>,
                Some(&Request { member_openids }),
            )
            .await?;
        if response.remove_members_result != "success" {
            return Err(BotError::invalid_data(format!(
                "unexpected remove_members_result: {}",
                response.remove_members_result
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_impl::tests::test_api;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn group_member_removal_checks_limits_and_preserves_qq_errors() {
        use axum::{
            Json, Router,
            extract::Path,
            http::{HeaderMap, StatusCode},
            routing::post,
        };
        use serde_json::{Value, json};

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let api = test_api(format!("http://{}", listener.local_addr().unwrap())).await;
        let remove = |Path(group): Path<String>, headers: HeaderMap, Json(body): Json<Value>| async move {
            assert_eq!(headers["authorization"], "QQBot ACCESS_TOKEN_XXXXXX");
            let result = match group.as_str() {
                "web-disabled" => return (StatusCode::FORBIDDEN, Json(json!({"code": 40103011}))),
                "owner-disabled" => {
                    return (StatusCode::FORBIDDEN, Json(json!({"code": 40103012})));
                }
                "unconfirmed" => "unknown",
                "batch" => {
                    let expected: Vec<_> = (0..20).map(|i| format!("member-{i}")).collect();
                    assert_eq!(body, json!({"member_openids": expected}));
                    "success"
                }
                "single" => {
                    assert_eq!(body, json!({"member_openids": ["member"]}));
                    "success"
                }
                _ => panic!("unexpected group"),
            };
            (
                StatusCode::OK,
                Json(json!({"remove_members_result": result})),
            )
        };
        let app = Router::new().route("/v2/groups/{group}/batch_remove_members", post(remove));
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        api.remove_group_members("single", &["member"])
            .await
            .unwrap();
        let batch: Vec<_> = (0..20).map(|i| format!("member-{i}")).collect();
        api.remove_group_members("batch", &batch).await.unwrap();
        for ids in [&[][..], &[" "][..], &["member"; 21][..]] {
            assert!(matches!(
                api.remove_group_members("invalid", ids).await,
                Err(BotError::InvalidData(_))
            ));
        }
        for (group, code) in [("web-disabled", 40103011), ("owner-disabled", 40103012)] {
            let err = api
                .remove_group_members(group, &["member"])
                .await
                .unwrap_err();
            assert_eq!(err.response().unwrap().platform_code, Some(code));
        }
        assert!(
            api.remove_group_members("unconfirmed", &["member"])
                .await
                .is_err()
        );
        server.abort();
    }

    #[tokio::test]
    async fn group_state_is_an_authenticated_get_with_independent_permissions() {
        use crate::models::group::{GroupMemberRole, GroupReceiveMode};
        use axum::{Json, Router, extract::Path, http::HeaderMap, routing::get};
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let api = test_api(format!("http://{}", listener.local_addr().unwrap())).await;
        let app = Router::new().route(
            "/v2/groups/{group}/bot_state",
            get(|Path(group): Path<String>, headers: HeaderMap| async move {
                assert_eq!(headers["authorization"], "QQBot ACCESS_TOKEN_XXXXXX");
                Json(serde_json::json!({
                    "member_openid": "bot", "joined_at": "2025-06-15T14:30:00+08:00",
                    "allow_proactive_msg": group == "only_mention",
                    "recv_msg_setting": group, "member_role": "admin"
                }))
            }),
        );
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        for (wire, mode) in [
            ("only_mention", GroupReceiveMode::OnlyMention),
            ("mention_and_context", GroupReceiveMode::MentionAndContext),
            ("all", GroupReceiveMode::All),
            ("future", GroupReceiveMode::Other("future".into())),
        ] {
            let state = api.get_group_bot_state(wire).await.unwrap();
            assert_eq!(state.recv_msg_setting, mode);
            assert_eq!(state.allow_proactive_msg, wire == "only_mention");
            assert_eq!(state.member_role, GroupMemberRole::Admin);
            assert_eq!(serde_json::to_value(mode).unwrap(), wire);
        }
        server.abort();
    }
}
