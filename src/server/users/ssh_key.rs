use crate::error::{JZError, JZResult};
use crate::models::users::ssh_key;
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn users_ssh_add(
        &self,
        uid: Uuid,
        ssh_key: String,
        access: i16,
        expire: i64,
        name: String,
    ) -> JZResult<ssh_key::Model> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[42] User Not Found")));
        }
        let result = ssh_key::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(uid),
            content: Set(ssh_key),
            name: Set(name),
            access: Set(access),
            created: Set(chrono::Local::now().timestamp()),
            updated: Set(chrono::Local::now().timestamp()),
            expire: Set(expire),
            hasused: Set(chrono::Local::now().timestamp()),
        }
        .insert(&self.database)
        .await;
        match result {
            Ok(model) => Ok(model),
            Err(err) => Err(JZError::Other(anyhow!("[43] {:?}", err))),
        }
    }
    pub async fn users_ssh_del(&self, uid: Uuid, ssh_key: Uuid) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[44] User Not Found")));
        }
        let result = ssh_key::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(ssh_key::Column::UserId.eq(uid))
                    .add(ssh_key::Column::Uid.eq(ssh_key)),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[46] {:?}", err))),
        }
    }
    pub async fn users_ssh_list(&self, uid: Uuid) -> JZResult<Vec<ssh_key::Model>> {
        let models = ssh_key::Entity::find()
            .filter(ssh_key::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
}
