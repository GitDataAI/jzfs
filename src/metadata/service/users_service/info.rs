use sea_orm::*;
use uuid::Uuid;
use crate::api::dto::user_dto::UserOv;
use crate::metadata::model::users::{users, users_data, UsersData};
use crate::metadata::service::users_service::UserService;

impl UserService{
    pub async fn info(&self, uid: Uuid) -> anyhow::Result<UserOv>{
        let v1 = users::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        match v1 {
            Some(x) => {
                Ok(UserOv::from(x))
            },
            None => Err(anyhow::anyhow!("[Error] User Not Found"))
        }
    }
    pub async fn user_data(&self, uid: Uuid) -> anyhow::Result<UsersData>{
        let v1 = users_data::Entity::find()
            .filter(users_data::Column::UserId.eq(uid))
            .one(&self.db)
            .await?;
        match v1 {
            Some(x) => Ok(x),
            None => Err(anyhow::anyhow!("[Error] User Data Not Found"))
        }
    }
    pub async fn _user_private(&self, uid: Uuid) -> anyhow::Result<users::Model>{
        let v1 = users::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        match v1 {
            Some(x) => Ok(x),
            None => Err(anyhow::anyhow!("[Error] User Not Found"))
        }
    }
}