use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowLevelArtifactGC {
    /// ForceFinalizerRemoval: if set to true, the finalizer 
    /// will be removed in the case that Artifact GC fails
    #[serde(rename = "forceFinalizerRemoval", skip_serializing_if = "Option::is_none")]
    pub force_finalizer_removal: Option<bool>,

    #[serde(rename = "podMetadata", skip_serializing_if = "Option::is_none")]
    pub pod_metadata: Option<Box<super::Metadata>>,
    
    /// PodSpecPatch holds strategic merge patch to apply against the artgc pod spec.
    #[serde(rename = "podSpecPatch", skip_serializing_if = "Option::is_none")]
    pub pod_spec_patch: Option<String>,

    /// ServiceAccountName is an optional field for specifying 
    /// the Service Account that should be assigned to the Pod doing the deletion
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,

    /// Strategy is the strategy to use.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl WorkflowLevelArtifactGC {
    /// WorkflowLevelArtifactGC describes how to delete artifacts from 
    /// completed Workflows - this spec is used on the Workflow level
    pub fn new() -> WorkflowLevelArtifactGC {
        WorkflowLevelArtifactGC {
            force_finalizer_removal: None,
            pod_metadata: None,
            pod_spec_patch: None,
            service_account_name: None,
            strategy: None,
        }
    }
}
