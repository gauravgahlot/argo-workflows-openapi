use serde::{Deserialize, Serialize};

/// Memoization enables caching for the Outputs of the template
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Memoize {
    #[serde(rename = "cache")]
    pub cache: Box<super::Cache>,

    /// Key is the key to use as the caching key
    #[serde(rename = "key")]
    pub key: String,
    
    /// MaxAge is the maximum age (e.g. \"180s\", \"24h\") of an entry that is still
    /// considered valid. If an entry is older than the MaxAge, it will be ignored.
    #[serde(rename = "maxAge")]
    pub max_age: String,
}

impl Memoize {
    pub fn new(cache: super::Cache, key: String, max_age: String) -> Memoize {
        Memoize {
            cache: Box::new(cache),
            key,
            max_age,
        }
    }
}
