use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Revset {
    pub inner: String,
}

impl Revset {
    pub fn new(inner: &str) -> Self {
        Self {
            inner: inner.to_string(),
        }
    }
}
