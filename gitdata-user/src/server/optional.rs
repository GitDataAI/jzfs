use lib_entity::{ActiveModelTrait, ColumnTrait, IntoActiveModel};
use lib_entity::{EntityTrait, QueryFilter};
use lib_entity::prelude::Uuid;
use lib_entity::users::users;
use lib_entity::users::users::{UpdateOption, UsersOption};
use crate::server::AppUserState;

impl AppUserState {
    pub async fn update_optional(&self, uid: Uuid, parma: UpdateOption) -> anyhow::Result<()> {
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        match model.into_active_model()
            .update_optional(parma)
            .update(&self.write)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
    pub async fn acquire_optional(&self, uid: Uuid) -> anyhow::Result<UsersOption> {
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        Ok(UsersOption::from(model))
    }
    pub async fn username_to_uid(&self, username: String) -> anyhow::Result<Uuid> {
        let model = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("用户不存在"))?;
        Ok(model.uid)
    }
}