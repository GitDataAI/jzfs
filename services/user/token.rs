use sea_orm::{ActiveModelTrait, ColumnTrait, Set};
use std::io;
use std::ops::Add;
use chrono::{Datelike, Days, NaiveDateTime, Utc};
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::token::TokenUtils;
use crate::model::users::{tokens, users};
use crate::services::AppState;

#[derive(Deserialize,Serialize,Clone)]
pub struct TokenCreate {
    name: String,
    description: Option<String>,
    expire: i64,
    access: i64
}

#[derive(Deserialize,Serialize,Clone)]
pub struct TokenCreateReopens {
    uid: Uuid,
    token: String,
    expire: i64
}

#[derive(Deserialize,Serialize,Clone)]
pub struct TokenDelete {
    uid: Uuid,
    name: String
}

// #[derive(Deserialize,Serialize,Clone)]
// pub enum TokenAccess {
//     Read = 1,
//     Write = 2,
//     Admin = 3,
//     All = 4
// }


impl AppState {
    pub async fn token_list(&self, users_uid: Uuid) -> io::Result<Vec<tokens::Model>> {
        tokens::Entity::find()
            .filter(tokens::Column::UserId.eq(users_uid))
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get token list"))
    }
    pub async fn token_create(&self, users_uid: Uuid,param: TokenCreate) -> io::Result<TokenCreateReopens> {
        let token = TokenUtils::generate_token();
        let expires = if param.expire == 0 {
            let date = Utc::now().date_naive();
            date.checked_sub_days(Days::new(date.day() as u64)).unwrap()
        } else {
            Utc::now().date_naive().add(chrono::Duration::days(param.expire))
        };
        let now = Utc::now().naive_local();
        let access = match param.access {
            1 => "read".to_string(),
            2 => "write".to_string(),
            3 => "admin".to_string(),
            _ => "all".to_string()
        };
        let entity = tokens::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: sea_orm::ActiveValue::Set(users_uid),
            name: Set(param.name),
            description: Set(param.description),
            token: sea_orm::ActiveValue::Set(token.clone()),
            access: Set(access),
            use_history: Set(vec![]),
            created_at: Set(now),
            updated_at: Set(now),
            expires_at: Set(NaiveDateTime::from(expires)),
        };
        let model = entity.insert(&self.write)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create token"))?;
        Ok(TokenCreateReopens {
            uid: model.uid,
            token,
            expire: param.expire
        })
    }
    pub async fn token_delete(&self, users_uid: Uuid,param: TokenDelete) -> io::Result<()> {
        tokens::Entity::delete_many()
            .filter(tokens::Column::UserId.eq(users_uid))
            .filter(tokens::Column::Name.eq(param.name))
            .exec(&self.write)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to delete token"))?;
        Ok(())
    }
    
    pub async fn self_token_find(&self, username: String, token: String) -> io::Result<(users::Model,tokens::Model)> {
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find user"))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "User not found"))?;
        let token = tokens::Entity::find()
            .filter(tokens::Column::UserId.eq(user.uid))
            .filter(tokens::Column::Token.eq(token))
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to find token"))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Token not found"))?;
        Ok((user, token))
    }
}