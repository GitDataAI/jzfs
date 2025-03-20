use crate::AppModule;
use jz_git::branches::GitBranches;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct RepoCreateBranch {
    name: String,
    from: String,
}

impl AppModule {
    pub async fn repo_list_branch(
        &self,
        _: Option<Uuid>,
        owner: String,
        repo: String,
    ) -> anyhow::Result<Vec<GitBranches>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let mut git = repo.git()?;
        let branches = git.list_branches()?;
        Ok(branches)
    }
    pub async fn repo_create_branch(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
        param: RepoCreateBranch,
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
        git.create_branch(param.name, Option::from(param.from))?;
        Ok(())
    }
    pub async fn repo_delete_branch(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
        name: String,
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
        git.delete_branch(name)?;
        Ok(())
    }
    pub async fn repo_rename_branch(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
        name: String,
        new_name: String,
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
        git.rename_branch(name, new_name)?;
        Ok(())
    }
    pub async fn repo_checkout_head(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
        name: String,
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
        git.checkout_head(name)?;
        Ok(())
    }
}
