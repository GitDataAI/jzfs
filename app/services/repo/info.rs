use std::io;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::app::services::AppState;
use crate::model::repository::repository;
use crate::model::users::users;

impl AppState {
    pub async fn repo_info(
        &self,
        owner: String, 
        repo: String
    )
    -> io::Result<repository::Model>
    {
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(owner))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "user not found"))?;
        repository::Entity::find()
            .filter(repository::Column::OwnerId.eq(user.uid))
            .filter(repository::Column::Name.eq(repo))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))
    }
    pub async fn repo_get_by_uid(&self, uid: Uuid) -> io::Result<repository::Model> {
        repository::Entity::find_by_id(uid)
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))
    }
}