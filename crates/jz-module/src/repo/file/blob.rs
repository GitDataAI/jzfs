use serde::{Deserialize, Serialize};
use crate::AppModule;

#[derive(Deserialize,Serialize)]
pub struct BlobQuery {
    sha: String,
    path: String,
}

impl AppModule {
    pub async fn repo_blob(&self, owner: String, repo: String, param: BlobQuery) -> anyhow::Result<Vec<u8>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let mut git = repo.git()?;
        git.blob(param.sha, param.path)
    }
}