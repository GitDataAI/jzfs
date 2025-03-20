use crate::AppModule;
use jz_model::users;
use sea_orm::*;
use serde::Deserialize;
use sha256::Sha256Digest;

#[derive(Deserialize, Clone)]
pub struct Sigup {
    username: String,
    password: String,
    email: String,
    pub captcha: String,
}
#[derive(Deserialize)]
pub struct Sigin {
    username: String,
    password: String,
    pub captcha: String,
}

#[derive(Deserialize)]
pub struct SigupCheck {
    username: String,
    email: String
}

impl AppModule {
    pub async fn user_sigin(&self, param: Sigin) -> anyhow::Result<users::Model> {
        let username = param.username;
        let password = param.password;
        let user = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(username.clone()))
                    .add(users::Column::Email.eq(username)),
            )
            .filter(users::Column::Password.eq(password.digest()))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("user not found"))?;
        if !user.allow_use {
            return Err(anyhow::anyhow!("user not allow use"));
        }
        Ok(user)
    }
    pub async fn user_signup(&self, param: Sigup) -> anyhow::Result<()> {
        let username = param.username;
        let password = param.password;
        let email = param.email;
        let user = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(username.clone()))
                    .add(users::Column::Email.eq(username.clone())),
            )
            .one(&self.read)
            .await?;
        if user.is_some() {
            return Err(anyhow::anyhow!("username or email already exist"));
        }
        let user = users::ActiveModel::new(username, password.digest(), email);
        user.insert(&self.write).await?;
        Ok(())
    }
    pub async fn users_check(&self, check: SigupCheck) -> anyhow::Result<()> {
        let username = check.username;
        let email = check.email;
        let user = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(username.clone()))
            )
            .one(&self.read)
            .await?;
        if user.is_some() {
            return Err(anyhow::anyhow!("username already exist"));
        }
        let orgs = self.org_by_name(username).await;
        if orgs.is_ok() {
            return Err(anyhow::anyhow!("username already exist"));
        }
        if !email.is_empty() {
            let user = users::Entity::find()
                .filter(users::Column::Email.eq(email))
                .one(&self.read)
                .await?;
            if user.is_some() {
                return Err(anyhow::anyhow!("email already exist"));
            }
        }
        Ok(())
    }
}
