use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,

    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    
    #[serde(rename = "labelsFrom", skip_serializing_if = "Option::is_none")]
    pub labels_from: Option<::std::collections::HashMap<String, super::LabelValueFrom>>,
}

impl WorkflowMetadata {
    pub fn new() -> WorkflowMetadata {
        WorkflowMetadata {
            annotations: None,
            labels: None,
            labels_from: None,
        }
    }
}
