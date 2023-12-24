use serde::{Deserialize, Serialize};

/// Mutex holds Mutex configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mutex {
    /// name of the mutex
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Namespace is the namespace of the mutex,
    /// default: [namespace of workflow].
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl Mutex {
    /// Mutex holds Mutex configuration
    pub fn new() -> Mutex {
        Mutex {
            name: None,
            namespace: None,
        }
    }
}
