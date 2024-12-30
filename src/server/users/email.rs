use crate::error::{JZError, JZResult};
use crate::models::users::{email, users};
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::prelude::Expr;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn users_info_email(&self, email: String) -> JZResult<users::Model> {
        let models = users::Entity::find()
            .filter(users::Column::MainEmail.eq(email))
            .one(&self.database)
            .await?;
        if models.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[09] User Not Found")));
        }
        Ok(models.unwrap())
    }
    pub async fn users_emails_uid(&self, uid: Uuid) -> JZResult<Vec<email::Model>> {
        let models = email::Entity::find()
            .filter(email::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
    pub async fn users_email_add(&self, uid: Uuid, email: String) -> JZResult<email::Model> {
        if self.check_user_email(email.clone()).await? {
            return Err(JZError::Other(anyhow!("[10] Email already exists")));
        }
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[11] User Not Found")));
        }
        let result = email::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(uid),
            content: Set(email),
            main: Set(false),
            primary: Set(true),
            created: Set(chrono::Local::now().timestamp()),
            updated: Set(chrono::Local::now().timestamp()),
            hasused: Set(chrono::Local::now().timestamp()),
        }
        .insert(&self.database)
        .await;
        match result {
            Ok(model) => Ok(model),
            Err(err) => Err(JZError::Other(anyhow!("[12] {:?}", err))),
        }
    }
    pub async fn users_email_del(&self, uid: Uuid, email: String) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[13] User Not Found")));
        }
        let result = email::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(email::Column::UserId.eq(uid))
                    .add(email::Column::Content.eq(email)),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[14] {:?}", err))),
        }
    }
    pub async fn users_email_set_main(&self, uid: Uuid, email: String) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[15] User Not Found")));
        }
        let result = email::Entity::update_many()
            .col_expr(email::Column::Main, Expr::value(false))
            .col_expr(email::Column::Primary, Expr::value(true))
            .filter(
                Condition::all()
                    .add(email::Column::UserId.eq(uid))
                    .add(email::Column::Content.eq(email.clone())),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => {
                let result = email::Entity::update_many()
                    .col_expr(email::Column::Main, Expr::value(true))
                    .col_expr(email::Column::Primary, Expr::value(true))
                    .filter(
                        Condition::all()
                            .add(email::Column::UserId.eq(uid))
                            .add(email::Column::Content.eq(email.clone())),
                    )
                    .exec(&self.database)
                    .await;
                match result {
                    Ok(_) => {
                        let result = users::Entity::update_many()
                            .col_expr(users::Column::MainEmail, Expr::value(email))
                            .filter(users::Column::Uid.eq(uid))
                            .exec(&self.database)
                            .await;
                        match result {
                            Ok(_) => Ok(()),
                            Err(err) => Err(JZError::Other(anyhow!("[18] {:?}", err))),
                        }
                    }
                    Err(err) => Err(JZError::Other(anyhow!("[16] {:?}", err))),
                }
            }
            Err(err) => Err(JZError::Other(anyhow!("[17] {:?}", err))),
        }
    }
    pub async fn users_email_set_primary(
        &self,
        uid: Uuid,
        email: String,
        primary: bool,
    ) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[19] User Not Found")));
        }
        let result = email::Entity::update_many()
            .col_expr(email::Column::Primary, Expr::value(primary))
            .filter(
                Condition::all()
                    .add(email::Column::UserId.eq(uid))
                    .add(email::Column::Content.eq(email.clone())),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[21] {:?}", err))),
        }
    }
    pub async fn users_email_list(&self, uid: Uuid) -> JZResult<Vec<email::Model>> {
        let models = email::Entity::find()
            .filter(email::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
    pub async fn users_update_email(&self, uid: Uuid, email: String) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[55] User Not Found")));
        }
        let result = users::Entity::update_many()
            .filter(users::Column::Uid.eq(uid))
            .col_expr(users::Column::MainEmail, Expr::value(email))
            .col_expr(
                users::Column::Updated,
                Expr::value(chrono::Local::now().timestamp()),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[56] {:?}", err))),
        }
    }
}
