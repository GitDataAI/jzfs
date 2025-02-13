use lib_entity::ActiveModelTrait;
use lib_entity::ActiveValue::Set;
use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::ModelTrait;
use lib_entity::QueryFilter;
use lib_entity::prelude::Uuid;
use lib_entity::sqlx::types::chrono::Utc;
use lib_entity::users::token_key;
use serde::Deserialize;
use serde::Serialize;
use sha256::Sha256Digest;

use crate::server::AppUserState;

#[derive(Deserialize, Clone)]
pub struct UserCreateToken {
    pub name : String,
    pub access : i64,
    pub expire : i64,
}
#[derive(Serialize, Clone)]
pub struct UserCreateTokenResponse {
    pub token : String,
}

#[derive(Deserialize, Clone)]
pub struct UserTokenDelete {
    pub name : String,
    pub uid : Uuid,
}

impl AppUserState {
    pub async fn create_token(
        &self,
        user_id : Uuid,
        token : UserCreateToken,
    ) -> std::io::Result<UserCreateTokenResponse> {
        let tokens = format!(
            "grt_{}",
            format!(
                "{}-{}-{}",
                sha256::digest(user_id.to_string()),
                sha256::digest(token.name.clone()),
                sha256::digest(token.access.to_string())
            )
            .digest()
        );
        let token_key = token_key::ActiveModel {
            uid : Set(Uuid::new_v4()),
            user_id : Set(user_id),
            content : Set(tokens.clone()),
            name : Set(token.name),
            access : Set(token.access),
            created : Set(Utc::now().timestamp()),
            updated : Set(Utc::now().timestamp()),
            expire : Set(token.expire),
            hasused : Set(0),
        };
        match token_key.insert(&self.write).await {
            Ok(_) => Ok(UserCreateTokenResponse { token : tokens }),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            )),
        }
    }
    pub async fn delete_token(
        &self,
        user_id : Uuid,
        token : UserTokenDelete,
    ) -> std::io::Result<()> {
        let token_key = token_key::Entity::find_by_id(token.uid)
            .one(&self.read)
            .await
            .unwrap();
        match token_key {
            Some(token_key) => {
                if token_key.user_id == user_id {
                    match token_key.delete(&self.write).await {
                        Ok(_) => Ok(()),
                        Err(err) => Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            err.to_string(),
                        )),
                    }
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "token not found".to_string(),
                    ))
                }
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "token not found".to_string(),
            )),
        }
    }

    pub async fn get_token(
        &self,
        user_id : Uuid,
        token : String,
    ) -> std::io::Result<token_key::Model> {
        let token_key = token_key::Entity::find()
            .filter(token_key::Column::UserId.eq(user_id))
            .filter(token_key::Column::Content.eq(token))
            .one(&self.read)
            .await
            .unwrap();
        match token_key {
            Some(token_key) => Ok(token_key),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "token not found".to_string(),
            )),
        }
    }
    pub async fn token_list(&self, user_id : Uuid) -> std::io::Result<Vec<token_key::Model>> {
        let token_key = token_key::Entity::find()
            .filter(token_key::Column::UserId.eq(user_id))
            .all(&self.read)
            .await
            .unwrap();
        Ok(token_key)
    }
}
