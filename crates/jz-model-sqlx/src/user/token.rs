use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::users::UsersModel;
use crate::uuid_v7;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct TokenModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub fingerprint: String,
    pub description: Option<String>,
    #[serde(skip)]
    pub token: String,
    pub access: String,
    pub use_history: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct TokenList {
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub fingerprint: String,
    pub description: Option<String>,
    pub access: String,
    pub use_history: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl From<TokenModel> for TokenList {
    fn from(token: TokenModel) -> Self {
        Self {
            uid: token.uid,
            user_id: token.user_id,
            name: token.name,
            fingerprint: token.fingerprint,
            description: token.description,
            access: token.access,
            use_history: token.use_history,
            created_at: token.created_at,
            updated_at: token.updated_at,
            expires_at: token.expires_at,
        }
    }
}

pub struct TokenMapper {
    pub db: sqlx::PgPool
}
impl TokenMapper {
    pub async fn insert(&self, token: TokenModel) -> Result<TokenModel, sqlx::Error> {
        sqlx::query_as::<_, TokenModel>(
            "INSERT INTO tokens (uid, user_id, name, fingerprint, description, token, access, use_history, created_at, updated_at, expires_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *"
        )
            .bind(token.uid)
            .bind(token.user_id)
            .bind(token.name)
            .bind(token.fingerprint)
            .bind(token.description)
            .bind(token.token)
            .bind(token.access)
            .bind(token.use_history)
            .bind(token.created_at)
            .bind(token.updated_at)
            .bind(token.expires_at)
            .fetch_one(&self.db)
            .await
    }
    pub fn query(&self) -> TokenQuery {
        TokenQuery {
            db: self.db.clone(),
        }
    }
    pub fn update(&self, token: TokenModel) -> TokenUpdate {
        TokenUpdate {
            db: self.db.clone(),
            token,
        }
    }
    pub fn relation(&self, token: TokenModel) -> TokenRelation {
        TokenRelation {
            db: self.db.clone(),
            token,
        }
    }
}

pub struct TokenQuery {
    pub db: sqlx::PgPool,
}

impl TokenQuery {
    pub async fn query_by_uid(&self, uid: Uuid) -> Result<TokenModel, sqlx::Error> {
        sqlx::query_as::<_, TokenModel>("SELECT * FROM tokens WHERE uid = $1")
            .bind(uid)
            .fetch_one(&self.db)
            .await
    }
    pub async fn query_by_fingerprint(&self, fingerprint: String) -> Result<TokenModel, sqlx::Error> {
        sqlx::query_as::<_, TokenModel>("SELECT * FROM tokens WHERE fingerprint = $1")
            .bind(fingerprint)
            .fetch_one(&self.db)
            .await
    }
    pub async fn query_by_user_id(&self, user_id: Uuid) -> Result<Vec<TokenModel>, sqlx::Error> {
        sqlx::query_as::<_, TokenModel>("SELECT * FROM tokens WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.db)
            .await
    }
    pub async fn query_by_token(&self, token: String) -> Result<TokenModel, sqlx::Error> {
        sqlx::query_as::<_, TokenModel>("SELECT * FROM tokens WHERE token = $1")
            .bind(token)
            .fetch_one(&self.db)
            .await
    }
}

pub struct TokenUpdate {
    pub db: sqlx::PgPool,
    pub token: TokenModel,
}

impl TokenUpdate {
    pub async fn update_description(&self, description: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tokens SET description = $1 WHERE uid = $2")
            .bind(description)
            .bind(self.token.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_expires_at(&self, expires_at: DateTime<Utc>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tokens SET expires_at = $1 WHERE uid = $2")
            .bind(expires_at)
            .bind(self.token.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_use_history(&self, use_history: Vec<String>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tokens SET use_history = $1 WHERE uid = $2")
            .bind(use_history)
            .bind(self.token.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_access(&self, access: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tokens SET access = $1 WHERE uid = $2")
            .bind(access)
            .bind(self.token.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_level(&self, level: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tokens SET level = $1 WHERE uid = $2")
            .bind(level)
            .bind(self.token.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

pub struct TokenRelation {
    pub db: sqlx::PgPool,
    pub token: TokenModel,
}

impl TokenRelation {
    pub async fn delete(&self) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tokens WHERE uid = $1")
            .bind(self.token.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn users(&self) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>("SELECT * FROM users WHERE uid = $1")
            .bind(self.token.user_id)
            .fetch_one(&self.db)
            .await
    }
}


pub struct TokenBuilder {
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub access: Option<String>,
    pub fingerprint: Option<String>,
    pub token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl TokenModel {
    pub fn builder() -> TokenBuilder {
        TokenBuilder::new()
    }
}

impl TokenBuilder {
    pub fn new() -> Self {
        Self {
            user_id: None,
            name: None,
            description: None,
            access: None,
            fingerprint: None,
            token: None,
            expires_at: None,
        }
    }
    pub fn user_id(&mut self, user_id: Uuid) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn description(&mut self, description: Option<String>) -> &mut Self {
        self.description = description;
        self
    }
    pub fn access(&mut self, access: String) -> &mut Self {
        self.access = Some(access);
        self
    }
    pub fn expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn fingerprint(&mut self, fingerprint: String) -> &mut Self {
        self.fingerprint = Some(fingerprint);
        self
    }
    pub fn token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);
        self
    }
    pub fn build(&self) -> anyhow::Result<TokenModel> {
        let uid = uuid_v7();
        let Some(user_id) = self.user_id else {
            return Err(anyhow::anyhow!("user_id is required"));
        };
        let Some(name) = self.name.clone() else {
            return Err(anyhow::anyhow!("name is required"));
        };
        let Some(access) = self.access.clone() else {
            return Err(anyhow::anyhow!("access is required"));
        };
        let Some(fingerprint) = self.fingerprint.clone() else {
            return Err(anyhow::anyhow!("fingerprint is required"));
        };
        let Some(expires_at) = self.expires_at else {
            return Err(anyhow::anyhow!("expires_at is required"));
        };
        let Some(token) = self.token.clone() else {
            return Err(anyhow::anyhow!("token is required"));
        };
        Ok(TokenModel {
            uid,
            user_id,
            name,
            fingerprint,
            description: self.description.clone(),
            token,
            access,
            expires_at,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            use_history: vec![],
        })
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum UsersTokenAccess {
    Read = 1,
    Write = 2,
    Admin = 3,
    All = 4,
}
impl UsersTokenAccess {
    pub fn to_i64(&self) -> i64 {
        match self {
            UsersTokenAccess::Read => 1,
            UsersTokenAccess::Write => 2,
            UsersTokenAccess::Admin => 3,
            UsersTokenAccess::All => 4,
        }
    }
    pub fn from_i64(i: i64) -> UsersTokenAccess {
        match i {
            1 => UsersTokenAccess::Read,
            2 => UsersTokenAccess::Write,
            3 => UsersTokenAccess::Admin,
            4 => UsersTokenAccess::All,
            _ => UsersTokenAccess::Read,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            UsersTokenAccess::Read => "read".to_string(),
            UsersTokenAccess::Write => "write".to_string(),
            UsersTokenAccess::Admin => "admin".to_string(),
            UsersTokenAccess::All => "all".to_string(),
        }
    }
    pub fn from_string(s: String) -> UsersTokenAccess {
        match s.to_lowercase().as_str() {
            "read" => UsersTokenAccess::Read,
            "write" => UsersTokenAccess::Write,
            "admin" => UsersTokenAccess::Admin,
            "all" => UsersTokenAccess::All,
            _ => UsersTokenAccess::Read,
        }
    }
}
