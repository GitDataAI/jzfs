use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jz_model::tags;
use crate::AppModule;

#[derive(Deserialize,Serialize)]
pub struct TagAdd {
    pub color: String,
    pub description: Option<String>,
}

impl AppModule {
    pub async fn tag_add(&self, owner: String, repo: String, param: TagAdd, opsuid: Uuid) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let access = self.repo_access(opsuid).await?;
        if let None = access.iter().find(|x|x.repo_uid == repo.uid) {
            return Err(anyhow::anyhow!("permission denied"));
        }
        let tag = tags::ActiveModel::new(param.color, param.description, repo.uid);
        tag.insert(&self.write).await?;
        Ok(())
    }
    pub async fn tag_del(&self, owner: String, repo: String, tag_id: Uuid, opsuid: Uuid) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let access = self.repo_access(opsuid).await?;
        if let None = access.iter().find(|x|x.repo_uid == repo.uid) {
            return Err(anyhow::anyhow!("permission denied"));
        }
        let _ = tags::Entity::delete_many()
            .filter(tags::Column::RepoUid.eq(repo.uid))
            .filter(tags::Column::Uid.eq(tag_id))
            .exec(&self.write)
            .await?;
        Ok(())
    }
    pub async fn tag_list(&self, owner: String, repo: String) -> anyhow::Result<Vec<tags::Model>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let tags = tags::Entity::find()
            .filter(tags::Column::RepoUid.eq(repo.uid))
            .all(&self.read)
            .await?;
        Ok(tags)
    }
}