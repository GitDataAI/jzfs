use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::api::dto::users::{UsersLoginEmail, UsersLoginUsername};
use crate::api::middleware::session::SessionModel;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users;

impl UserService {
    pub async fn login_by_email(&self, dto: UsersLoginEmail) -> anyhow::Result<SessionModel>{
        let email = dto.email;
        let passwd = dto.passwd;
        let model = users::Entity::find()
            .filter(
                users::Column::Email.eq(email)
            )
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("Email or Passwd Err"));
        }
        let model = model.unwrap();
        if model.passwd != sha256::digest(passwd){
            return Err(anyhow::anyhow!("Email or Passwd Err"));
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
            return Err(anyhow::anyhow!("Username or Passwd Err"));
        }
        let model = model.unwrap();
        if model.passwd != sha256::digest(passwd){
            return Err(anyhow::anyhow!("Username or Passwd Err"));
        }
        Ok(SessionModel::from(model))
    }
}