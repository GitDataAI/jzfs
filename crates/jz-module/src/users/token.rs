use crate::AppModule;
use chrono::Utc;
use jz_model::{DeleteOption, token, uuid_v4, uuid_v7};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenCreate {
    name: String,
    description: Option<String>,
    expire: i64,
    access: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenCreateResponse {
    uid: Uuid,
    token: String,
    expire: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenDelete {
    uid: Uuid,
    name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum TokenAccess {
    Read = 1,
    Write = 2,
    Admin = 3,
    All = 4,
}
impl TokenAccess {
    pub fn to_i64(&self) -> i64 {
        match self {
            TokenAccess::Read => 1,
            TokenAccess::Write => 2,
            TokenAccess::Admin => 3,
            TokenAccess::All => 4,
        }
    }
    pub fn from_i64(i: i64) -> TokenAccess {
        match i {
            1 => TokenAccess::Read,
            2 => TokenAccess::Write,
            3 => TokenAccess::Admin,
            4 => TokenAccess::All,
            _ => TokenAccess::Read,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            TokenAccess::Read => "read".to_string(),
            TokenAccess::Write => "write".to_string(),
            TokenAccess::Admin => "admin".to_string(),
            TokenAccess::All => "all".to_string(),
        }
    }
    pub fn from_string(s: String) -> TokenAccess {
        match s.to_lowercase().as_str() {
            "read" => TokenAccess::Read,
            "write" => TokenAccess::Write,
            "admin" => TokenAccess::Admin,
            "all" => TokenAccess::All,
            _ => TokenAccess::Read,
        }
    }
}

impl AppModule {
    pub async fn token_create(
        &self,
        users_uid: Uuid,
        param: TokenCreate,
    ) -> anyhow::Result<TokenCreateResponse> {
        let token = format!("{}-{}-{}-{}", uuid_v4(), uuid_v7(), users_uid, Utc::now()).digest();
        let fingerprint = format!("{}-{}-{}-{}", uuid_v4(), uuid_v7(), users_uid, token);
        let now = Utc::now();
        let expire = if param.expire == 0 {
            now + chrono::Duration::days(365 * 10)
        } else {
            now + chrono::Duration::days(param.expire)
        };
        let active = token::ActiveModel::new(
            users_uid,
            param.name,
            fingerprint,
            param.description,
            token,
            match param.access {
                1 => TokenAccess::Read,
                2 => TokenAccess::Write,
                3 => TokenAccess::Admin,
                4 => TokenAccess::All,
                _ => TokenAccess::Read,
            }
            .to_string(),
            expire.naive_local(),
        );
        match active.clone().insert(&self.write).await {
            Ok(_) => Ok(TokenCreateResponse {
                uid: active.uid.unwrap(),
                token: active.token.unwrap(),
                expire: expire.timestamp(),
            }),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
    pub async fn token_delete(
        &self,
        users_uid: Uuid,
        param: TokenDelete,
    ) -> anyhow::Result<DeleteOption> {
        let active = token::Entity::delete_many()
            .filter(token::Column::UserId.eq(users_uid))
            .filter(token::Column::Name.eq(param.name))
            .exec(&self.write)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(DeleteOption::from(active))
    }
    pub async fn token_list(&self, users_uid: Uuid) -> anyhow::Result<Vec<token::Model>> {
        Ok(token::Entity::find()
            .filter(token::Column::UserId.eq(users_uid))
            .all(&self.read)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?)
    }
}
