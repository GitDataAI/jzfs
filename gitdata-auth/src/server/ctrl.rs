use lib_entity::{ActiveModelTrait, ColumnTrait};
use serde::{Deserialize, Serialize};
use lib_entity::{EntityTrait, QueryFilter};
use lib_entity::prelude::Uuid;
use lib_entity::session::UsersSessionModel;
use lib_entity::users::{email, users};
use lib_entity::users::users::UsersOption;
use crate::server::AppAuthState;


#[derive(Clone,Deserialize,Serialize)]
pub struct UsersApply {
    username: String,
    password: String,
    email: String,
}



impl AppAuthState {
    pub async fn check_have_username(&self, username: String) -> anyhow::Result<bool> {
        if users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.read)
            .await?
            .is_some()
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub async fn check_have_email(&self, email: String) -> anyhow::Result<bool> {
        if users::Entity::find()
            .filter(users::Column::MainEmail.eq(email.clone()))
            .one(&self.read)
            .await?
            .is_some()
        {
            Ok(true)
        } else {
            if email::Entity::find()
                .filter(email::Column::Content.eq(email))
                .one(&self.read)
                .await?
                .is_some()
            {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
    pub async fn auth_apply(&self, param: UsersApply) -> anyhow::Result<UsersSessionModel> {
        if self.check_have_username(param.username.clone()).await? {
            return Err(anyhow::anyhow!("用户名已被注册"));
        }
        if self.check_have_email(param.email.clone()).await? {
            return Err(anyhow::anyhow!("邮箱已被注册"));
        }
        match users::ActiveModel::new_users(
            param.username,
            param.password,
            param.email,
        )
            .insert(&self.write)
            .await{
                Ok(user) => {
                    Ok(UsersSessionModel::from(user))
                },
                Err(e) => {
                    Err(anyhow::anyhow!("注册失败：{}",e))
                }
        }
    }
    pub async fn now_user(&self, users_uid: Uuid) -> anyhow::Result<UsersOption> {
        match users::Entity::find_by_id(users_uid)
            .one(&self.read)
            .await?{
                Some(user) => {
                    Ok(UsersOption::from(user))
                },
                None => {
                    Err(anyhow::anyhow!("用户不存在"))
                }
        }
    }
}

