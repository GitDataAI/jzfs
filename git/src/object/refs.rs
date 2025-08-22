use crate::GitContext;
use anyhow::anyhow;
use error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RefsItem {
    pub name: String,
    pub hash: String,
    pub is_head: bool,
    pub upstream: Option<String>,
}

impl GitContext {
    pub fn refs_list(&self) -> Result<Vec<RefsItem>, AppError> {
        let repo = self.repo()?;
        let mut branches = repo
            .branches(None)
            .map_err(|_| AppError::from(anyhow!("get refs error")))?;
        let mut result = vec![];
        while let Some(Ok((branch, _))) = branches.next() {
            let is_head = branch.is_head();
            let name = branch.name().unwrap_or(None).unwrap_or("None").to_string();
            let refs = branch.get().target();
            if name != "None"
                && let Some(head) = refs
            {
                let head = head.to_string();
                let upstream = if let Ok(upstream) = branch.upstream() {
                    match upstream.name() {
                        Ok(x) => x.map(|x| x.to_string()),
                        Err(_) => None,
                    }
                } else {
                    None
                };
                result.push(RefsItem {
                    name,
                    hash: head,
                    is_head,
                    upstream,
                })
            }
        }
        Ok(result)
    }
    pub fn refs_rename(&self, old_name: &str, new_name: &str) -> Result<(), AppError> {
        let repo = self.repo()?;
        let mut branch = repo.find_branch(old_name, git2::BranchType::Local)?;
        if repo.find_branch(new_name, git2::BranchType::Local).is_ok() {
            return Err(AppError::from(anyhow!("branch already exists")));
        };
        branch.rename(new_name, true)?;
        Ok(())
    }
    pub fn refs_delete(&self, name: &str) -> Result<(), AppError> {
        let repo = self.repo()?;
        let mut branch = repo.find_branch(name, git2::BranchType::Local)?;
        branch.delete()?;
        Ok(())
    }
    pub fn refs_exchange_head(&self, name: &str) -> Result<(), AppError> {
        let mut name = name.to_string();
        let repo = self.repo()?;
        if !name.starts_with("refs/heads/") {
            name = format!("refs/heads/{}", name);
        };
        repo.set_head(&name)?;
        Ok(())
    }
}
