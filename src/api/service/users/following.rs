use sea_orm::*;
use crate::api::ov::users::UserFollowerOv;
use crate::api::service::users::UserService;
use crate::metadata::model::users::{users, users_other};

impl UserService {
    pub async fn followers(&self, uid: uuid::Uuid) -> anyhow::Result<Vec<UserFollowerOv>> {
        let model = users_other::Entity::find()
            .filter(
                users_other::Column::UserId.eq(uid)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        let models = users::Entity::find()
            .filter(
                users::Column::Uid.is_in(model.follow)
            )
            .all(&self.db)
            .await?
            .iter()
            .map(|x|{
                UserFollowerOv{
                    uid: x.uid.clone(),
                    name: x.name.clone(),
                    username: x.username.clone(),
                    avatar: None,
                    description: x.description.clone(),
                }
            })
            .collect::<Vec<_>>();
        Ok(models)
    }
}