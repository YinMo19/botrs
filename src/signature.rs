//! Ed25519 signature helpers for interaction callbacks.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use reqwest::header::HeaderMap;

pub const HEADER_SIGNATURE: &str = "X-Signature-Ed25519";
pub const HEADER_TIMESTAMP: &str = "X-Signature-Timestamp";

fn seed(secret: &str) -> crate::Result<[u8; 32]> {
    if secret.is_empty() {
        return Err(crate::BotError::invalid_data("secret invalid"));
    }

    let mut repeated = secret.as_bytes().to_vec();
    while repeated.len() < 32 {
        repeated.extend_from_slice(secret.as_bytes());
    }

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&repeated[..32]);
    Ok(seed)
}

fn original_content(timestamp: &str, body: &[u8]) -> crate::Result<Vec<u8>> {
    if timestamp.is_empty() {
        return Err(crate::BotError::invalid_data("timestamp is nil"));
    }
    let mut content = Vec::with_capacity(timestamp.len() + body.len());
    content.extend_from_slice(timestamp.as_bytes());
    content.extend_from_slice(body);
    Ok(content)
}

fn header_value(headers: &HeaderMap, name: &str) -> String {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

pub fn generate(secret: &str, headers: &HeaderMap, http_body: &[u8]) -> crate::Result<String> {
    let key = SigningKey::from_bytes(&seed(secret)?);
    let content = original_content(&header_value(headers, HEADER_TIMESTAMP), http_body)?;
    Ok(hex::encode(key.sign(&content).to_bytes()))
}

pub fn verify(secret: &str, headers: &HeaderMap, http_body: &[u8]) -> crate::Result<bool> {
    let signature = header_value(headers, HEADER_SIGNATURE);
    if signature.is_empty() {
        return Err(crate::BotError::invalid_data("not found signature"));
    }

    let signature = hex::decode(signature).map_err(|err| {
        crate::BotError::invalid_data(format!("hex decode signature failed: {err}"))
    })?;
    let signature = Signature::from_slice(&signature)
        .map_err(|_| crate::BotError::invalid_data("signature decode result is not a valid buf"))?;
    let verifying_key = VerifyingKey::from(&SigningKey::from_bytes(&seed(secret)?));
    let content = original_content(&header_value(headers, HEADER_TIMESTAMP), http_body)?;

    Ok(verifying_key.verify(&content, &signature).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_signature_verifies() {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_TIMESTAMP, "123456".parse().unwrap());
        let body = br#"{"hello":"world"}"#;

        let signature = generate("secret", &headers, body).unwrap();
        headers.insert(HEADER_SIGNATURE, signature.parse().unwrap());

        assert!(verify("secret", &headers, body).unwrap());
        assert!(!verify("secret", &headers, b"changed").unwrap());
    }
}
