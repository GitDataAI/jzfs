use sea_orm::*;
use uuid::Uuid;
use crate::metadata::model::repos::{repo, repo_branch};
use crate::metadata::transaction::repos::RepoTransaction;
use crate::store::dto::ObjectFile;
use crate::store::host::GitLocal;

impl RepoTransaction {
    pub async fn object_tree(&self, repo_id: Uuid, branch_id: Uuid) -> anyhow::Result<Vec<ObjectFile>>{
        let repo = repo::Entity::find_by_id(repo_id)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let branch = repo_branch::Entity::find_by_id(branch_id)
            .one(&self.db)
            .await?;
        if branch.is_none(){
            return Err(anyhow::anyhow!("branch not found"))
        }
        let repo = repo.unwrap();
        let branch = branch.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        let tree = store.object_tree(branch.branch);
        if tree.is_err(){
            return Err(anyhow::anyhow!("tree error:{}",tree.err().unwrap()))
        }
        let tree = tree?;
        Ok(tree)
    }
}