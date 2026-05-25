//! Botgo-compatible inline search simulator.

#![allow(non_snake_case)]

use reqwest::header::HeaderMap;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub app_id: String,
    pub end_point: String,
    pub secret: String,
}

impl Config {
    pub fn new(
        app_id: impl Into<String>,
        end_point: impl Into<String>,
        secret: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            end_point: end_point.into(),
            secret: secret.into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct SearchInteraction<'a> {
    application_id: &'a str,
    #[serde(rename = "type")]
    interaction_type: crate::interaction::InteractionType,
    data: SearchInteractionData,
    version: u32,
}

#[derive(Debug, Serialize)]
struct SearchInteractionData {
    name: &'static str,
    #[serde(rename = "type")]
    data_type: crate::interaction::InteractionDataType,
    resolved: serde_json::Value,
}

pub async fn SimulateSearch(
    config: &Config,
    keyword: &str,
) -> crate::Result<crate::interaction::SearchRsp> {
    let body = serde_json::to_vec(&SearchInteraction {
        application_id: &config.app_id,
        interaction_type: crate::interaction::InteractionTypeCommand,
        data: SearchInteractionData {
            name: "search",
            data_type: crate::interaction::InteractionDataTypeChatSearch,
            resolved: serde_json::to_value(crate::interaction::SearchInputResolved {
                keyword: keyword.to_string(),
            })?,
        },
        version: 1,
    })?;
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let mut headers = HeaderMap::new();
    headers.insert(
        crate::signature::HeaderTimestamp,
        timestamp
            .parse()
            .map_err(|_| crate::BotError::invalid_data("invalid timestamp"))?,
    );
    let signature = crate::signature::Generate(&config.secret, &headers, &body)?;
    headers.insert(
        crate::signature::HeaderSig,
        signature
            .parse()
            .map_err(|_| crate::BotError::invalid_data("invalid signature header"))?,
    );

    let response = reqwest::Client::new()
        .post(&config.end_point)
        .headers(headers)
        .body(body)
        .send()
        .await?;
    let bytes = response.bytes().await?;
    serde_json::from_slice(&bytes).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_constructor_sets_fields() {
        let config = Config::new("app", "https://example.com", "secret");
        assert_eq!(config.app_id, "app");
        assert_eq!(config.end_point, "https://example.com");
        assert_eq!(config.secret, "secret");
    }
}
