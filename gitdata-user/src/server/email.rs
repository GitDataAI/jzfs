use serde::Deserialize;
use lib_entity::{ActiveModelTrait, IntoActiveModel, QueryFilter};
use lib_entity::ActiveValue::Set;
use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::prelude::Uuid;
use lib_entity::sqlx::types::chrono::Utc;
use lib_entity::users::{email, users};
use crate::server::AppUserState;

#[derive(Deserialize,Clone)]
pub struct MainEmailUpdate {
    pub email: String,
}

#[derive(Deserialize,Clone)]
pub struct EmailAddParma {
    pub email: String,
    pub primary: bool,
}

impl AppUserState {
    pub async fn user_visible_email_can(&self, user_uid: Uuid) -> anyhow::Result<()> {
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        let mut active = model.into_active_model();
        active.visible_email = Set(true);
        active.update(&self.write).await?;
        Ok(())
    }
    pub async fn user_visible_email_cannot(&self, user_uid: Uuid) -> anyhow::Result<()> {
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        let mut active = model.into_active_model();
        active.visible_email = Set(false);
        active.update(&self.write).await?;
        Ok(())
    }
    pub async fn user_visible_email_is(&self, user_uid: Uuid) -> anyhow::Result<bool> {
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        Ok(model.visible_email)
    }
    pub async fn user_email_update(&self, user_uid: Uuid, parma: MainEmailUpdate) -> anyhow::Result<()> {
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        let mut active = model.into_active_model();
        active.main_email = Set(parma.email);
        active.update(&self.write).await?;
        Ok(())
    }
    pub async fn user_email_add(&self, user_uid: Uuid, parma: EmailAddParma) -> anyhow::Result<()> {
        users::Entity::find()
            .filter(users::Column::Uid.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        if email::Entity::find()
            .filter(email::Column::UserId.eq(user_uid))
            .filter(email::Column::Content.eq(parma.email.clone()))
            .one(&self.read)
            .await?
            .is_some() {
            return Err(anyhow::anyhow!("邮箱已存在"));
        }
        let active = email::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_uid),
            content: Set(parma.email),
            main: Set(false),
            primary: Set(parma.primary),
            created: Set(Utc::now().timestamp()),
            updated: Set(Utc::now().timestamp()),
            hasused: Set(0),
        };
        active.insert(&self.write).await?;
        Ok(())
    }
    pub async fn user_email_delete(&self, user_uid: Uuid, email: String) -> anyhow::Result<()> {
        let model = email::Entity::find()
            .filter(email::Column::Content.eq(email))
            .filter(email::Column::UserId.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("邮箱不存在"))?;
        model.into_active_model().delete(&self.write).await?;
        Ok(())
    }
    pub async fn user_email_list(&self, user_uid: Uuid) -> anyhow::Result<Vec<email::Model>> {
        let models = email::Entity::find()
            .filter(email::Column::UserId.eq(user_uid))
            .all(&self.read)
            .await?;
        Ok(models)
    }
    pub async fn user_email_set_primary(&self, user_uid: Uuid, email: String) -> anyhow::Result<()> {
        let model = email::Entity::find()
            .filter(email::Column::Content.eq(email))
            .filter(email::Column::UserId.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("邮箱不存在"))?;
        let mut active = model.into_active_model();
        active.primary = Set(true);
        active.update(&self.write).await?;
        Ok(())
    }
    pub async fn user_email_set_no_primary(&self, user_uid: Uuid, email: String) -> anyhow::Result<()> {
        let model = email::Entity::find()
            .filter(email::Column::Content.eq(email))
            .filter(email::Column::UserId.eq(user_uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("邮箱不存在"))?;
        let mut active = model.into_active_model();
        active.primary = Set(false);
        active.update(&self.write).await?;
        Ok(())
    }
    
}