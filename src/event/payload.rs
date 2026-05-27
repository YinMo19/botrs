use crate::api::BotApi;
use crate::http::HttpClient;
use crate::models::gateway::*;
use serde::de::DeserializeOwned;

pub fn parse_data<T: DeserializeOwned>(message: &[u8]) -> crate::Result<T> {
    let value: serde_json::Value = serde_json::from_slice(message)?;
    serde_json::from_value(value.get("d").cloned().unwrap_or(serde_json::Value::Null))
        .map_err(Into::into)
}

pub trait PayloadData: Sized {
    fn parse_from_payload(payload: &WSPayload, message: &[u8]) -> crate::Result<Self>;
}

impl<T> PayloadData for T
where
    T: DeserializeOwned,
{
    fn parse_from_payload(_payload: &WSPayload, message: &[u8]) -> crate::Result<Self> {
        parse_data(message)
    }
}

fn payload_data(message: &[u8]) -> crate::Result<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_slice(message)?;
    Ok(value.get("d").cloned().unwrap_or(serde_json::Value::Null))
}

fn payload_event_id(payload: &WSPayload) -> Option<String> {
    payload.base.event_id.clone()
}

fn event_api() -> BotApi {
    BotApi::new(HttpClient::new(crate::DEFAULT_TIMEOUT, false).expect("valid default api client"))
}

macro_rules! impl_constructed_payload_data {
    ($ty:ty, $ctor:expr) => {
        impl PayloadData for $ty {
            fn parse_from_payload(payload: &WSPayload, message: &[u8]) -> crate::Result<Self> {
                let data = payload_data(message)?;
                Ok($ctor(payload, data))
            }
        }
    };
}

impl PayloadData for crate::reaction::Reaction {
    fn parse_from_payload(payload: &WSPayload, message: &[u8]) -> crate::Result<Self> {
        let data = parse_data(message)?;
        Ok(crate::reaction::Reaction::from_message_reaction(
            event_api(),
            payload_event_id(payload),
            data,
        ))
    }
}
impl_constructed_payload_data!(
    crate::forum::Thread,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Thread::new(event_api(), payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::forum::Post,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Post::new(event_api(), payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::forum::Reply,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Reply::new(event_api(), payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::interaction::Interaction,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::interaction::Interaction::new(event_api(), payload_event_id(payload), &data)
    }
);
