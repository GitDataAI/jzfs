use crate::{AppCore, Paginator};
use anyhow::anyhow;
use database::entity::user_access_keys;
use error::AppError;
use sea_orm::ColumnTrait;
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::{QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use session::Session;
use sha256::Sha256Digest;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SettingAccessNewParam {
    pub title: String,
    pub description: Option<String>,
    pub expiration: String,
    // access 0 no 1 read 2 read and write
    pub repo_access: i32,
    pub email_access: i32,
    pub event_access: i32,
    pub follow_access: i32,
    pub gpg_access: i32,
    pub ssh_access: i32,
    pub webhook_access: i32,
    pub wiki_access: i32,
    pub project_access: i32,
    pub issue_access: i32,
    pub comment_access: i32,
    pub profile_access: i32,
}

impl AppCore {
    pub async fn setting_access_key_new(
        &self,
        session: Session,
        param: SettingAccessNewParam,
    ) -> Result<String, AppError> {
        let user_session = self.user_context_current(session).await?;
        if param.title.len() < 1 || param.title.len() > 50 {
            return Err(AppError::from(anyhow!(
                "Title length must be between 1 and 50 characters"
            )));
        }
        if user_access_keys::Entity::find()
            .filter(user_access_keys::Column::Title.eq(param.title.clone()))
            .filter(user_access_keys::Column::ResourceOwnerUid.eq(user_session.user_uid))
            .one(&self.db)
            .await?
            .is_some()
        {
            return Err(AppError::from(anyhow!("Access key already exists")));
        }
        let token = format!("_gta{}", Uuid::new_v4().to_string().digest());
        let fingerprint = format!("SHA256:{}", sha256::digest(token.clone()));
        let active = user_access_keys::ActiveModel {
            uid: Set(Uuid::now_v7()),
            title: Set(param.title),
            description: Set(param.description),
            token: Set(token.clone()),
            use_history: Set(vec![]),
            resource_owner: Set(user_session.username),
            resource_owner_uid: Set(user_session.user_uid),
            expiration: Set(param.expiration),
            fingerprint: Set(fingerprint),
            repo_access: Set(param.repo_access),
            email_access: Set(param.email_access),
            event_access: Set(param.event_access),
            follow_access: Set(param.follow_access),
            gpg_access: Set(param.gpg_access),
            ssh_access: Set(param.ssh_access),
            webhook_access: Set(param.webhook_access),
            wiki_access: Set(param.wiki_access),
            project_access: Set(param.project_access),
            issue_access: Set(param.issue_access),
            comment_access: Set(param.comment_access),
            profile_access: Set(param.profile_access),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        };
        active.insert(&self.db).await?;
        Ok(token)
    }
    pub async fn setting_access_key_delete(
        &self,
        session: Session,
        name: &str,
    ) -> Result<(), AppError> {
        let user_session = self.user_context_current(session).await?;
        user_access_keys::Entity::delete_many()
            .filter(user_access_keys::Column::ResourceOwnerUid.eq(user_session.user_uid))
            .filter(user_access_keys::Column::Title.eq(name))
            .exec(&self.db)
            .await?;
        Ok(())
    }
    pub async fn setting_access_key_list(
        &self,
        session: Session,
        paginator: Paginator,
    ) -> Result<Vec<user_access_keys::Model>, AppError> {
        let user_session = self.user_context_current(session).await?;
        user_access_keys::Entity::find()
            .filter(user_access_keys::Column::ResourceOwnerUid.eq(user_session.user_uid))
            .order_by_desc(user_access_keys::Column::CreatedAt)
            .limit(paginator.page_size)
            .offset(paginator.page * paginator.page_size)
            .all(&self.db)
            .await
            .map_err(|e| AppError::from(anyhow!(e)))
    }
    pub async fn inner_setting_access_key_find(
        &self,
        token: String,
    ) -> Result<user_access_keys::Model, AppError> {
        user_access_keys::Entity::find()
            .filter(user_access_keys::Column::Token.eq(token))
            .one(&self.db)
            .await
            .map_err(|e| AppError::from(anyhow!(e)))?
            .ok_or(AppError::from(anyhow!("Not found")))
    }
}
