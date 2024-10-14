use crate::client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub default_path: String,
    pub url: String,
}

impl Repository {
    pub fn new() -> Result<Self, String> {
        let default_path = client::config_value("paths.default")?;
        let url = match default_path.strip_suffix(".git") {
            Some(url) => url.to_string(),
            None => return Err("Could not get the url".to_string()),
        };

        Ok(Self { default_path, url })
    }
}
