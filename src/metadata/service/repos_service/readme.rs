use uuid::Uuid;
use crate::metadata::model::repo::repo;
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
use crate::git::repo::GitRepo;

impl RepoService{
    pub async fn readme(&self, repo_id: Uuid, branch: String) -> anyhow::Result<Vec<u8>>{
        let repo_model = repo::Entity::find()
            .filter(repo::Column::Uid.eq(repo_id))
            .one(&self.db)
            .await?;
        if repo_model.is_none(){
            return Err(anyhow::anyhow!("repo not found"));
        }
        let repo_model = repo_model.unwrap();
        let repo = GitRepo::from(repo_model);
        repo.readme(branch)
    }
}