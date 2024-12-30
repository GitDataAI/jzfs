use crate::error::{JZError, JZResult};
use crate::git::git::options::BlobTreeMsg;
use crate::server::MetaData;

impl MetaData {
    pub async fn repo_tree(
        &self,
        owner: String,
        name: String,
        branch: String,
    ) -> JZResult<BlobTreeMsg> {
        let info = match self.repo_info(owner, name).await {
            Ok(tree) => Ok(tree),
            Err(e) => Err(JZError::Other(anyhow::anyhow!("[039] {}", e.to_string()))),
        };
        let local = match self.git.open_repo(info?.uid.to_string()) {
            Ok(repo) => repo,
            Err(e) => return Err(JZError::Other(anyhow::anyhow!("[040] {}", e.to_string()))),
        };
        match local.build_tree_msg(branch, None) {
            Ok(tree) => Ok(tree),
            Err(e) => Err(JZError::Other(anyhow::anyhow!("[041] {}", e.to_string()))),
        }
    }
    pub async fn repo_tree_sha(
        &self,
        owner: String,
        name: String,
        branch: String,
        sha: String,
    ) -> JZResult<BlobTreeMsg> {
        let info = match self.repo_info(owner, name).await {
            Ok(tree) => Ok(tree),
            Err(e) => Err(JZError::Other(anyhow::anyhow!("[042] {}", e.to_string()))),
        };
        let local = match self.git.open_repo(info?.uid.to_string()) {
            Ok(repo) => repo,
            Err(e) => return Err(JZError::Other(anyhow::anyhow!("[043] {}", e.to_string()))),
        };
        match local.build_tree_msg(branch, Some(sha)) {
            Ok(tree) => Ok(tree),
            Err(e) => Err(JZError::Other(anyhow::anyhow!("[044] {}", e.to_string()))),
        }
    }
}
