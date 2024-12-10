use deadpool_redis::redis::AsyncCommands;
use sea_orm::*;
use sha256::Sha256Digest;
use uuid::Uuid;
use crate::api::dto::users::{UserResetPasswd, UserResetPassword};
use crate::api::service::users::UserService;
use crate::metadata::model::users::{users, users_email};
use crate::server::redis::REDIS;

impl UserService {
    pub async fn reset_by_token(&self, dto: UserResetPassword) -> anyhow::Result<()>{
        let token = dto.token;
        let passwd = dto.password;

        let mut redis = REDIS.get().unwrap().write().unwrap();
        let email = match redis.get::<String,String>(token.clone()).await{
            Ok(email) => email,
            Err(_) => {
                return Err(anyhow::anyhow!("Token Expired"))
            }
        };
        if email != dto.email{
            return Err(anyhow::anyhow!("Token Expired"))
        }
        redis.del::<String, String>(token).await.ok();
        let model = users_email::Entity::find()
            .filter(
                users_email::Column::Email.eq(dto.email)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        let model = users::Entity::find_by_id(model.user_id)
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        let mut model = model.into_active_model();
        model.passwd = Set(passwd.digest());
        model.update(&self.db).await?;
        Ok(())
    }
    pub async fn reset(&self, dto: UserResetPasswd, uid: Uuid) ->anyhow::Result<()>{
        let model = users::Entity::find()
            .filter(
                users::Column::Uid.eq(uid)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("User Not Found"))
        }
        let model = model.unwrap();
        if model.passwd != dto.old_password.digest(){
            return Err(anyhow::anyhow!("Old Password Err"))
        }
        let mut model = model.into_active_model();
        model.passwd = Set(dto.new_password.digest());
        model.update(&self.db).await?;
        Ok(())
    }
}