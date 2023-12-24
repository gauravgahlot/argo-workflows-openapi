use serde::{Deserialize, Serialize};

/// Pod metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            annotations: None,
            labels: None,
        }
    }
}
