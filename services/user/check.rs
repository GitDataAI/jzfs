use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use std::io;
use sea_orm::{Condition, EntityTrait};
use serde::{Deserialize, Serialize};
use crate::services::AppState;
use crate::model::users::users;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCheckParma {
    pub email: Option<String>,
    pub username: Option<String>,
}

impl AppState {
    pub async fn users_check(&self, parma: UserCheckParma) -> io::Result<bool> {
        let mut condition = Condition::any() ;
        if let Some(email) = parma.email {
            condition = condition.add(users::Column::Email.eq(email))
        }
        if let Some(username) = parma.username {
            condition = condition.add(users::Column::Username.eq(username))
        }
        users::Entity::find()
            .filter(condition)
            .all(&self.read)
            .await
            .map(|us| us.is_empty())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "users_check error"))
    }
}