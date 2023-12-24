use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManifestFrom {
    #[serde(rename = "artifact")]
    pub artifact: Box<super::Artifact>,
}

impl ManifestFrom {
    pub fn new(artifact: super::Artifact) -> ManifestFrom {
        ManifestFrom {
            artifact: Box::new(artifact),
        }
    }
}
