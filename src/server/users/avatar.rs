use crate::error::{JZError, JZResult};
use crate::models::users::users;
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::prelude::Expr;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn users_avatar_get(&self, uid: Uuid) -> JZResult<Option<String>> {
        let model = users::Entity::find_by_id(uid).one(&self.database).await?;
        match model {
            Some(model) => Ok(model.avatar_url),
            None => Err(JZError::Other(anyhow!("[55] User Not Found"))),
        }
    }
    pub async fn users_avatar_set(&self, uid: Uuid, avatar_url: String) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[56] User Not Found")));
        }
        let result = users::Entity::update_many()
            .filter(users::Column::Uid.eq(uid))
            .col_expr(users::Column::AvatarUrl, Expr::value(avatar_url))
            .col_expr(
                users::Column::Updated,
                Expr::value(chrono::Local::now().timestamp()),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[57] {:?}", err))),
        }
    }
}
