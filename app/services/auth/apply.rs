use std::io;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;
use uuid::Uuid;
use crate::app::services::AppState;
use crate::model::users::users;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ApplyParma {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl AppState {
    pub async fn auth_apply(
        &self,
        parma: ApplyParma,
    )  -> io::Result<users::Model> {
        let user = users::ActiveModel {
            uid: Set(Uuid::new_v4()),
            username: Set(parma.username.clone()),
            password: Set(parma.password.digest()),
            description: Set(None),
            website: Set(None),
            avatar: Set(None),
            location: Set(None),
            language: Set(None),
            theme: Set(None),
            timezone: Set(None),
            setting: Set(vec![]),
            active: Set(true),
            created_at: Set(chrono::Local::now().naive_local()),
            email: Set(parma.email),
            name: Set(parma.username),
            updated_at: Set(chrono::Local::now().naive_local()),
        };
        let user = user.insert(&self.write).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(user)
    }
}