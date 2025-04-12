use crate::GitParam;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone)]
pub struct GitCommit {
    pub id: String,
    pub msg: String,
    pub author: String,
    pub email: String,
    pub date: i64,
}

#[derive(Deserialize)]
pub struct CreateGitCommit {
    pub branches: String,
    pub user: String,
    pub email: String,
    pub msg: String,
    pub path: String,
    pub file: String,
    pub ops: i32, // 1 add 2 del todo: 3 modify
    pub context: Vec<u8>,
}

impl GitParam {
    pub fn list_commit(&mut self, branches: Option<String>) -> anyhow::Result<Vec<GitCommit>> {
        let repo = self.repo()?;
        let mut commits = vec![];
        let refs = match branches {
            Some(x) => match repo.find_branch(&*x, git2::BranchType::Local) {
                Ok(branches) => branches.into_reference(),
                Err(_) => return Err(anyhow::anyhow!("branch not found")),
            },
            None => repo.head()?,
        };

        let mut root = refs.peel_to_commit()?;
        commits.push(GitCommit {
            id: root.id().to_string(),
            msg: root.message().unwrap_or("N/A").to_string(),
            author: root.author().name().unwrap_or("N/A").to_string(),
            email: root.author().email().unwrap_or("N/A").to_string(),
            date: root.time().seconds(),
        });
        while let Ok(commit) = root.parent(0) {
            commits.push(GitCommit {
                id: commit.id().to_string(),
                msg: commit.message().unwrap_or("N/A").to_string(),
                author: commit.author().name().unwrap_or("N/A").to_string(),
                email: commit.author().email().unwrap_or("N/A").to_string(),
                date: commit.time().seconds(),
            });
            if root.parent(0).is_err() {
                break;
            }
            root = commit;
        }
        Ok(commits)
    }
    pub fn create_commit(&mut self, param: CreateGitCommit) -> anyhow::Result<()> {
        let branches = param.branches;
        let user = param.user;
        let email = param.email;
        let path = param.path;
        let file = param.file;
        let content = param.context;
        let msg = param.msg;
        let repo = self.repo()?;
        let blob = repo.blob(&content)?;
        let file_path = PathBuf::from(path).join(file);
        let head_commit = match repo.find_branch(&branches, git2::BranchType::Local) {
            Ok(branch) => {
                let commit = branch.get().peel_to_commit()?;
                Some(commit)
            },
            Err(e) => {
                if e.code() == git2::ErrorCode::NotFound {
                    None
                } else {
                    return Err(anyhow::anyhow!("find_branch error"));
                }
            }
        };
        let mut tree_builder = match &head_commit {
            Some(commit) => {
                let tree = commit.tree()?;
                repo.treebuilder(Some(&tree))?
            }
            None => repo.treebuilder(None)?,
        };
        tree_builder.insert(file_path, blob, 0o100644)?;
        let tree = tree_builder.write()?;
        let tree = repo.find_tree(tree)?;
        let signature = git2::Signature::now(&user, &email)?;
        if let Some(head_commit) = head_commit {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &msg,
                &tree,
                &[&head_commit],
            )?;
        } else {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &msg,
                &tree,
                &[],
            )?;
        }
        Ok(())
    }

}
