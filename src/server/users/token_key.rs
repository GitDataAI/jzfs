use crate::error::{JZError, JZResult};
use crate::models::users::token_key;
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn users_token_del(&self, uid: Uuid, token: String) -> JZResult<()> {
        let result = token_key::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(token_key::Column::UserId.eq(uid))
                    .add(token_key::Column::Content.eq(token)),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[48] {:?}", err))),
        }
    }
    pub async fn users_token_list(&self, uid: Uuid) -> JZResult<Vec<token_key::Model>> {
        let models = token_key::Entity::find()
            .filter(token_key::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
    pub async fn users_token_add(
        &self,
        uid: Uuid,
        token: String,
        access: i16,
        expire: i64,
        name: String,
    ) -> JZResult<token_key::Model> {
        let result = token_key::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(uid),
            content: Set(token),
            access: Set(access),
            name: Set(name),
            created: Set(chrono::Local::now().timestamp()),
            updated: Set(chrono::Local::now().timestamp()),
            expire: Set(expire),
            hasused: Set(chrono::Local::now().timestamp()),
        }
        .insert(&self.database)
        .await;
        match result {
            Ok(model) => Ok(model),
            Err(err) => Err(JZError::Other(anyhow!("[47] {:?}", err))),
        }
    }
}
