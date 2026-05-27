//! Interaction webhook helpers.

use reqwest::header::HeaderMap;
use serde::Serialize;

use crate::models::gateway::{HTTP_CALLBACK_ACK, WS_HEARTBEAT_ACK};
use crate::models::webhook::{WebhookValidationRequest, WebhookValidationResponse};

#[derive(Debug, Clone, Copy, Serialize)]
struct Ack {
    op: u8,
    d: u32,
}

pub fn heartbeat_ack(seq: u32) -> String {
    serde_json::to_string(&Ack {
        op: WS_HEARTBEAT_ACK,
        d: seq,
    })
    .expect("heartbeat ack is serializable")
}

pub fn dispatch_ack(success: bool) -> String {
    serde_json::to_string(&Ack {
        op: HTTP_CALLBACK_ACK,
        d: u32::from(!success),
    })
    .expect("dispatch ack is serializable")
}

pub fn validation_ack(
    req: &WebhookValidationRequest,
    headers: &HeaderMap,
    secret: &str,
) -> crate::Result<Vec<u8>> {
    let mut headers = headers.clone();
    headers.insert(
        crate::signature::HEADER_TIMESTAMP,
        req.event_ts
            .parse()
            .map_err(|_| crate::BotError::invalid_data("invalid event timestamp header"))?,
    );
    let signature = crate::signature::generate(secret, &headers, req.plain_token.as_bytes())?;
    serde_json::to_vec(&WebhookValidationResponse {
        plain_token: req.plain_token.clone(),
        signature,
        data_version: String::new(),
    })
    .map_err(Into::into)
}

pub fn handle_http_callback(
    body: &[u8],
    headers: &HeaderMap,
    app_id: impl Into<String>,
    secret: &str,
) -> crate::Result<Option<Vec<u8>>> {
    if !crate::signature::verify(secret, headers, body)? {
        return Err(crate::BotError::auth("signature verify failed"));
    }

    let mut payload: crate::models::gateway::WSPayload = serde_json::from_slice(body)?;
    payload.raw_message = Some(body.to_vec());
    payload.session = Some(crate::session_manager::Session::from_app_id(app_id));
    let trace_id = headers
        .get(crate::constant::HEADER_TRACE_ID)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if payload.base.op_code == crate::models::gateway::HTTP_CALLBACK_VALIDATION {
        let data = payload
            .data
            .as_ref()
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let req: WebhookValidationRequest = serde_json::from_value(data)?;
        return validation_ack(&req, headers, secret).map(Some);
    }

    match payload.base.op_code {
        crate::models::gateway::WS_HEARTBEAT => {
            let seq = payload
                .data
                .as_ref()
                .and_then(|value| value.as_u64())
                .unwrap_or_default() as u32;
            Ok(Some(heartbeat_ack(seq).into_bytes()))
        }
        crate::models::gateway::WS_DISPATCH_EVENT => {
            match crate::event::parse_and_handle(&mut payload) {
                Ok(()) => Ok(Some(dispatch_ack(true).into_bytes())),
                Err(err) => {
                    tracing::error!("parse_and_handle failed, {}, traceID:{}", err, trace_id);
                    Ok(Some(dispatch_ack(false).into_bytes()))
                }
            }
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ack_payloads_match_expected_shape() {
        assert_eq!(heartbeat_ack(7), r#"{"op":11,"d":7}"#);
        assert_eq!(dispatch_ack(true), r#"{"op":12,"d":0}"#);
        assert_eq!(dispatch_ack(false), r#"{"op":12,"d":1}"#);
    }

    #[test]
    fn validation_ack_contains_signature() {
        let mut headers = HeaderMap::new();
        headers.insert(crate::signature::HEADER_TIMESTAMP, "1".parse().unwrap());
        let req = WebhookValidationRequest {
            plain_token: "plain".to_string(),
            event_ts: "2".to_string(),
        };

        let body = validation_ack(&req, &headers, "secret").unwrap();
        let rsp: WebhookValidationResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(rsp.plain_token, "plain");
        assert!(!rsp.signature.is_empty());
    }

    #[test]
    fn http_handler_acks_valid_dispatch() {
        let secret = "secret";
        let body = br#"{"op":0,"t":"MESSAGE_CREATE","d":{"id":"1","content":"hello"}}"#;
        let mut headers = HeaderMap::new();
        headers.insert(
            crate::signature::HEADER_TIMESTAMP,
            "123456".parse().unwrap(),
        );
        let signature = crate::signature::generate(secret, &headers, body).unwrap();
        headers.insert(
            crate::signature::HEADER_SIGNATURE,
            signature.parse().unwrap(),
        );

        let response = handle_http_callback(body, &headers, "app-id-1", secret).unwrap();

        assert_eq!(response, Some(dispatch_ack(true).into_bytes()));
    }
}
