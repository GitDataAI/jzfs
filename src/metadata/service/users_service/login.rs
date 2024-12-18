use crate::metadata::service::users_service::UserService;
use sea_orm::*;
use crate::api::dto::user_dto::{UsersLoginEmail, UsersLoginUsername};
use crate::api::middleware::session::model::SessionModel;
use crate::metadata::model::users::{users, users_email};

impl UserService {
    pub async fn login_by_email(&self, dto: UsersLoginEmail) -> anyhow::Result<SessionModel>{
        let email = dto.email;
        let passwd = dto.passwd;
        let model = users_email::Entity::find()
            .filter(
                users_email::Column::Email.eq(email)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("Email or Passwd Err 0"));
        }
        let model = model.unwrap();
        let model = users::Entity::find_by_id(model.user_id)
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("Email or Passwd Err 1"));
        }
        let model = model.unwrap();
        if model.passwd != sha256::digest(passwd){
            return Err(anyhow::anyhow!("Email or Passwd Err 2"));
        }
        Ok(SessionModel::from(model))
    }
    pub async fn login_by_username(&self, dto: UsersLoginUsername) -> anyhow::Result<SessionModel>{
        let username = dto.username;
        let passwd = dto.passwd;
        let model = users::Entity::find()
            .filter(
                users::Column::Username.eq(username)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("Username or Passwd Err 0"));
        }
        let model = model.unwrap();
        if model.passwd != sha256::digest(passwd){
            return Err(anyhow::anyhow!("Username or Passwd Err 1"));
        }
        Ok(SessionModel::from(model))
    }
}