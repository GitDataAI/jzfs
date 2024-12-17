use uuid::Uuid;
use crate::metadata::model::users::users_data;
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
use crate::metadata::model::groups::{groups_data, groups_users};

impl RepoService {
    pub async fn repo_access_user(&self, repo_id: Uuid, user_id: Uuid) -> anyhow::Result<i32>{
        let users_data = users_data::Entity::find()
            .filter(users_data::Column::UserId.eq(user_id))
            .filter(users_data::Column::Repo.contains(repo_id))
            .one(&self.db)
            .await?;
        if let Some(data) = users_data{
            Ok(if data.repo.contains(&repo_id) { 3 } else { 10 })
        }else { 
            let group_data = groups_data::Entity::find()
                .filter(groups_data::Column::RepoId.eq(repo_id))
                .one(&self.db)
                .await?;
            if let Some(data) = group_data{
                let group_id = data.group_id;
                let group_users = groups_users::Entity::find()
                    .filter(groups_users::Column::GroupId.eq(group_id))
                    .filter(groups_users::Column::UsersId.eq(user_id))
                    .one(&self.db)
                    .await?;
                if let Some(data) = group_users{
                    Ok(data.access)
                }else {
                    Ok(10)
                }
            }else {
                Ok(10)
            }
        }
    }
}