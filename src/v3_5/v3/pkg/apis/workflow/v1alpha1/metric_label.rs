use serde::{Deserialize, Serialize};

/// MetricLabel is a single label for a prometheus metric.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricLabel {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

impl MetricLabel {
    pub fn new(key: String, value: String) -> MetricLabel {
        MetricLabel {
            key,
            value,
        }
    }
}
