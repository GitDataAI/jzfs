use std::io;
use sea_orm::{EntityTrait, QueryOrder, QuerySelect};
use crate::app::services::AppState;
use crate::model::repository::repository;

impl AppState {
    pub async fn recommend_repo_by_new(&self, size: u64) -> io::Result<Vec<repository::Model>> {
        let models = repository::Entity::find()
            .order_by_asc(repository::Column::CreatedAt)
            .limit(size)
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to fetch repository"))?;
        Ok(models)
    }
    pub async fn recommend_repo_by_update(&self, size: u64) -> io::Result<Vec<repository::Model>> {
        let models = repository::Entity::find()
            .order_by_desc(repository::Column::UpdatedAt)
            .limit(size)
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to fetch repository"))?;
        Ok(models)
    }
    pub async fn recommend_repo_by_star(&self, size: u64) -> io::Result<Vec<repository::Model>> {
        let models = repository::Entity::find()
            .order_by_desc(repository::Column::NumsStar)
            .limit(size)
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to fetch repository"))?;
        Ok(models)
    }
    pub async fn recommend_repo_by_fork(&self, size: u64) -> io::Result<Vec<repository::Model>> {
        let models = repository::Entity::find()
            .order_by_desc(repository::Column::NumsFork)
            .limit(size)
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to fetch repository"))?;
        Ok(models)
    }
    pub async fn recommend_repo_by_watch(&self, size: u64) -> io::Result<Vec<repository::Model>> {
        let models = repository::Entity::find()
            .order_by_desc(repository::Column::NumsWatch)
            .limit(size)
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to fetch repository"))?;
        Ok(models)
    }
}