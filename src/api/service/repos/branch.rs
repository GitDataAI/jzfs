use sea_orm::*;
use crate::api::service::repos::RepoService;
use crate::metadata::model::repos::{repo, repo_branch};
use crate::store::dto::ConflictsFiles;
use crate::store::host::GitLocal;

impl RepoService {
    pub async fn branch(&self, uid: uuid::Uuid) -> anyhow::Result<Vec<String>>{
        let models = repo_branch::Entity::find()
            .filter(
                repo_branch::Column::RepoId.eq(uid)
            )
            .all(&self.db)
            .await?;
        let store = GitLocal::init(uid.to_string());
        let db_branchs = models.into_iter().map(|x| x.branch).collect::<Vec<_>>();
        let branch = store.branchs()?;
        if db_branchs != branch{
            self.transaction.async_repo_branch_commit(uid).await?;
        }
        Ok(branch)
    }
    pub async fn new_branch(&self, uid: uuid::Uuid, from: String, branch: String) -> anyhow::Result<()>{
        let repo = repo::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo = repo.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        store.new_branches(from, branch)?;
        self.transaction.async_repo_branch_commit(uid).await?;
       Ok(())
    }
    pub async fn del_branch(&self, uid: uuid::Uuid, branch: String) -> anyhow::Result<()>{
        let repo = repo::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo = repo.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        store.del_branchs(branch)?;
        self.transaction.async_repo_branch_commit(uid).await?;
        Ok(())
    }
    pub async fn check_conflicts(&self, uid: uuid::Uuid, branch: String, target: String) -> anyhow::Result<Vec<ConflictsFiles>>{
        let repo = repo::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo = repo.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        let conflicts = store.check_conflicts(branch, target)?;
        Ok(conflicts)
    }
    pub async fn merge_branch(&self, uid: uuid::Uuid, branch: String, target: String) -> anyhow::Result<()>{
        let repo = repo::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo = repo.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        store.merge(branch, target)?;
        self.transaction.async_repo_branch_commit(uid).await?;
        Ok(())
    }
    pub async fn check_merge_conflicts(&self, uid: uuid::Uuid, branch: String, target: String) -> anyhow::Result<Vec<ConflictsFiles>>{
        let repo = repo::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo = repo.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        let conflicts = store.check_conflicts(branch, target)?;
        Ok(conflicts)
    }
    pub async fn rename_branch(&self, uid: uuid::Uuid, branch: String, new_branch: String) -> anyhow::Result<()>{
        let repo = repo::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo = repo.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        store.rename(branch, new_branch)?;
        self.transaction.async_repo_branch_commit(uid).await?;
        Ok(())
    }
    pub async fn protect_branch(&self, uid: uuid::Uuid, branch: String, protect: bool) -> anyhow::Result<()>{
        let repo = repo_branch::Entity::find()
            .filter(repo_branch::Column::RepoId.eq(uid))
            .filter(repo_branch::Column::Branch.eq(branch))
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let mut repo = repo.unwrap().into_active_model();
        repo.protect = Set(protect);
        repo.update(&self.db).await?;
        Ok(())
    }
}