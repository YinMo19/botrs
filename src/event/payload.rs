use crate::models::gateway::*;
use serde::de::DeserializeOwned;

pub(super) fn parse_data<T: DeserializeOwned>(message: &[u8]) -> crate::Result<T> {
    let value: serde_json::Value = serde_json::from_slice(message)?;
    serde_json::from_value(value.get("d").cloned().unwrap_or(serde_json::Value::Null))
        .map_err(Into::into)
}

pub(super) trait PayloadData: Sized {
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
            payload_event_id(payload),
            data,
        ))
    }
}
impl_constructed_payload_data!(
    crate::forum::Thread,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Thread::new(payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::forum::Post,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Post::new(payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::forum::Reply,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::forum::Reply::new(payload_event_id(payload), &data)
    }
);
impl_constructed_payload_data!(
    crate::interaction::Interaction,
    |payload: &WSPayload, data: serde_json::Value| {
        crate::interaction::Interaction::new(payload_event_id(payload), &data)
    }
);
