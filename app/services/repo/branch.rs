use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use std::io;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use crate::app::services::AppState;
use crate::model::repository::branches;

#[derive(Deserialize,Serialize,Clone)]
pub struct BranchCreateParma {
    pub name: String,
    pub head: String,
}


impl AppState {
    pub async fn branch_list(&self, owner: String, repo: String) -> io::Result<Vec<branches::Model>> {
        let repo = self.repo_info(owner, repo)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        branches::Entity::find()
            .filter(branches::Column::RepoUid.eq(repo.uid))
            .all(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))
    }
    
}