use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    // pub base_ref_name: String,
    // pub head_ref_name: String,
    pub head_ref_oid: String,
    // pub number: u32,
    // pub review_decision: String,
    // pub state: String,
    pub url: String,
}

pub fn pull_request(repo: &str, branch: &str) -> Option<PullRequest> {
    let result = Command::new("gh")
        .arg("--repo")
        .arg(repo)
        .arg("pr")
        .arg("view")
        .arg("--json")
        .arg("number,url,state,baseRefName,headRefName,headRefOid,reviewDecision")
        .arg(branch)
        .output()
        .expect("Failed to execute command");

    if !result.status.success() {
        return None;
    }

    match serde_json::from_slice::<PullRequest>(&result.stdout) {
        Ok(pr) => Some(pr),
        Err(_) => None,
    }
}
