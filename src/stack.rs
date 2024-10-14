use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::commit::{Commit, CommitList};
use crate::revset::Revset;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub date: DateTime<Utc>,
    pub head: Commit,
    pub child_head: Commit,
    pub commits: Vec<Commit>,
}

impl Change {
    pub fn compare_url(&self, repo_url: &str) -> String {
        if self.child_head.phase == "public" {
            format!("{}/compare/{}", repo_url, self.head.name().unwrap())
        } else {
            format!(
                "{}/compare/{}...{}",
                repo_url,
                self.child_head.name().unwrap(),
                self.head.name().unwrap()
            )
        }
    }
}

pub fn build_stack() -> Result<Vec<Change>, String> {
    let mut stack = Vec::new();
    let base_commit = match Commit::try_from(Revset::new("bottom^")) {
        Ok(commit) => commit,
        Err(e) => {
            // If the base commit is public, we can't build a stack. This is generally when there
            // is only one commit in the repo and we cat get a parent.
            if e.trim() == "abort: current commit is public" {
                return Ok(stack);
            }

            println!("Error: '{}'", e);

            return Err(e);
        }
    };

    let book_stack = CommitList::try_from(Revset::new("bottom::top and bookmark()"))?;
    for (i, head) in book_stack.iter().enumerate() {
        let base = if i == 0 {
            &base_commit.clone()
        } else {
            &book_stack[i - 1]
        };

        let revset = format!("{}::{} - {}", base.node, head.node, base.node);
        let commits = CommitList::try_from(Revset::new(&revset))?;

        stack.push(Change {
            date: Utc::now(),
            child_head: base.clone(),
            head: head.clone(),
            commits,
        });
    }

    Ok(stack)
}
