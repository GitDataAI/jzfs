use crate::api::dto::repo_dto::RepoTree;
use crate::metadata::model::repo::repo;
use crate::metadata::service::repos_service::RepoService;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use sea_orm::*;
use uuid::Uuid;

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
        let (repo, owner) = (repo_model.name, repo_model.owner);
        let doc = {
            if commid_id.is_some(){
                doc!{
                    "repo":repo,
                    "owner": owner,
                    "branch":branch,
                    "hash":commid_id.unwrap()
                }
            }else{
                doc!{
                    "repo":repo,
                    "owner": owner,
                    "branch":branch,
                }
            }
        };
        let mut cursor = self.mongo.tree.find(doc).await?;
        let mut result = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc);
        }
        Err(anyhow::anyhow!("branch not found"))
    }
}