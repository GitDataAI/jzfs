use sea_orm::*;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::metadata::model::repo::{repo_branch, repo_commit};
use crate::metadata::service::repos_service::RepoService;

impl RepoService {
    pub async fn branch(&self, owner: String, repo: String) -> anyhow::Result<Vec<repo_branch::Model>>{
        let uid = self.owner_name_by_uid(owner,repo).await;
        if uid.is_err(){
            return Err(uid.err().unwrap())
        }
        let uid = uid?;
        let models = repo_branch::Entity::find()
            .filter(repo_branch::Column::RepoId.eq(uid))
            .all(&self.db)
            .await?;
        Ok(models)
    }
    pub async fn branch_by_name(&self, owner: String, repo: String, branch: String) -> anyhow::Result<repo_branch::Model>{
        let uid = self.owner_name_by_uid(owner,repo).await;
        if uid.is_err(){
            return Err(uid.err().unwrap())
        }
        let uid = uid?;
        let model = repo_branch::Entity::find()
            .filter(repo_branch::Column::RepoId.eq(uid))
            .filter(repo_branch::Column::Branch.eq(branch))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("branch not found"))
        }
        Ok(model.unwrap())
    }
    pub async fn create_branch(&self, owner: String, repo: String, branch: String, source: String) -> anyhow::Result<repo_branch::Model>{
        let txn = self.db.begin().await?;
        let uid = self.owner_name_by_uid(owner,repo).await;
        if uid.is_err(){
            return Err(uid.err().unwrap())
        }
        let uid = uid?;
        let source_model = repo_branch::Entity::find()
            .filter(repo_branch::Column::RepoId.eq(uid))
            .filter(repo_branch::Column::Branch.eq(source))
            .one(&txn)
            .await?;
        if source_model.is_none(){
            return Err(anyhow::anyhow!("source branch not found"))
        }
        let source_model = source_model.unwrap();
        let source_commit = repo_commit::Entity::find()
            .filter(repo_commit::Column::RepoId.eq(uid))
            .filter(repo_commit::Column::BranchId.eq(source_model.uid))
            .order_by_desc(repo_commit::Column::CreatedAt)
            .all(&txn)
            .await?;
        let mut model = source_model.into_active_model();
        let branch_uid = Uuid::new_v4();
        model.branch = Set(branch);
        model.uid = Set(branch_uid);
        model.clone().insert(&self.db).await?;
        
        for commit in source_commit{
            let mut commit = commit.into_active_model();
            commit.branch_id = Set(branch_uid);
            commit.uid = Set(Uuid::new_v4());
            commit.insert(&txn).await?;
        }
        
        Ok(model.try_into_model()?)
    }
}