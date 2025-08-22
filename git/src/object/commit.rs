use crate::GitContext;
use error::AppError;
use git2::Oid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct CommitPaginator {
    pub start_oid: Option<String>,
    pub end_oid: Option<String>,
    pub refs: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommitItem {
    pub tree_oid: String,
    pub commit_oid: String,
    pub author: Signature,
    pub committer: Signature,
    pub message: String,
    pub parents: Vec<String>,
    pub time: i64,
    pub offset_date: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

impl GitContext {
    pub fn commit_list(&self, param: CommitPaginator) -> Result<Vec<CommitItem>, AppError> {
        let repo = self.repo()?;
        let refs = match param.refs {
            None => repo.head()?,
            Some(refs) => repo
                .find_branch(&refs, git2::BranchType::Local)?
                .into_reference(),
        };
        let mut commit = refs.peel_to_commit()?;
        if let Some(end_commit) = param.end_oid.clone() {
            let end_commit_oid = Oid::from_str(&end_commit)?;
            let end_commit = repo.find_commit(end_commit_oid)?;
            commit = end_commit;
        }
        let mut result = vec![];
        while let Ok(parent) = commit.parent(0) {
            let tree_oid = commit.tree_id().to_string();
            let commit_oid = commit.id().to_string();
            let message = commit.message().unwrap_or("nil").to_string();
            let author_email = commit.author().email().unwrap_or("nil").to_string();
            let author_name = commit.author().name().unwrap_or("nil").to_string();
            let commiter_email = commit.committer().email().unwrap_or("nil").to_string();
            let commiter_name = commit.committer().name().unwrap_or("nil").to_string();
            let parents = commit
                .parents()
                .map(|x| x.id().to_string())
                .collect::<Vec<_>>();
            let time = commit.time().seconds();
            let offset_date = commit.time().offset_minutes();
            result.push(CommitItem {
                tree_oid,
                commit_oid: commit_oid.clone(),
                author: Signature {
                    name: author_name,
                    email: author_email,
                },
                committer: Signature {
                    name: commiter_name,
                    email: commiter_email,
                },
                message,
                parents,
                time,
                offset_date,
            });
            if let Some(start) = param.start_oid.clone() {
                if commit_oid == start {
                    break;
                }
            }
            if parent.parent_count() == 0 {
                break;
            }
            commit = parent;
        }
        Ok(result)
    }
}

#[test]
fn test_commit() {
    use std::path::PathBuf;
    let ctx = GitContext {
        path_dir: PathBuf::from("E:\\Code\\acl-anthology.git"),
    };
    let r = ctx
        .commit_list(CommitPaginator {
            start_oid: None,
            end_oid: None,
            refs: None,
        })
        .unwrap_or(vec![]);
    dbg!(r.first().unwrap());
}
