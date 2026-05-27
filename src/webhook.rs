//! Botgo-compatible interaction webhook helpers.

#![allow(non_snake_case, non_upper_case_globals)]

use std::sync::{LazyLock, RwLock};

use reqwest::header::HeaderMap;
use serde::Serialize;

use crate::models::gateway::{HTTPCallbackAck, WSHeartbeatAck};
use crate::models::webhook::{WHValidationReq, WHValidationRsp};

#[derive(Debug, Clone, Copy, Serialize)]
struct Ack {
    op: u8,
    d: u32,
}

pub type GetSecretFunc = fn() -> String;

pub static DefaultGetSecretFunc: LazyLock<RwLock<GetSecretFunc>> =
    LazyLock::new(|| RwLock::new(|| std::env::var("QQBotSecret").unwrap_or_default()));

pub fn GenHeartbeatACK(seq: u32) -> String {
    serde_json::to_string(&Ack {
        op: WSHeartbeatAck,
        d: seq,
    })
    .expect("heartbeat ack is serializable")
}

pub fn GenDispatchACK(success: bool) -> String {
    serde_json::to_string(&Ack {
        op: HTTPCallbackAck,
        d: u32::from(!success),
    })
    .expect("dispatch ack is serializable")
}

pub fn GenValidationACK(
    req: &WHValidationReq,
    headers: &HeaderMap,
    secret: &str,
) -> crate::Result<Vec<u8>> {
    let mut headers = headers.clone();
    headers.insert(
        crate::signature::HeaderTimestamp,
        req.event_ts
            .parse()
            .map_err(|_| crate::BotError::invalid_data("invalid event timestamp header"))?,
    );
    let signature = crate::signature::Generate(secret, &headers, req.plain_token.as_bytes())?;
    serde_json::to_vec(&WHValidationRsp {
        plain_token: req.plain_token.clone(),
        signature,
        data_version: String::new(),
    })
    .map_err(Into::into)
}

pub fn HTTPHandler(
    body: &[u8],
    headers: &HeaderMap,
    app_id: impl Into<String>,
    secret: &str,
) -> crate::Result<Option<Vec<u8>>> {
    if !crate::signature::Verify(secret, headers, body)? {
        return Err(crate::BotError::auth("signature verify failed"));
    }

    let mut payload: crate::models::gateway::WSPayload = serde_json::from_slice(body)?;
    payload.raw_message = Some(body.to_vec());
    payload.session = Some(crate::session_manager::Session::from_app_id(app_id));
    let trace_id = headers
        .get(crate::constant::HeaderTraceID)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if payload.base.op_code == crate::models::gateway::HTTPCallbackValidation {
        let data = payload
            .data
            .as_ref()
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let req: WHValidationReq = serde_json::from_value(data)?;
        return GenValidationACK(&req, headers, secret).map(Some);
    }

    match payload.base.op_code {
        crate::models::gateway::WSHeartbeat => {
            let seq = payload
                .data
                .as_ref()
                .and_then(|value| value.as_u64())
                .unwrap_or_default() as u32;
            Ok(Some(GenHeartbeatACK(seq).into_bytes()))
        }
        crate::models::gateway::WSDispatchEvent => match crate::event::ParseAndHandle(&mut payload)
        {
            Ok(()) => Ok(Some(GenDispatchACK(true).into_bytes())),
            Err(err) => {
                tracing::error!("parseAndHandle failed, {}, traceID:{}", err, trace_id);
                Ok(Some(GenDispatchACK(false).into_bytes()))
            }
        },
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn captured_app_id() -> &'static Mutex<Option<String>> {
        static CAPTURED: OnceLock<Mutex<Option<String>>> = OnceLock::new();
        CAPTURED.get_or_init(|| Mutex::new(None))
    }

    fn capture_session_app_id(
        payload: &mut crate::models::gateway::WSPayload,
        _: &[u8],
    ) -> crate::Result<()> {
        *captured_app_id().lock().unwrap() = payload
            .session
            .as_ref()
            .and_then(|session| session.app_id.clone());
        Ok(())
    }

    #[test]
    fn ack_payloads_match_expected_shape() {
        assert_eq!(GenHeartbeatACK(7), r#"{"op":11,"d":7}"#);
        assert_eq!(GenDispatchACK(true), r#"{"op":12,"d":0}"#);
        assert_eq!(GenDispatchACK(false), r#"{"op":12,"d":1}"#);
    }

    #[test]
    fn validation_ack_contains_signature() {
        let mut headers = HeaderMap::new();
        headers.insert(crate::signature::HeaderTimestamp, "1".parse().unwrap());
        let req = WHValidationReq {
            plain_token: "plain".to_string(),
            event_ts: "2".to_string(),
        };

        let body = GenValidationACK(&req, &headers, "secret").unwrap();
        let rsp: WHValidationRsp = serde_json::from_slice(&body).unwrap();
        assert_eq!(rsp.plain_token, "plain");
        assert!(!rsp.signature.is_empty());
    }

    #[test]
    fn http_handler_sets_app_id_session_for_dispatch() {
        let secret = "secret";
        let body = br#"{"op":0,"t":"WEBHOOK_TEST","d":{"hello":"world"}}"#;
        let mut headers = HeaderMap::new();
        headers.insert(crate::signature::HeaderTimestamp, "123456".parse().unwrap());
        let signature = crate::signature::Generate(secret, &headers, body).unwrap();
        headers.insert(crate::signature::HeaderSig, signature.parse().unwrap());

        crate::event::RegisterHandler(
            crate::models::gateway::WSDispatchEvent,
            "WEBHOOK_TEST",
            capture_session_app_id,
        );
        *captured_app_id().lock().unwrap() = None;

        let response = HTTPHandler(body, &headers, "app-id-1", secret).unwrap();

        assert_eq!(response, Some(GenDispatchACK(true).into_bytes()));
        assert_eq!(
            captured_app_id().lock().unwrap().as_deref(),
            Some("app-id-1")
        );
    }
}
