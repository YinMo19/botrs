use crate::models::gateway::WSPayload;

pub(crate) type EventParseFn = fn(&mut WSPayload, &[u8]) -> crate::Result<()>;
