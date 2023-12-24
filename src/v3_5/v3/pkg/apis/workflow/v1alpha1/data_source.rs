use serde::{Deserialize, Serialize};

/// DataSource sources external data into a data template.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(rename = "artifactPaths", skip_serializing_if = "Option::is_none")]
    pub artifact_paths: Option<Box<super::ArtifactPaths>>,
}

impl DataSource {
    pub fn new() -> DataSource {
        DataSource {
            artifact_paths: None,
        }
    }
}
