use lib_entity::ActiveValue::Set;
use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::ModelTrait;
use lib_entity::QueryFilter;
use lib_entity::prelude::Uuid;
use lib_entity::sqlx::types::chrono;
use lib_entity::users::ssh_key;
use serde::Deserialize;

use crate::server::AppUserState;

#[derive(Deserialize, Clone)]
pub struct UserCreateSshKey {
    pub name : String,
    pub content : String,
    pub access : i64,
    pub expire : i64,
}
#[derive(Deserialize, Clone)]
pub struct UserTokenDelete {
    pub name : String,
    pub uid : Uuid,
}

impl AppUserState {
    pub async fn create_ssh_key(
        &self,
        user_id : Uuid,
        token : UserCreateSshKey,
    ) -> std::io::Result<()> {
        if !ssh_key::Entity::find()
            .filter(ssh_key::Column::Content.eq(token.content.clone()))
            .all(&self.read)
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to query ssh_key"))?
            .is_empty()
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "SshKey already exists",
            ));
        }
        let active = ssh_key::ActiveModel {
            user_id : Set(user_id),
            uid : Set(Uuid::new_v4()),
            content : Set(token.content),
            name : Set(token.name),
            access : Set(token.access),
            created : Set(chrono::Utc::now().timestamp()),
            updated : Set(chrono::Utc::now().timestamp()),
            expire : Set(token.expire),
            hasused : Set(0),
        };
        ssh_key::Entity::insert(active)
            .exec(&self.write)
            .await
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to insert ssh_key")
            })?;
        Ok(())
    }
    pub async fn delete_ssh_key(
        &self,
        user_id : Uuid,
        token : UserTokenDelete,
    ) -> std::io::Result<()> {
        let active = ssh_key::Entity::find_by_id(token.uid)
            .one(&self.read)
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to query ssh_key"))?
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "SshKey not found",
            ))?;
        if active.user_id != user_id {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "SshKey not found",
            ));
        }
        active.delete(&self.write).await.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to delete ssh_key")
        })?;
        Ok(())
    }
    pub async fn list_ssh_key(&self, user_id : Uuid) -> std::io::Result<Vec<ssh_key::Model>> {
        ssh_key::Entity::find()
            .filter(ssh_key::Column::UserId.eq(user_id))
            .all(&self.read)
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to query ssh_key"))
    }
}
