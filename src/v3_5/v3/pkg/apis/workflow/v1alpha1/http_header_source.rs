use serde::{Deserialize, Serialize};
use k8s_openapi::api::core;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPHeaderSource {
    #[serde(rename = "secretKeyRef", skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<Box<core::v1::SecretKeySelector>>,
}

impl HTTPHeaderSource {
    pub fn new() -> HTTPHeaderSource {
        HTTPHeaderSource {
            secret_key_ref: None,
        }
    }
}
