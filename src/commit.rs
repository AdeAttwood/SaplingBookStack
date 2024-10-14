use crate::revset::Revset;
use serde::{Deserialize, Serialize};

const TEMPLATE: &str =
    "{dict(phase, bookmarks, github_pull_request_number, node, short_node=node|short, title=desc|firstline) | json}";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commit {
    pub node: String,
    pub short_node: String,
    pub title: String,
    pub phase: String,
    pub bookmarks: Vec<String>,
    pub github_pull_request_number: Option<u32>,
}

impl Commit {
    pub fn name(&self) -> Result<String, String> {
        match self.bookmarks.first() {
            Some(name) => Ok(name.clone()),
            None => Err("No bookmark found, could not get a name".to_string()),
        }
    }
}

pub type CommitList = Vec<Commit>;

impl TryFrom<Revset> for Commit {
    type Error = String;

    fn try_from(value: Revset) -> Result<Self, Self::Error> {
        crate::client::sl::<Commit>(&value.inner, TEMPLATE)
    }
}

impl TryFrom<Revset> for CommitList {
    type Error = String;

    fn try_from(value: Revset) -> Result<Self, Self::Error> {
        crate::client::sl_list::<Commit>(&value.inner, TEMPLATE)
    }
}
