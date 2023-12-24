use serde::{Deserialize, Serialize};

/// ExecutorConfig holds configurations of an executor container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutorConfig {
    /// ServiceAccountName specifies the service account name of the executor container.
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
}

impl ExecutorConfig {
    pub fn new() -> ExecutorConfig {
        ExecutorConfig {
            service_account_name: None,
        }
    }
}
