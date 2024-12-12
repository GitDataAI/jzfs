use crate::metadata::model::repos::repo;
use crate::metadata::transaction::repos::RepoTransaction;
use crate::store::dto::ObjectFile;
use crate::store::host::GitLocal;
use sea_orm::*;
use uuid::Uuid;

impl RepoTransaction {
    pub async fn object_tree(&self, repo_id: Uuid, branch: String) -> anyhow::Result<Vec<ObjectFile>>{
        let repo = repo::Entity::find_by_id(repo_id)
            .one(&self.db)
            .await?;
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo = repo.unwrap();
        let store = GitLocal::init(repo.uid.to_string());
        let tree = store.object_tree(branch);
        if tree.is_err(){
            return Err(anyhow::anyhow!("tree error:{}",tree.err().unwrap()))
        }
        let tree = tree?;
        Ok(tree)
    }
}