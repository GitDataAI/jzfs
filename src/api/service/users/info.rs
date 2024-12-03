use sea_orm::EntityTrait;
use crate::api::ov::users::UserOv;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users;

impl UserService {
    pub async fn info(&self, uid: uuid::Uuid) -> anyhow::Result<UserOv>{
        let model = users::Entity::find_by_id(uid)
            .one(&self.db)
            .await?;
        match model {
            Some(x) => {
                Ok(UserOv::from(x))
            },
            None => Err(anyhow::anyhow!("[Error] User Not Found"))
        }
    }
}