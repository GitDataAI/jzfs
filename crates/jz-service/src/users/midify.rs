use sha256::Sha256Digest;
use uuid::Uuid;
use crate::app::AppService;

pub struct UsersModifyFromParam {
    pub description: Option<String>,
    pub website: Vec<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub location: Option<String>,
}

pub struct UsersModifyPassword {
    pub old: String,
    pub new: String,
}

impl AppService {
    pub async fn users_modify_from(&self, user_uid: Uuid, param: UsersModifyFromParam) -> anyhow::Result<()> {
        let mapper = self.user_mapper();
        let model = mapper.query().get_user_by_uid(user_uid).await?;
        let update = mapper.update(model);
        if let Some(description) = param.description {
            update.update_description(description).await?;
        }
        if let Some(timezone) = param.timezone {
            update.update_timezone(timezone).await?;
        }
        if let Some(language) = param.language {
            update.update_language(language).await?;
        }
        if let Some(location) = param.location {
            update.update_location(location).await?;
        }
        if !param.website.is_empty() {
            update.update_website(param.website).await?;
        }
        Ok(())
    }
    pub async fn users_modify_avatar(&self, user_uid: Uuid, avatar: String) -> anyhow::Result<()> {
        let mapper = self.user_mapper();
        let model = mapper.query().get_user_by_uid(user_uid).await?;
        let update = mapper.update(model);
        update.update_avatar(avatar).await?;
        Ok(())
    }
    pub async fn users_modify_password(&self, users_uid: Uuid, param: UsersModifyPassword) -> anyhow::Result<()> {
        let old = param.old.digest();
        let new = param.new.digest();
        
        let mapper = self.user_mapper();
        let model = mapper.query().get_user_by_uid(users_uid).await?;
        if !model.allow_use {
            return Err(anyhow::anyhow!("User is not allowed to change password"));
        }
        if old != model.password {
            return Err(anyhow::anyhow!("Old password is incorrect"));
        }
        let update = mapper.update(model);
        update.update_password(new).await?;
        Ok(())
    }
    pub async fn users_modify_email(&self, users_uid: Uuid, email: String) -> anyhow::Result<()> {
        let mapper = self.user_mapper();
        if !mapper.query().get_user_by_email(email.clone()).await.is_err() {
            return Err(anyhow::anyhow!("Email already exists"));
        }
        if !mapper.query().get_user_by_uid(users_uid).await?.allow_use {
            return Err(anyhow::anyhow!("User is not allowed to change email"));
        }
        let update = mapper.update(mapper.query().get_user_by_uid(users_uid).await?);
        update.update_email(email).await?;
        Ok(())
    }
    pub async fn users_modify_username(&self) -> anyhow::Result<()> {
        todo!()
    }
}