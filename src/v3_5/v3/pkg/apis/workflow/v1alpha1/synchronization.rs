use serde::{Deserialize, Serialize};

/// Synchronization holds synchronization lock configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Synchronization {
    #[serde(rename = "mutex", skip_serializing_if = "Option::is_none")]
    pub mutex: Option<Box<super::Mutex>>,

    #[serde(rename = "semaphore", skip_serializing_if = "Option::is_none")]
    pub semaphore: Option<Box<super::SemaphoreRef>>,
}

impl Synchronization {
    pub fn new() -> Synchronization {
        Synchronization {
            mutex: None,
            semaphore: None,
        }
    }
}
