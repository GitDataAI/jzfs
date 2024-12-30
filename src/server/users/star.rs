use crate::error::{JZError, JZResult};
use crate::models::repos::repos;
use crate::models::users::star;
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::prelude::Expr;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn users_star_add(&self, uid: Uuid, rid: Uuid) -> JZResult<star::Model> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[34] User Not Found")));
        }
        if !self.check_repo_id(rid).await? {
            return Err(JZError::Other(anyhow!("[35] Repo Not Found")));
        }
        if self.check_users_star(uid, rid).await? {
            return Err(JZError::Other(anyhow!("[36] Star already exists")));
        }
        let result = star::ActiveModel {
            uid: ActiveValue::Set(Uuid::new_v4()),
            user_id: ActiveValue::Set(uid),
            repo_id: ActiveValue::Set(rid),
            created: ActiveValue::Set(chrono::Local::now().timestamp()),
        }
        .insert(&self.database)
        .await;
        match result {
            Ok(model) => {
                repos::Entity::update_many()
                    .col_expr(
                        repos::Column::NumsStar,
                        Expr::add(Expr::col(repos::Column::NumsStar), 1),
                    )
                    .filter(repos::Column::Uid.eq(rid))
                    .exec(&self.database)
                    .await?;
                Ok(model)
            }
            Err(err) => Err(JZError::Other(anyhow!("[37] {:?}", err))),
        }
    }
    pub async fn users_star_del(&self, uid: Uuid, rid: Uuid) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[38] User Not Found")));
        }
        if !self.check_repo_id(rid).await? {
            return Err(JZError::Other(anyhow!("[39] Repo Not Found")));
        }
        if !self.check_users_star(uid, rid).await? {
            return Err(JZError::Other(anyhow!("[40] Star Not Found")));
        }
        let result = star::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(star::Column::UserId.eq(uid))
                    .add(star::Column::RepoId.eq(rid)),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => {
                repos::Entity::update_many()
                    .col_expr(
                        repos::Column::NumsStar,
                        Expr::sub(Expr::col(repos::Column::NumsStar), 1),
                    )
                    .filter(repos::Column::Uid.eq(rid))
                    .exec(&self.database)
                    .await?;
                Ok(())
            }
            Err(err) => Err(JZError::Other(anyhow!("[41] {:?}", err))),
        }
    }
    pub async fn users_star_list(&self, uid: Uuid) -> JZResult<Vec<star::Model>> {
        let models = star::Entity::find()
            .filter(star::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
}
