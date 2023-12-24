use serde::{Deserialize, Serialize};

/// HTTPArtifact allows a file served on HTTP to be placed
/// as an input artifact in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPArtifact {
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<Box<super::HTTPAuth>>,
    
    /// Headers are an optional list of headers to send with
    /// HTTP requests for artifacts.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<super::Header>>,
    
    /// URL of the artifact
    #[serde(rename = "url")]
    pub url: String,
}

impl HTTPArtifact {
    pub fn new(url: String) -> HTTPArtifact {
        HTTPArtifact {
            auth: None,
            headers: None,
            url,
        }
    }
}
