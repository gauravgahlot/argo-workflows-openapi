use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Arguments {
    /// Artifacts is the list of artifacts to pass to the template or workflow.
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<super::Artifact>>,

    /// Parameters is the list of parameters to pass to the template or workflow.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<super::Parameter>>,
}
