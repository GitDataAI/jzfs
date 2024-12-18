use anyhow::anyhow;
use sea_orm::Set;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::metadata::model::users::users;
use crate::metadata::service::users_service::UserService;
use sea_orm::*;

impl UserService {
    pub async fn avatar(&self, uid: Uuid) -> anyhow::Result<String>{
        let avatar = users::Entity::find()
            .filter(
                users::Column::Uid.eq(uid)
            )
            .one(
                &self.db
            )
            .await;
        match avatar {
            Ok(result) => {
                match result {
                    Some(result) => {
                        Ok(result.avatar.unwrap_or(String::new()))
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
    pub async fn upload_avatar(&self, uid: Uuid, avatar: String) -> anyhow::Result<()>{
        let model = users::Entity::find()
            .filter(
                users::Column::Uid.eq(uid)
            )
            .one(
                &self.db
            )
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let mut model = model.unwrap().into_active_model();
        model.avatar = Set(Some(avatar));
        model.updated_at = Set(OffsetDateTime::now_utc());
        model.update(&self.db).await?;
        Ok(())
    }
    pub async fn delete_avatar(&self, uid: Uuid) -> anyhow::Result<()>{
        let model = users::Entity::find()
            .filter(
                users::Column::Uid.eq(uid)
            )
            .one(
                &self.db
            )
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let mut model = model.unwrap().into_active_model();
        model.avatar = Set(None);
        model.updated_at = Set(OffsetDateTime::now_utc());
        model.update(&self.db).await?;
        Ok(())
    }
}