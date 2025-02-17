use crate::app::services::AppState;
use crate::model::users::users;
use sea_orm::{ColumnTrait, Condition};
use sea_orm::{EntityTrait, QueryFilter};
use std::io;
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct AuthPasswd {
    pub username: String,
    pub password: String,
}

impl AppState {
    pub async fn auth_passwd(
        &self,
        parma: AuthPasswd,
    )  -> io::Result<users::Model> {
        let model = users::Entity::find()
            .filter(
                Condition::all()
                    .add(
                        Condition::any()
                            .add(users::Column::Username.eq(&parma.username))
                            .add(users::Column::Email.eq(&parma.username))
                    )
                    .add(
                        users::Column::Password.eq(parma.password.digest())
                    )
            )
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .ok_or(io::Error::new(io::ErrorKind::Other, "用户名或密码错误"))?;
        Ok(model)
    }
}
