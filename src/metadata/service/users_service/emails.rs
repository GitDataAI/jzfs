use uuid::Uuid;
use crate::metadata::model::users::users_email;
use crate::metadata::service::users_service::UserService;
use sea_orm::*;

impl UserService {
    pub async fn email(&self, uid: Uuid) -> anyhow::Result<Vec<users_email::Model>> {
        anyhow::Ok(users_email::Entity::find()
            .filter(
                users_email::Column::UserId.eq(uid)
            )
            .all(&self.db)
            .await?
        )
    }
    pub async fn bind(&self, email: String, user_id: Uuid, name: Option<String>) ->anyhow::Result<()>{
        let model = users_email::Entity::find()
            .filter(users_email::Column::Email.eq(email.clone()))
            .one(&self.db)
            .await?;
        if model.is_some(){
            return Err(anyhow::anyhow!("Email already exists"));
        }
        users_email::Entity::insert(users_email::ActiveModel{
            uid: Set(user_id),
            email: Set(email),
            is_public: Set(true),
            name: Set(name.unwrap_or("".to_string())),
            verified: Set(false),
            ..Default::default()
        }).exec(&self.db).await?;
        Ok(())
    }
    pub async fn unbind(&self, email: String, user_id: Uuid) ->anyhow::Result<()>{
        users_email::Entity::delete_many()
            .filter(users_email::Column::Email.eq(email))
            .filter(users_email::Column::Uid.eq(user_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
    pub async fn verify(&self, _email: String, _code: String) ->anyhow::Result<()>{
        todo!()
    }

}