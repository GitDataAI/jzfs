use crate::error::{JZError, JZResult};
use crate::models::users::star;
use crate::models::users::star::Model;
use crate::server::MetaData;
use sea_orm::*;
impl MetaData {
    pub async fn repo_star_list(&self, owner: String, repo: String) -> JZResult<Vec<Model>> {
        let result = match self.repo_info(owner, repo).await {
            Ok(model) => model,
            Err(e) => return Err(e),
        };
        let result = match star::Entity::find()
            .filter(star::Column::RepoId.eq(result.uid))
            .all(&self.database)
            .await
        {
            Ok(model) => model,
            Err(e) => {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[039] Open Repo Failed: {}",
                    e.to_string()
                )));
            }
        };
        Ok(result)
    }
}
