use sea_orm::ColumnTrait;
use std::io;
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use futures::try_join;
use crate::services::AppState;
use crate::blob::GitBlob;
use crate::model::repository::repository;
use crate::model::users::{follow, star, users, watch};
use crate::services::product::list::ProductList;

#[derive(Deserialize, Serialize)]
pub struct UserDashBored {
    pub user: users::Model,
    pub repos: Vec<repository::Model>,
    pub stars: Vec<star::Model>,
    pub following: Vec<follow::Model>,  
    pub followers: Vec<follow::Model>, 
    pub watch: Vec<watch::Model>,
    pub readme: Option<String>,
    pub products: Vec<ProductList>,
}

fn db_error<E: std::error::Error>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("Database error: {}", e))
}

impl AppState {
    async fn find_user_by_column(
        &self,
        column: users::Column,
        value: impl Into<sea_orm::Value>,
    ) -> io::Result<users::Model> {
        users::Entity::find()
            .filter(column.eq(value))
            .one(&self.read)
            .await
            .map_err(db_error)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "User Not Found"))
    }

    pub async fn user_info_by_username(&self, username: String) -> io::Result<users::Model> {
        self.find_user_by_column(users::Column::Username, username).await
    }

    pub async fn user_info_by_uid(&self, uid: Uuid) -> io::Result<users::Model> {
        users::Entity::find_by_id(uid)
            .one(&self.read)
            .await
            .map_err(db_error)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "User Not Found"))
    }

    pub async fn user_info_by_email(&self, email: String) -> io::Result<users::Model> {
        self.find_user_by_column(users::Column::Email, email).await
    }

    pub async fn user_dashbored(&self, uid: Uuid) -> io::Result<UserDashBored> {
        let user = self.user_info_by_uid(uid).await?;
        let (
            repos,
            stars,
            following,
            followers,
            watch,
        ) = try_join!(
            repository::Entity::find()
                .filter(repository::Column::OwnerId.eq(user.uid))
                .all(&self.read),
            star::Entity::find()
                .filter(star::Column::UserId.eq(user.uid))
                .all(&self.read),
            follow::Entity::find()
                .filter(follow::Column::UserId.eq(user.uid))
                .all(&self.read),
            follow::Entity::find()
                .filter(follow::Column::TargetId.eq(user.uid))
                .all(&self.read),
            watch::Entity::find()
                .filter(watch::Column::UserId.eq(user.uid))
                .all(&self.read),
        ).map_err(db_error)?;
        
        let products = self.product_owner(user.uid).await?;
        
        let readme = match self.repo_info(user.username.clone(), "readme".to_string()).await {
            Ok(repo) => {
                let path = format!("{}/{}/{}", crate::http::GIT_ROOT, repo.node_uid, repo.uid);
                GitBlob::new(path.into())
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                    .and_then(|blob| 
                        blob.file_readme()
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                    )
                    .and_then(|bytes| 
                        String::from_utf8(bytes)
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                    )
                    .ok()
            }
            Err(_) => None,
        };



        Ok(UserDashBored {
            user,
            repos,
            stars,
            following,  
            followers, 
            watch,
            readme,
            products,
        })
    }
}
