//! Signed QQ HTTP callbacks. Serve the returned value as a JSON response.

use super::{Client, Context, EventHandler};
use crate::models::gateway::{GatewayEvent, opcodes};
use crate::{BotError, Result};
use data_encoding::{HEXLOWER, HEXLOWER_PERMISSIVE};
use reqwest::header::HeaderMap;
use ring::signature::{ED25519, Ed25519KeyPair, KeyPair, UnparsedPublicKey};
use serde::{Deserialize, Serialize};

/// Response body for a verified callback.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum WebhookReply {
    Validation {
        plain_token: String,
        signature: String,
    },
    Ack {
        op: u8,
        d: u64,
    },
}

pub(super) fn signing_key(secret: &str) -> Result<Ed25519KeyPair> {
    if secret.is_empty() {
        return Err(BotError::config("empty Webhook app secret"));
    }
    let mut seed = [0; 32];
    for (byte, source) in seed.iter_mut().zip(secret.bytes().cycle()) {
        *byte = source;
    }
    Ed25519KeyPair::from_seed_unchecked(&seed).map_err(|_| BotError::config("invalid Webhook key"))
}

fn signed_bytes(timestamp: &[u8], body: &[u8]) -> Vec<u8> {
    [timestamp, body].concat()
}

fn verify(key: &Ed25519KeyPair, headers: &HeaderMap, body: &[u8]) -> Result<()> {
    let timestamp = headers
        .get("X-Signature-Timestamp")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| BotError::auth("missing Webhook timestamp"))?;
    let signature = headers
        .get("X-Signature-Ed25519")
        .filter(|s| s.as_bytes().len() == 128)
        .and_then(|s| HEXLOWER_PERMISSIVE.decode(s.as_bytes()).ok())
        .ok_or_else(|| BotError::auth("invalid Webhook signature header"))?;
    UnparsedPublicKey::new(&ED25519, key.public_key())
        .verify(&signed_bytes(timestamp.as_bytes(), body), &signature)
        .map_err(|_| BotError::auth("invalid Webhook signature"))
}

impl<H: EventHandler + 'static> Client<H> {
    /// Verifies a callback over its original HTTP body, then handles its opcode.
    /// Return the serialized reply with HTTP 200. Authentication and malformed
    /// envelope errors should be mapped to HTTP 401 and 400 respectively.
    /// Handler failures are reported to the error hook and acknowledged for retry.
    pub async fn handle_webhook(&self, headers: &HeaderMap, body: &[u8]) -> Result<WebhookReply> {
        let key = &self.webhook_key;
        verify(key, headers, body)?;
        let event: GatewayEvent = serde_json::from_slice(body)?;
        match event.opcode {
            opcodes::CALLBACK_VALIDATION => {
                #[derive(Deserialize)]
                struct Challenge {
                    plain_token: String,
                    event_ts: String,
                }
                let challenge: Challenge = serde_json::from_value(event.data.unwrap_or_default())?;
                if challenge.event_ts.is_empty() || challenge.plain_token.is_empty() {
                    return Err(BotError::invalid_data("empty Webhook challenge"));
                }
                let signature = key.sign(&signed_bytes(
                    challenge.event_ts.as_bytes(),
                    challenge.plain_token.as_bytes(),
                ));
                Ok(WebhookReply::Validation {
                    plain_token: challenge.plain_token,
                    signature: HEXLOWER.encode(signature.as_ref()),
                })
            }
            opcodes::HEARTBEAT => {
                let seq = event
                    .data
                    .as_ref()
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| BotError::invalid_data("expected heartbeat sequence"))?;
                Ok(WebhookReply::Ack {
                    op: opcodes::HEARTBEAT_ACK,
                    d: seq,
                })
            }
            opcodes::DISPATCH => {
                let result = self
                    .handle_event(Context::new(self.api.clone()), event)
                    .await;
                let failed = result.is_err();
                if let Err(err) = result {
                    self.handler.error(err).await;
                }
                Ok(WebhookReply::Ack {
                    op: opcodes::CALLBACK_ACK,
                    d: u64::from(failed),
                })
            }
            op => Err(BotError::invalid_data(format!(
                "unsupported Webhook opcode {op}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Event, Intents, Token};
    use serde_json::{json, to_value};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Generated independently by botgo v0.2.1 interaction/signature.Generate.
    const VALIDATION: &[u8] = br#"{"op":13,"d":{"plain_token":"challenge","event_ts":"42"}}"#;
    const VALIDATION_SIGNATURE: &str = "84bd07ba1a8ac7953227f7b17e10c70bade26b3b3787a82a672de652acff7c52ef78c04c48a576cd5f1dc22564035bb6637808e3e824aa733c3b228e7e91380a";
    const CHALLENGE_SIGNATURE: &str = "f044061c95530e94a5ca0715f660c42d3827c1aed0156ff818b27aa4246b6cd51e4ca4aef553fda3668127254b2198e3263d99f8831f6ee84977e741c7b3f900";
    const HEARTBEAT: &[u8] = br#"{"op":1,"d":97}"#;
    const HEARTBEAT_SIGNATURE: &str = "93464a6ae6f8bfe6f16af05b1e23fc8c13f013f575613ce90a99f32eee1fa8238f2d599529f7f0a28924f76840eb94cfe66cf1dfb78cdf24c34d437c7b827c07";
    const RESUMED: &[u8] = br#"{"op":0,"t":"RESUMED","d":{}}"#;
    const RESUMED_SIGNATURE: &str = "5abfe8635cb2209f8691b1599f531650faf6182c11af926f10376fded1042b63fbcb1fa0ba203c0d97cdb97e8fa06428aaca0d66c3ad835e845d37ec8f536a00";
    const BAD_MESSAGE: &[u8] = br#"{"op":0,"t":"C2C_MESSAGE_CREATE","d":{}}"#;
    const BAD_MESSAGE_SIGNATURE: &str = "dfe9db2f16a5a53c096b92aeb6d84cfced4124cac85e11812724e5d8f1ab4cc0813183ee8241040a1888b71351a0250ba16156cc6f20bc89beccc658a3656105";

    fn headers(signature: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("x-signature-timestamp", "123".parse().unwrap());
        headers.insert("x-signature-ed25519", signature.parse().unwrap());
        headers
    }

    #[tokio::test]
    async fn verifies_botgo_callbacks_and_signs_challenge() {
        let client = Client::new(
            Token::new("test", "webhook-test"),
            Intents::new(),
            |event| async move {
                assert!(matches!(event, Event::Resumed(_)));
                Ok(())
            },
            false,
        )
        .unwrap();
        for (body, signature, expected) in [
            (
                VALIDATION,
                VALIDATION_SIGNATURE,
                json!({"plain_token":"challenge","signature":CHALLENGE_SIGNATURE}),
            ),
            (HEARTBEAT, HEARTBEAT_SIGNATURE, json!({"op":11,"d":97})),
            (RESUMED, RESUMED_SIGNATURE, json!({"op":12,"d":0})),
        ] {
            let response = client
                .handle_webhook(&headers(signature), body)
                .await
                .unwrap();
            assert_eq!(to_value(response).unwrap(), expected);
        }
    }

    #[derive(Default)]
    struct FailingHandler {
        calls: AtomicUsize,
        errors: AtomicUsize,
    }

    #[async_trait::async_trait]
    impl EventHandler for FailingHandler {
        async fn handle(&self, _: Event) -> Result<()> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            Err(BotError::internal("handler failed"))
        }
        async fn error(&self, _: BotError) {
            self.errors.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[tokio::test]
    async fn dispatch_reports_failures_and_rejects_changed_or_unsigned_requests() {
        let client = Client::new(
            Token::new("test", "webhook-test"),
            Intents::new(),
            FailingHandler::default(),
            false,
        )
        .unwrap();
        for (body, signature) in [
            (RESUMED, RESUMED_SIGNATURE),
            (BAD_MESSAGE, BAD_MESSAGE_SIGNATURE),
        ] {
            let reply = client
                .handle_webhook(&headers(signature), body)
                .await
                .unwrap();
            assert_eq!(to_value(reply).unwrap(), json!({"op":12,"d":1}));
        }
        for (h, body) in [
            (HeaderMap::new(), RESUMED),
            (headers(RESUMED_SIGNATURE), BAD_MESSAGE),
        ] {
            assert!(matches!(
                client.handle_webhook(&h, body).await,
                Err(BotError::Auth(_))
            ));
        }
        assert_eq!(client.handler.calls.load(Ordering::Relaxed), 1);
        assert_eq!(client.handler.errors.load(Ordering::Relaxed), 2);
    }
}
