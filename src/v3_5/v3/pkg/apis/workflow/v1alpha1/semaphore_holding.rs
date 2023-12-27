use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SemaphoreHolding {
    /// Holders stores the list of current holder names in the 
    /// io.argoproj.workflow.v1alpha1.
    #[serde(rename = "holders", skip_serializing_if = "Option::is_none")]
    pub holders: Option<Vec<String>>,
    
    /// Semaphore stores the semaphore name.
    #[serde(rename = "semaphore", skip_serializing_if = "Option::is_none")]
    pub semaphore: Option<String>,
}

impl SemaphoreHolding {
    pub fn new() -> SemaphoreHolding {
        SemaphoreHolding {
            holders: None,
            semaphore: None,
        }
    }
}
