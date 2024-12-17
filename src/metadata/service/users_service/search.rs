use crate::api::dto::user_dto::UserOv;
use crate::metadata::model::users::users;
use crate::metadata::service::users_service::UserService;
use sea_orm::*;

impl UserService {
    pub async fn search(&self, keywords: String, page: u64, size: u64) -> anyhow::Result<Vec<UserOv>>{
        let models = users::Entity::find()
            .filter(users::Column::Username.contains(keywords.clone()))
            .filter(users::Column::Name.contains(keywords.clone()))
            .filter(users::Column::Description.contains(keywords))
            .offset(page * size)
            .limit(size)
            .all(&self.db)
            .await?;
        Ok(models.into_iter().map(|model| UserOv::from(model)).collect())
    }
}