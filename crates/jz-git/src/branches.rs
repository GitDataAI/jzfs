use crate::GitParam;
use git2::Reference;
use serde::Serialize;

#[derive(Serialize,Clone)]
pub struct GitBranches {
    pub name: String,
    pub is_head: bool,
    pub upstream: Option<String>,
    pub active: Option<GitBranchesActive>,
}

#[derive(Serialize,Clone)]
pub struct GitBranchesActive {
    pub id: String,
    pub msg: String,
    pub author: String,
    pub email: String,
    pub date: i64,
}

impl GitParam {
    pub fn list_branches(&mut self) -> anyhow::Result<Vec<GitBranches>> {
        let repo = self.repo()?;
        let mut result = vec![];
        for branch in repo.branches(None)? {
            let (branch, _) = branch?;
            let name = match branch.name() {
                Ok(name) => name.unwrap_or("").to_string(),
                Err(_) => continue,
            };
            let is_head = branch.is_head();
            let upstream = match branch.upstream() {
                Ok(upstream) => upstream.name().unwrap_or(Some("")).map(|x| x.to_string()),
                Err(_) => None,
            };
            let reference = branch.into_reference();
            let commit = match reference.peel_to_commit() {
                Ok(commit) => {
                    let id = commit.id().to_string();
                    let msg = commit.message().unwrap_or("N/A").to_string();
                    let author = commit.author().name().unwrap_or("N/A").to_string();
                    let email = commit.author().email().unwrap_or("N/A").to_string();
                    let date = commit.time().seconds();
                    Some(GitBranchesActive {
                        id,
                        msg,
                        author,
                        email,
                        date,
                    })
                }
                Err(_) => None,
            };
            result.push(GitBranches {
                name,
                is_head,
                upstream,
                active: commit,
            });
        }
        Ok(result)
    }
    pub fn create_branch(&mut self, name: String, from: Option<String>) -> anyhow::Result<()> {
        let repo = self.repo()?;
        let branch = repo.find_branch(name.as_str(), git2::BranchType::Local);
        if branch.is_ok() {
            return Err(anyhow::anyhow!("branch already exists"));
        }
        let from = match from {
            Some(from) => from,
            None => {
                let head = repo.head()?;
                head.target().unwrap().to_string()
            }
        };
        let ns = format!("refs/heads/{}", name);
        if !Reference::is_valid_name(ns.as_str()) {
            return Err(anyhow::anyhow!("branch name is invalid"));
        }
        let from = match repo.find_reference(from.as_str()) {
            Ok(from) => from,
            Err(_) => {
                return Err(anyhow::anyhow!("from is invalid"));
            }
        };
        let head = match from.peel_to_commit() {
            Ok(head) => head,
            Err(_) => {
                return Err(anyhow::anyhow!("from is invalid"));
            }
        };
        match repo.branch(name.as_str(), &head, true) {
            Ok(_) => {}
            Err(_) => {
                return Err(anyhow::anyhow!("branch create failed"));
            }
        }
        Ok(())
    }
    pub fn delete_branch(&mut self, name: String) -> anyhow::Result<()> {
        let repo = self.repo()?;
        let branch = repo.find_branch(name.as_str(), git2::BranchType::Local);
        if branch.is_err() {
            return Err(anyhow::anyhow!("branch not found"));
        }
        let mut branch = branch?;
        branch.delete()?;
        Ok(())
    }
    pub fn rename_branch(&mut self, name: String, new_name: String) -> anyhow::Result<()> {
        let repo = self.repo()?;
        let branch = repo.find_branch(name.as_str(), git2::BranchType::Local);
        if branch.is_err() {
            return Err(anyhow::anyhow!("branch not found"));
        }
        if repo
            .find_branch(new_name.as_str(), git2::BranchType::Local)
            .is_ok()
        {
            return Err(anyhow::anyhow!("branch already exists"));
        }
        let mut branch = branch?;
        branch.rename(new_name.as_str(), true)?;
        Ok(())
    }
    pub fn checkout_head(&mut self, name: String) -> anyhow::Result<()> {
        let repo = self.repo()?;
        let branch = repo.find_branch(name.as_str(), git2::BranchType::Local);
        if branch.is_err() {
            return Err(anyhow::anyhow!("branch not found"));
        }
        let branch = branch?;
        let reference = branch.into_reference();
        let commit = match reference.peel_to_commit() {
            Ok(commit) => commit,
            Err(_) => {
                return Err(anyhow::anyhow!("branch not found"));
            }
        };
        repo.set_head(reference.name().unwrap())?;
        repo.checkout_tree(&commit.into_object(), None)?;
        Ok(())
    }
}
