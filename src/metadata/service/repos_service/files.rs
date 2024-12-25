use uuid::Uuid;
use crate::git::repo::GitRepo;
use crate::metadata::model::repo::repo;
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
impl RepoService {
    pub async fn get_file(&self, repo_id: Uuid, branch: String, commit_id: Option<String>,file_path: String) -> anyhow::Result<Vec<u8>>{
        let repo_model = repo::Entity::find()
            .filter(repo::Column::Uid.eq(repo_id))
            .one(&self.db)
            .await?;
        if repo_model.is_none() {
            return Err(anyhow::anyhow!("repo not found"));
        }
        let repo_model = repo_model.unwrap();
        let repo = GitRepo::from(repo_model);
        repo.files(branch,commit_id,file_path)
    }
    pub async fn add_file(&self, repo_id: Uuid, branch: String,file_name: String, path: String, content: Vec<u8>, msg: String, username: String, email: String) -> anyhow::Result<()>{
        let repo_model = repo::Entity::find()
            .filter(repo::Column::Uid.eq(repo_id))
            .one(&self.db)
            .await?;
        if repo_model.is_none() {
            return Err(anyhow::anyhow!("repo not found"));
        }
        let repo_model = repo_model.unwrap();
        let repo = GitRepo::from(repo_model);
        repo.add_file(branch,path,file_name,content,msg,username,email)?;
        Ok(())
    }
}