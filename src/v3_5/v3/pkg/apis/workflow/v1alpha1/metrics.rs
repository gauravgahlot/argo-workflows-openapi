use serde::{Deserialize, Serialize};

/// Metrics are a list of metrics emitted from a Workflow/Template.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    /// Prometheus is a list of prometheus metrics to be emitted.
    #[serde(rename = "prometheus")]
    pub prometheus: Vec<super::Prometheus>,
}

impl Metrics {
    pub fn new(prometheus: Vec<super::Prometheus>) -> Metrics {
        Metrics {
            prometheus,
        }
    }
}
