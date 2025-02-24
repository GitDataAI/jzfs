use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use std::io;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::services::AppState;
use crate::model::origin::{members, organization};
use crate::model::repository::repository;
use crate::model::users::users;

#[derive(Deserialize,Serialize,Clone)]
#[derive(Debug)]
pub struct AccessOwner {
    pub owner_uid: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub repos: Vec<String>,
    pub repo_uids: Vec<Uuid>,
}
#[derive(Deserialize,Serialize,Clone)]
pub struct AccessOwnerModel {
    pub owner_uid: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub repos: Vec<repository::Model>,
}


impl AppState {
    pub async fn user_access_owner(&self, uid: Uuid) -> io::Result<Vec<AccessOwner>> {
        let mut result = Vec::new();
        if let Some(x) = users::Entity::find()
            .filter(users::Column::Uid.eq(uid))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?{
            let respo = repository::Entity::find()
                .filter(repository::Column::OwnerId.eq(x.uid))
                .all(&self.read)
                .await
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
            result.push(AccessOwner {
                owner_uid: x.uid,
                name: x.username,
                avatar: x.avatar,
                repos: respo.iter().map(|x| x.name.clone()).collect(),
                repo_uids: respo.iter().map(|x| x.uid).collect(),
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
                let respo = repository::Entity::find()
                    .filter(repository::Column::OwnerId.eq(x.uid))
                    .all(&self.read)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                result.push(AccessOwner{
                    owner_uid: x.uid,
                    name: x.username,
                    avatar: x.avatar,
                    repos: respo.iter().map(|x| x.name.clone()).collect(),
                    repo_uids: respo.iter().map(|x| x.uid).collect(),
                })
            }
        }
        Ok(result)
    }
    pub async fn user_access_owner_model(&self, uid: Uuid) -> io::Result<Vec<AccessOwnerModel>> {
        let mut result = Vec::new();
        if let Some(x) = users::Entity::find()
            .filter(users::Column::Uid.eq(uid))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?{
            let respo = repository::Entity::find()
                .filter(repository::Column::OwnerId.eq(x.uid))
                .all(&self.read)
                .await
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
            result.push(AccessOwnerModel {
                owner_uid: x.uid,
                name: x.username,
                avatar: x.avatar,
                repos: respo,
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
                let respo = repository::Entity::find()
                    .filter(repository::Column::OwnerId.eq(x.uid))
                    .all(&self.read)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
                result.push(AccessOwnerModel{
                    owner_uid: x.uid,
                    name: x.username,
                    avatar: x.avatar,
                    repos: respo,
                })
            }
        }
        Ok(result)
    }
}