use uuid::Uuid;
use crate::metadata::model::repo::repo;
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
use crate::api::dto::repo_dto::RepoTree;
use crate::git::branchs::GitBranch;
use crate::git::repo::GitRepo;

impl RepoService {
    pub async fn tree(&self, repo_id: Uuid, branch: String, commid_id: Option<String>) -> anyhow::Result<RepoTree>{
        let repo_model = repo::Entity::find()
            .filter(repo::Column::Uid.eq(repo_id))
            .one(&self.db)
            .await?;
        if repo_model.is_none(){
            return Err(anyhow::anyhow!("repo not found"));
        }
        let repo_model = repo_model.unwrap();
        let repo = GitRepo::from(repo_model);
        let branchse = GitBranch::new(repo.repo);
        let branchs = branchse.branchs();
        if branchs.is_err(){
            return Err(anyhow::anyhow!("branchs error"));
        }
        let branchs = branchs?;
        for item in branchs{
            if item.name()?.unwrap() == branch{
                let tree = branchse.tree(
                    item,
                    commid_id
                );
                if tree.is_err(){
                    return Err(anyhow::anyhow!("tree error"));
                }
                let tree = tree?;
                return Ok(tree)
            }
        }
        Err(anyhow::anyhow!("branch not found"))
    }
}