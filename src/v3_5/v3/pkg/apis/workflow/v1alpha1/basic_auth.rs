use serde::{Deserialize, Serialize};
use k8s_openapi::api::core;

/// BasicAuth describes the secret selectors required for basic authentication.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicAuth {
    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "usernameSecret", skip_serializing_if = "Option::is_none")]
    pub username_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl BasicAuth {    
    pub fn new() -> BasicAuth {
        BasicAuth {
            password_secret: None,
            username_secret: None,
        }
    }
}
