use serde::{Deserialize, Serialize};

/// Identifies a specific API for permission demand requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct APIPermissionDemandIdentify {
    /// The API path/endpoint
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// The HTTP method for this API
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub method: String,
}
