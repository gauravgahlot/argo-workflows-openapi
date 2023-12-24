use serde::{Deserialize, Serialize};

/// RetryAffinity prevents running steps on the same host.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetryAffinity {
    /// RetryNodeAntiAffinity is a placeholder for future expansion,
    /// only empty nodeAntiAffinity is allowed. In order to prevent 
    /// running steps on the same host, it uses \"kubernetes.io/hostname\".
    #[serde(rename = "nodeAntiAffinity", skip_serializing_if = "Option::is_none")]
    pub node_anti_affinity: Option<serde_json::Value>,
}

impl RetryAffinity {
    pub fn new() -> RetryAffinity {
        RetryAffinity {
            node_anti_affinity: None,
        }
    }
}
