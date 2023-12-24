use serde::{Deserialize, Serialize};

/// TarStrategy will tar and gzip the file or directory when saving.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TarStrategy {
    /// CompressionLevel specifies the gzip compression level 
    /// to use for the artifact. Defaults to gzip.DefaultCompression.
    #[serde(rename = "compressionLevel", skip_serializing_if = "Option::is_none")]
    pub compression_level: Option<i32>,
}

impl TarStrategy {
    /// TarStrategy will tar and gzip the file or directory when saving.
    pub fn new() -> TarStrategy {
        TarStrategy {
            compression_level: None,
        }
    }
}
