use anyhow::anyhow;
use serde::Deserialize;
use sha256::Sha256Digest;
use jz_model_sqlx::users::UsersModel;
use crate::app::AppService;

#[derive(Deserialize, Clone)]
pub struct Signup {
    pub username: String,
    pub password: String,
    pub email: String,
    pub captcha: String,
}
#[derive(Deserialize)]
pub struct Signin {
    pub username: String,
    pub password: String,
    pub captcha: String,
}

#[derive(Deserialize)]
pub struct SignupCheck {
    pub username: Option<String>,
    pub email: Option<String>
}


impl AppService {
    pub async fn user_auth_signin(&self, param: Signin) -> anyhow::Result<UsersModel> {
        let username = param.username;
        if username.is_empty() {
            return Err(anyhow::anyhow!("username is empty"));
        }
        let password = param.password;
        if password.is_empty() {
            return Err(anyhow::anyhow!("password is empty"));
        }
        let password = password.digest();
        let mapper = self.user_mapper();
        let user = mapper.query().get_user_by_username(username).await?;
        if user.password != password {
            return Err(anyhow::anyhow!("password is wrong"));
        }
        if user.allow_use {
            return Err(anyhow::anyhow!("user is not allow use"));
        }
        if user.deleted_at.is_some() {
            return Err(anyhow::anyhow!("user is deleted"));
        }
        if user.last_login_at.is_none() {
            mapper.update(user.clone()).update_last_login_at().await?;
        }
        Ok(user)
    }
    pub async fn user_auth_signup(&self, param: Signup) -> anyhow::Result<UsersModel> {
        let username = param.username;
        let password = param.password.digest();
        let email = param.email;
        if username.is_empty() {
            return Err(anyhow::anyhow!("username is empty"));
        }
        if password.is_empty() {
            return Err(anyhow::anyhow!("password is empty"));
        }
        if email.is_empty() {
            return Err(anyhow::anyhow!("email is empty"));
        }
        if self.user_mapper().query().get_user_by_username(username.clone()).await.is_ok() {
            return Err(anyhow::anyhow!("username is exist"));
        }
        if self.user_mapper().query().get_user_by_email(email.clone()).await.is_ok() {
            return Err(anyhow::anyhow!("email is exist"));
        }
        let builder = UsersModel::builder()
            .username(username)
            .password(password)
            .email(email)
            .build(&self.write).await;
        if let Err(err) = builder {
            return Err(anyhow::anyhow!("{}", err));
        }
        let Ok(user) = builder else {
            return Err(anyhow!("builder error"))
        };
        Ok(user)
    }
    pub async fn users_auth_check(&self, check: SignupCheck) -> anyhow::Result<()> {
        let mapper = self.user_mapper();
        let query = mapper.query();
        if let Some(username) = check.username {
            if query.get_user_by_username(username).await.is_ok() {
                return Err(anyhow::anyhow!("username is exist"));
            }
        }
        if let Some(email) = check.email {
            if query.get_user_by_email(email).await.is_ok() {
                return Err(anyhow::anyhow!("email is exist"));
            }
        }
        Ok(())
    }
    pub async fn user_auth_logout(&self, _user: UsersModel) -> anyhow::Result<()> {
        todo!()
    }
    pub async fn user_auth_login_with_github(&self) -> anyhow::Result<()> {
        todo!()
    }
    pub async fn user_auth_login_with_wechat(&self) -> anyhow::Result<()> {
        todo!()
    }
    pub async fn user_auth_login_with_google(&self) -> anyhow::Result<()> {
        todo!()
    }
}