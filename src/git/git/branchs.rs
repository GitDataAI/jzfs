use crate::error::JZResult;
use crate::git::git::options::Branchs;
use crate::git::git::GitLocal;
use git2::BranchType;
use log::info;

impl GitLocal {
    pub fn branch_list(&self) -> JZResult<Vec<Branchs>> {
        let references = self.repository.references()?;
        let mut remote_result = vec![];
        let remote = self.repository.remotes()?;
        for idx in 0..remote.len() {
            if let Some(remote) = remote.get(idx) {
                if let Ok(remote) = self.repository.find_remote(remote) {
                    if let Some(url) = remote.url() {
                        remote_result
                            .push((remote.name().unwrap_or("").to_string(), url.to_string()))
                    }
                }
            } else {
                continue;
            }
        }
        let mut result = vec![];
        for reference in references {
            if let Ok(refs) = reference {
                let name = match refs
                    .name()
                    .ok_or(git2::Error::from_str("Failed to get reference name"))
                {
                    Ok(name) => name.to_string(),
                    Err(err) => {
                        info!("Failed to get reference name: {}", err);
                        continue;
                    }
                };
                let target = refs.target();
                if let Some(target) = target {
                    result.push(Branchs {
                        name: name
                            .replace("refs/tags/", "")
                            .replace("refs/remotes/", "")
                            .replace("refs/heads/", ""),
                        head: target.to_string(),
                        local: !refs.is_remote(),
                        remote_url: if refs.is_remote() {
                            Some(remote_result.clone())
                        } else {
                            None
                        },
                    })
                }
            }
        }
        Ok(result)
    }
    pub fn branch_new(&self, name: &str, head: &str) -> JZResult<()> {
        let head = self.repository.find_reference(head)?;
        let commit = head.peel_to_commit()?;
        self.repository.branch(name, &commit, true)?;
        Ok(())
    }
    pub fn branch_delete(&self, name: &str) -> JZResult<()> {
        let mut branch = self.repository.find_branch(name, BranchType::Local)?;
        branch.delete()?;
        Ok(())
    }
    pub fn branch_rename(&self, name: &str, new_name: &str, force: bool) -> JZResult<()> {
        let mut branch = self.repository.find_branch(name, BranchType::Local)?;
        branch.rename(new_name, force)?;
        Ok(())
    }
}
