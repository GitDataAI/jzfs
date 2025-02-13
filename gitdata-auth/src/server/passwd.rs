use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::session::UsersSessionModel;
use lib_entity::users::users;
use serde::Deserialize;
use serde::Serialize;

use crate::server::AppAuthState;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PasswordAuth {
    pub username : String,
    pub password : String,
}

impl AppAuthState {
    pub async fn auth_password(&self, param : PasswordAuth) -> anyhow::Result<UsersSessionModel> {
        let model = users::Entity::find()
            .filter(users::Column::Username.eq(param.username.clone()))
            .filter(users::Column::MainEmail.eq(param.username))
            .one(&self.read)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        if !model.verify_password(&param.password) {
            return Err(anyhow::anyhow!("password error"));
        }
        Ok(UsersSessionModel::from(model))
    }
}
