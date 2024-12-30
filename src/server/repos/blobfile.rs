use crate::error::{JZError, JZResult};
use crate::server::MetaData;

impl MetaData {
    pub async fn repo_blob(
        &self,
        owner: String,
        name: String,
        branchs: String,
        sha: Option<String>,
        path: String,
    ) -> JZResult<Vec<u8>> {
        let result = match self.repo_info(owner, name).await {
            Ok(model) => model,
            Err(e) => return Err(e),
        };
        let local = match self.git.open_repo(result.uid.to_string()) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[039] Open Repo Failed: {}",
                    e.to_string()
                )))
            }
        };
        Ok(local.blob(branchs, sha, path)?)
    }
}
