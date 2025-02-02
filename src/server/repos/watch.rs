use crate::error::{JZError, JZResult};
use crate::models::users::watchs;
use crate::server::MetaData;
use sea_orm::*;
impl MetaData {
    pub async fn repo_watch_list(
        &self,
        owner: String,
        name: String,
    ) -> JZResult<Vec<watchs::Model>> {
        let result = match self.repo_info(owner, name).await {
            Ok(model) => model,
            Err(e) => return Err(e),
        };
        let result = match watchs::Entity::find()
            .filter(watchs::Column::RepoId.eq(result.uid))
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
