use sea_orm::ColumnTrait;
use std::io;
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app::services::AppState;
use crate::blob::GitBlob;
use crate::model::repository::repository;
use crate::model::users::{follow, star, users};

#[derive(Deserialize,Serialize)]
pub struct UserDashBored {
    pub user: users::Model,
    pub repos: Vec<repository::Model>,
    pub stars: Vec<star::Model>,
    pub follow: Vec<follow::Model>,
    pub followed: Vec<follow::Model>,
    pub readme: Option<String>,
}

impl AppState {
    pub async fn user_info_by_username(&self, username: String) -> io::Result<users::Model> {
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Database Err"))?
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "User Not Found"))?;
        Ok(user)
    }
    pub async fn user_info_by_uid(&self, uid: Uuid) -> io::Result<users::Model> {
        let user = users::Entity::find_by_id(uid)
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Database Err"))?
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "User Not Found"))?;
        Ok(user)
    }
    pub async fn user_info_by_email(&self, email: String) -> io::Result<users::Model> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Database Err"))?
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "User Not Found"))?;
        Ok(user)
    }
    
    pub async fn user_dashbored(&self, uid: Uuid) -> io::Result<UserDashBored> {
        let user = self.user_info_by_uid(uid).await?;
        let repos = repository::Entity::find()
            .filter(repository::Column::OwnerId.eq(user.uid))
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Database Err"))?;
        let stars = star::Entity::find()
            .filter(star::Column::UserId.eq(user.uid))
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Database Err"))?;
        let follow = follow::Entity::find()
            .filter(follow::Column::UserId.eq(user.uid))
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Database Err"))?;
        let followed = follow::Entity::find()
            .filter(follow::Column::TargetId.eq(user.uid))
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Database Err"))?;
        let readme = if let Ok(repo) = self.repo_info(user.clone().username, "readme".to_string()).await {
            let path = format!("{}/{}/{}", crate::app::http::GIT_ROOT, repo.node_uid, repo.uid);
            if let Ok(blob) = GitBlob::new(path.into()){
                if let Ok(blob) = blob.file_readme() {
                    Some(std::str::from_utf8(&blob).unwrap().to_string())
                }else { 
                    None
                }
            }else { 
                None
            }
        }else { 
            None
        };
        Ok(UserDashBored {
            user,
            repos,
            stars,
            follow,
            followed,
            readme,
        })
    }
}