use crate::error::{JZError, JZResult};
use crate::server::MetaData;
use anyhow::anyhow;
use uuid::Uuid;

impl MetaData {
    pub async fn _sync_repo(&self, repo_id: Uuid) -> JZResult<()> {
        let repo = match self.git.open_repo(repo_id.to_string()) {
            Ok(repo) => repo,
            Err(err) => return Err(JZError::Other(anyhow!(err.to_string()))),
        };
        let branch = repo.branch_list()?;
        match self.repo_branch_sync(repo_id).await {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
        for branch in branch {
            match self.repo_commits_sync(repo_id, branch.name).await {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }
}
