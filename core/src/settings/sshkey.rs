use crate::{AppCore, Paginator};
use anyhow::anyhow;
use database::entity::ssh_keys;
use error::AppError;
use sea_orm::PaginatorTrait;
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use session::Session;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SettingSshKeyInsertParam {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
}

impl AppCore {
    pub async fn setting_ssh_key_insert(
        &self,
        param: SettingSshKeyInsertParam,
        session: Session,
    ) -> Result<(), AppError> {
        let user = self.user_context(session).await?;
        if ssh_keys::Entity::find()
            .filter(ssh_keys::Column::Content.eq(param.content.clone()))
            .one(&self.db)
            .await?
            .is_some()
        {
            return Err(AppError::from(anyhow!("SSH key already exists")));
        }
        let split = param
            .content
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let content = if split.len() == 3 {
            format!("{} {}", split[0], split[1])
        } else if split.len() == 2 {
            format!("{} {}", split[0], split[1])
        } else {
            return Err(AppError::from(anyhow!("Invalid SSH key")));
        };
        let finger = format!("SHA256:{}", sha256::digest(content.clone()));
        let active = ssh_keys::ActiveModel {
            uid: Set(Uuid::now_v7()),
            user_id: Set(user.user_uid),
            name: Set(param.name),
            fingerprint: Set(finger),
            description: Set(param.description),
            content: Set(content),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        };
        active.insert(&self.db).await?;
        Ok(())
    }
    pub async fn setting_ssh_key_delete(
        &self,
        session: Session,
        uid: Uuid,
    ) -> Result<(), AppError> {
        let user_session = self.user_context_current(session).await?;
        ssh_keys::Entity::delete_many()
            .filter(ssh_keys::Column::UserId.eq(user_session.user_uid))
            .filter(ssh_keys::Column::Uid.eq(uid))
            .exec(&self.db)
            .await?;
        Ok(())
    }
    pub async fn setting_ssh_key_list(
        &self,
        session: Session,
        paginator: Paginator,
    ) -> Result<serde_json::Value, AppError> {
        let user_session = self.user_context_current(session).await?;
        let ssh_key = ssh_keys::Entity::find()
            .filter(ssh_keys::Column::UserId.eq(user_session.user_uid))
            .order_by_desc(ssh_keys::Column::CreatedAt)
            .offset(paginator.page * paginator.page_size)
            .limit(paginator.page_size)
            .all(&self.db)
            .await
            .map_err(|_| AppError::from(anyhow!("Database conn err")))?;
        let count = ssh_keys::Entity::find()
            .filter(ssh_keys::Column::UserId.eq(user_session.user_uid))
            .count(&self.db)
            .await
            .map_err(|_| AppError::from(anyhow!("Database conn err")))?;
        Ok(json!({
            "total_count": count,
            "items": ssh_key
        }))
    }
    pub async fn inner_setting_ssh_key_find(
        &self,
        ssh_key: String,
    ) -> Result<ssh_keys::Model, AppError> {
        let ssh_key = ssh_keys::Entity::find()
            .filter(ssh_keys::Column::Content.eq(ssh_key))
            .one(&self.db)
            .await?;
        Ok(ssh_key.ok_or(AppError::from(anyhow!("Not found")))?)
    }
}
