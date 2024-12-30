use crate::error::{JZError, JZResult};
use crate::git::git::GitLocal;
use git2::Oid;

impl GitLocal {
    pub fn blob(&self, branch: String, sha: Option<String>, path: String) -> JZResult<Vec<u8>> {
        let refs = self
            .repository
            .find_branch(&branch, git2::BranchType::Local)
            .map_err(|x| JZError::Other(anyhow::anyhow!("[039] Branch Not Found: {}", x)))?;
        let commit = match sha {
            Some(sha) => {
                let commit = self
                    .repository
                    .find_commit(Oid::from_str(&sha).map_err(|x| {
                        JZError::Other(anyhow::anyhow!("[040] Commit Not Found: {}", x))
                    })?)
                    .map_err(|x| {
                        JZError::Other(anyhow::anyhow!("[041] Commit Not Found: {}", x))
                    })?;
                commit
            }
            None => {
                let commit = refs.into_reference().peel_to_commit().map_err(|x| {
                    JZError::Other(anyhow::anyhow!("[042] Commit Not Found: {}", x))
                })?;
                commit
            }
        };
        let tree = commit
            .tree()
            .map_err(|x| JZError::Other(anyhow::anyhow!("[043] Commit Not Found: {}", x)))?;
        let blob = tree
            .get_path(path.as_ref())
            .map_err(|x| JZError::Other(anyhow::anyhow!("[044] Commit Not Found: {}", x)))?;
        let blob = blob
            .to_object(&self.repository)
            .map_err(|x| JZError::Other(anyhow::anyhow!("[045] Commit Not Found: {}", x)))?;
        let context = blob.as_blob();
        if context.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[046] Commit Not Found")));
        }
        let context = context.unwrap();
        Ok(context.content().to_vec())
    }
}
