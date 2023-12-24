use serde::{Deserialize, Serialize};
use k8s_openapi::api::core;

/// Cache is the configuration for the type of cache to be used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
    #[serde(rename = "configMap")]
    pub config_map: Box<core::v1::ConfigMapKeySelector>,
}

impl Cache {
    pub fn new(config_map: core::v1::ConfigMapKeySelector) -> Cache {
        Cache {
            config_map: Box::new(config_map),
        }
    }
}
