use crate::AppModule;
use jz_git::commits::{CreateGitCommit, GitCommit};
use uuid::Uuid;

impl AppModule {
    pub async fn repo_list_commit(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
        branch: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> anyhow::Result<Vec<GitCommit>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let user = self.user_info_by_id(ops_uid).await?;
        if repo.is_private {
            if user.uid != repo.owner_uid {
                return Err(anyhow::anyhow!("permission denied"));
            }
            // TODO GROUP
        }
        let mut git = repo.git()?;
        let mut commits = git.list_commit(branch)?;
        if offset.is_some() && limit.is_some() {
            commits.drain(0..offset.unwrap() as usize * limit.unwrap() as usize);
            commits.truncate(limit.unwrap_or(100) as usize);
        }
        Ok(commits)
    }
    pub async fn repo_create_commit(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
        param: CreateGitCommit,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let user = self.user_info_by_id(ops_uid).await?;
        if repo.is_private {
            if user.uid != repo.owner_uid {
                return Err(anyhow::anyhow!("permission denied"));
            }
            // TODO GROUP
        }
        let mut git = repo.git()?;
        git.create_commit(param)?;
        Ok(())
    }
}
