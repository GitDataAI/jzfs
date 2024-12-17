use crate::api::dto::user_dto::UserOv;
use crate::metadata::model::groups::{groups, groups_users};
use crate::metadata::model::users::users;
use crate::metadata::service::groups_service::GroupService;
use sea_orm::*;
use time::OffsetDateTime;
use uuid::Uuid;

impl GroupService {
    pub async fn members(&self, group_id: Uuid) -> anyhow::Result<Vec<UserOv>>{
        let models = groups_users::Entity::find()
            .filter(groups_users::Column::GroupId.eq(group_id))
            .all(&self.db)
            .await;
        match models{
            Ok(models) => {
                let users = users::Entity::find()
                    .filter(users::Column::Uid.is_in(models.iter().map(|model| model.users_id)))
                    .all(&self.db)
                    .await;
                match users{
                    Ok(users) => {
                        let mut result = Vec::new();
                        for model in models.iter(){
                            for user in users.iter(){
                                if model.users_id == user.uid{
                                    result.push(UserOv::from(user.clone()));
                                }
                            }
                        }
                        Ok(result)
                    },
                    Err(e) => {
                        Err(anyhow::anyhow!(e))
                   }
                }
            },
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
    pub async fn member_add(&self, group_id: Uuid, user_id: Uuid, access: i32) -> anyhow::Result<()>{
        let model = groups_users::Entity::find()
            .filter(groups_users::Column::GroupId.eq(group_id))
            .filter(groups_users::Column::UsersId.eq(user_id))
            .one(&self.db)
            .await?;
        if model.is_some(){
            return Err(anyhow::anyhow!("Already Exists"))
        }
        let model = groups_users::ActiveModel{
            uid: Set(Uuid::new_v4()),
            group_id: Set(group_id),
            users_id: Set(user_id),
            access: Set(access),
            join_at: Set(OffsetDateTime::now_utc()),
        };
        model.insert(&self.db).await?;
        Ok(())
    }
    pub async fn member_remove(&self, group_id: Uuid, user_id: Uuid) -> anyhow::Result<()>{
        let model = groups_users::Entity::find()
            .filter(groups_users::Column::GroupId.eq(group_id))
            .filter(groups_users::Column::UsersId.eq(user_id))
            .one(&self.db)
            .await?;
        if model.is_none(){
            return Err(anyhow::anyhow!("Not Found"))
        }
        model.unwrap().delete(&self.db).await?;
        Ok(())
    }
    pub async fn find_member(&self, user_id: Uuid) -> anyhow::Result<Vec<groups::Model>>{
        let models = groups_users::Entity::find()
            .filter(groups_users::Column::UsersId.eq(user_id))
            .all(&self.db)
            .await;
        match models{
            Ok(models) => {
                let models = groups::Entity::find()
                    .filter(groups::Column::Uid.is_in(models.iter().map(|model| model.group_id)))
                    .all(&self.db)
                    .await?;                
                Ok(models)
            },
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
}