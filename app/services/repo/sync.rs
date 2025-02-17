use crate::app::services::AppState;
use crate::model::repository::tree;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use std::io;
use uuid::Uuid;

impl AppState {
    pub async fn repo_sync(&self, repo_uid: Uuid) -> io::Result<()> {
        let repo = self.repo_get_by_uid(repo_uid).await?;
        let path = format!("{}/{}/{}", crate::app::http::GIT_ROOT, repo.node_uid, repo.uid);
        let blob = crate::blob::GitBlob::new(path.into())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;     
        let branch = blob.branch()?;
        for branch in branch {
            if tree::Entity::find()
                .filter(tree::Column::RepoUid.eq(repo_uid))
                .filter(tree::Column::Branch.eq(branch.name.clone()))
                .filter(tree::Column::Head.eq(branch.head.clone()))
                .one(&self.read)
                .await
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get tree"))?
                .is_some()
            {
                continue
            }
            let tree = match blob.tree(branch.name.clone()) {
                Ok(tree) => tree,
                Err(_) => {
                    continue
                },
            };
            let _ = tree::ActiveModel {
                uid: Set(Uuid::new_v4()),
                repo_uid: Set(repo_uid),
                head: Set(branch.head),
                content: Set(serde_json::to_string(&tree)?),
                branch: Set(branch.name),
            }
                .insert(&self.write).await;
        }
        Ok(())
    }
}