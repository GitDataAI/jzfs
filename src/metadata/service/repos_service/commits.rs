use crate::metadata::model::repo::{repo_branch, repo_commit};
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
impl RepoService {
    pub async fn commits(&self, owner: String, repo: String, branch: String) -> anyhow::Result<Vec<repo_commit::Model>>{
        let uid = self.owner_name_by_uid(owner,repo).await;
        if uid.is_err(){
            return Err(uid.err().unwrap())
        }
        let repo_uid = uid?;

        let model = repo_branch::Entity::find()
            .filter(repo_branch::Column::RepoId.eq(repo_uid))
            .filter(repo_branch::Column::Branch.eq(branch))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("branch not found"))
        }
        let branch_uid = model.unwrap().uid;
        let models = repo_commit::Entity::find()
            .filter(repo_commit::Column::BranchId.eq(branch_uid))
            .all(&self.db)
            .await?;
        Ok(models)
    }
    pub async fn commit(&self, owner: String, repo: String, branch: String, commit_id: String) -> anyhow::Result<repo_commit::Model>{
        let uid = self.owner_name_by_uid(owner,repo).await;
        if uid.is_err(){
            return Err(uid.err().unwrap())
        }
        let repo_uid = uid?;
        let model = repo_branch::Entity::find()
            .filter(repo_branch::Column::RepoId.eq(repo_uid))
            .filter(repo_branch::Column::Branch.eq(branch))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("branch not found"))
        }
        let branch_uid = model.unwrap().uid;
        let model = repo_commit::Entity::find()
            .filter(repo_commit::Column::BranchId.eq(branch_uid))
            .filter(repo_commit::Column::CommitId.eq(commit_id))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("commit not found"))
        }
        Ok(model.unwrap())
    }
}