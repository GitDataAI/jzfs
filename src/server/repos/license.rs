use crate::error::{JZError, JZResult};
use crate::models::repos::{license, repos};
use crate::server::MetaData;
use sea_orm::*;
impl MetaData {
    pub async fn repo_license(&self, owner: String, name: String) -> JZResult<Vec<license::Model>> {
        let result = repos::Entity::find()
            .filter(repos::Column::Owner.eq(owner))
            .filter(repos::Column::Name.eq(name))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[036] Repo Not Found")));
        }
        let result = license::Entity::find()
            .filter(license::Column::RepoId.eq(result.unwrap().uid))
            .all(&self.database)
            .await?;
        Ok(result)
    }
}
