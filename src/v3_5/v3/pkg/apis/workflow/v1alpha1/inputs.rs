use serde::{Deserialize, Serialize};

/// Inputs are the mechanism for passing parameters, artifacts,
/// volumes from one template to another.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inputs {
    /// Artifact are a list of artifacts passed as inputs.
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<super::Artifact>>,

    /// Parameters are a list of parameters passed as inputs.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<super::Parameter>>,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            artifacts: None,
            parameters: None,
        }
    }
}
