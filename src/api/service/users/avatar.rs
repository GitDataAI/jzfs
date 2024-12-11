use anyhow::anyhow;
use sea_orm::*;
use uuid::Uuid;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users_avatar;

impl UserService {
    pub async fn avatar(&self, uid: Uuid) -> anyhow::Result<Vec<u8>>{
        let avatar = users_avatar::Entity::find()
            .filter(
                users_avatar::Column::UserId.eq(uid)
            )
            .one(
                &self.db
            )
            .await;
        match avatar {
            Ok(result) => {
                match result {
                    Some(result) => {
                        Ok(result.avatar)
                    },
                    None => {
                        Err(anyhow!(
                            "[Error] Avatar Not Found".to_string()
                        ))
                    }
                }
            },
            Err(e) => {
                Err(anyhow::Error::new(e))
            }
       }
    }
    pub async fn upload_avatar(&self, uid: Uuid, avatar: Vec<u8>) -> anyhow::Result<()>{
        let model = users_avatar::Entity::find()
            .filter(
                users_avatar::Column::UserId.eq(uid)
            )
            .one(
                &self.db
            )
            .await?;
        if model.is_some(){
            let model = users_avatar::ActiveModel{
                uid: Set(model.unwrap().uid),
                user_id: Set(uid),
                avatar: Set(avatar),
                upload_at: Set(time::OffsetDateTime::now_utc()),
                ..Default::default()
            };
            model.update(&self.db).await?;
        }else{
            let model = users_avatar::ActiveModel{
                uid: Default::default(),
                user_id: Set(uid),
                avatar: Set(avatar),
                upload_at: Set(time::OffsetDateTime::now_utc()),
            };
            model.insert(&self.db).await?;
        }
        Ok(())
    }
    pub async fn delete_avatar(&self, uid: Uuid) -> anyhow::Result<()>{
        let model = users_avatar::Entity::find()
            .filter(
                users_avatar::Column::UserId.eq(uid)
            )
            .one(
                &self.db
            )
            .await?;
        if model.is_some(){
            model.unwrap().delete(&self.db).await?;
        }
        Ok(())
    }
}