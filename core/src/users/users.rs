use crate::AppCore;
use anyhow::anyhow;
use database::entity::users;
use error::AppError;
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{Condition, QueryFilter};
use serde_json::json;
use session::Session;

impl AppCore {
    pub async fn users(
        &self,
        username: &str,
        session: Session,
    ) -> Result<serde_json::Value, AppError> {
        let owner = self.user_context(session).await.ok();
        let mut value = serde_json::Value::Null;
        let user = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(username))
                    .add(users::Column::DisplayName.eq(username)),
            )
            .one(&self.db)
            .await?
            .ok_or(AppError::from(anyhow!("user not found")))?;
        value["model"] = json!(user);
        if let Some(owner) = owner {
            if owner.user_uid == user.uid {
                value["is_owner"] = json!(true);
                value["is_follow"] = json!(false);
            } else {
                value["is_owner"] = json!(false);
                let follow = self.users_follow_is(owner.user_uid, user.uid).await;
                value["is_follow"] = json!(follow);
            }
        } else {
            value["is_owner"] = json!(false);
        }
        Ok(value)
    }
}
