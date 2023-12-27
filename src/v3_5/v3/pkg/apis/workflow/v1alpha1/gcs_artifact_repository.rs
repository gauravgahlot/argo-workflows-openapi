use serde::{Deserialize, Serialize};
use k8s_openapi::api::core;

/// GCSArtifactRepository defines the controller configuration for a GCS
/// artifact repository.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCSArtifactRepository {
    /// Bucket is the name of the bucket.
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// KeyFormat defines the format of how to store keys and can reference
    /// workflow variables.
    #[serde(rename = "keyFormat", skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,

    #[serde(rename = "serviceAccountKeySecret", skip_serializing_if = "Option::is_none")]
    pub service_account_key_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl GCSArtifactRepository {
    pub fn new() -> GCSArtifactRepository {
        GCSArtifactRepository {
            bucket: None,
            key_format: None,
            service_account_key_secret: None,
        }
    }
}
