use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use std::io;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app::services::AppState;
use crate::model::origin::{members, organization};
use crate::model::users::users;

#[derive(Deserialize,Serialize,Clone)]
pub struct AccessOwner {
    pub owner_uid: Uuid,
    pub name: String,
    pub avatar: Option<String>,
}


impl AppState {
    pub async fn user_access_owner(&self, uid: Uuid) -> io::Result<Vec<AccessOwner>> {
        let mut result = Vec::new();
        if let Some(x) = users::Entity::find()
            .filter(users::Column::Uid.eq(uid))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?{
            result.push(AccessOwner {
                owner_uid: x.uid,
                name: x.username,
                avatar: x.avatar,
            })
        }
        let members = members::Entity::find()
            .filter(members::Column::UsersUid.eq(uid))
            .filter(members::Column::Access.gte(2))
            .all(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        for x in members {
            if let Some(x) = organization::Entity::find()
                .filter(organization::Column::Uid.eq(x.group_uid))
                .one(&self.read)
                .await
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?{
                result.push(AccessOwner{
                    owner_uid: x.uid,
                    name: x.username,
                    avatar: x.avatar,
                })
            }
        }
        Ok(result)
    }
}