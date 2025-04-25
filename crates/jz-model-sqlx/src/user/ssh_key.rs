use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::users::UsersModel;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct SshKeyModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub fingerprint: String,

    pub description: Option<String>,
    #[serde(skip)]
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct SshKeyList {
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub fingerprint: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SshKeyModel> for SshKeyList {
    fn from(ssh_key: SshKeyModel) -> Self {
        SshKeyList {
            uid: ssh_key.uid,
            user_id: ssh_key.user_id,
            name: ssh_key.name,
            description: ssh_key.description,
            fingerprint: ssh_key.fingerprint,
            created_at: ssh_key.created_at,
            updated_at: ssh_key.updated_at,
        }
    }
}

pub struct SshKeyMapper {
    pub db: sqlx::PgPool
}

impl SshKeyMapper {
    pub async fn insert(&self, ssh_key: SshKeyModel) -> Result<SshKeyModel, sqlx::Error> {
        sqlx::query_as::<_, SshKeyModel>(
            "INSERT INTO ssh (uid, user_id, name, fingerprint, description, content, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
        )
        .bind(ssh_key.uid)
        .bind(ssh_key.user_id)
        .bind(ssh_key.name)
        .bind(ssh_key.fingerprint)
        .bind(ssh_key.description)
        .bind(ssh_key.content)
        .bind(ssh_key.created_at)
        .bind(ssh_key.updated_at)
        .fetch_one(&self.db)
        .await
    }
    pub fn query(&self) -> SshKeyQuery {
        SshKeyQuery {
            db: self.db.clone(),
        }
    }
    pub fn update(&self, ssh_key: SshKeyModel) -> SshKeyUpdate {
        SshKeyUpdate {
            db: self.db.clone(),
            ssh_key,
        }
    }
    pub fn relation(&self, ssh_key: SshKeyModel) -> SshKeyRelation {
        SshKeyRelation {
            db: self.db.clone(),
            ssh_key,
        }
    }
}

pub struct SshKeyQuery {
    pub db: sqlx::PgPool,
}

impl SshKeyQuery {
    pub async fn query_by_uid(&self, uid: Uuid) -> Result<SshKeyModel, sqlx::Error> {
        sqlx::query_as::<_, SshKeyModel>(
            "SELECT * FROM ssh WHERE uid = $1"
        )
        .bind(uid)
        .fetch_one(&self.db)
        .await
    }
    pub async fn query_by_fingerprint(&self, fingerprint: String) -> Result<SshKeyModel, sqlx::Error> {
        sqlx::query_as::<_, SshKeyModel>(
            "SELECT * FROM ssh WHERE fingerprint = $1"
        )
        .bind(fingerprint)
        .fetch_one(&self.db)
        .await
    }
    pub async fn query_by_user_id(&self, user_id: Uuid) -> Result<Vec<SshKeyModel>, sqlx::Error> {
        sqlx::query_as::<_, SshKeyModel>(
            "SELECT * FROM ssh WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await
    }
    pub async fn query_by_content(&self, content: String) -> Result<SshKeyModel, sqlx::Error> {
        sqlx::query_as::<_, SshKeyModel>(
            "SELECT * FROM ssh WHERE content = $1"
        )
        .bind(content)
        .fetch_one(&self.db)
        .await
    }
}

pub struct SshKeyUpdate {
    pub db: sqlx::PgPool,
    pub ssh_key: SshKeyModel,
}

impl SshKeyUpdate {
    pub async fn update_description(&self, description: String) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE ssh SET description = $1 WHERE uid = $2"
        )
        .bind(description)
        .bind(self.ssh_key.uid)
        .execute(&self.db)
        .await?;
        Ok(())
    }
    pub async fn update_updated_at(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE ssh SET updated_at = $1 WHERE uid = $2"
        )
        .bind(chrono::Local::now().naive_utc())
        .bind(self.ssh_key.uid)
        .execute(&self.db)
        .await?;
        Ok(())
    }
}

pub struct SshKeyRelation {
    pub db: sqlx::PgPool,
    pub ssh_key: SshKeyModel,
}

impl SshKeyRelation {
    pub async fn delete(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM ssh WHERE uid = $1"
        )
        .bind(self.ssh_key.uid)
        .execute(&self.db)
        .await?;
        Ok(())
    }
    pub async fn users(&self) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>(
            "SELECT * FROM users WHERE uid = $1"
        )
        .bind(self.ssh_key.user_id)
        .fetch_one(&self.db)
        .await
    }
}


pub struct SshKeyBuilder {
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub fingerprint: Option<String>,
}
impl SshKeyModel {
    pub fn builder() -> SshKeyBuilder {
        SshKeyBuilder {
            user_id: None,
            name: None,
            description: None,
            content: None,
            fingerprint: None,
        }
    }
}

impl SshKeyBuilder {
    pub fn new() -> Self {
        Self {
            user_id: None,
            name: None,
            description: None,
            content: None,
            fingerprint: None,
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
    pub fn content(&mut self, content: String) -> &mut Self {
        self.content = Some(content);
        self
    }
    pub fn fingerprint(&mut self, fingerprint: String) -> &mut Self {
        self.fingerprint = Some(fingerprint);
        self
    }
    pub fn build(&self) -> SshKeyModel {
        SshKeyModel {
            uid: Uuid::new_v4(),
            user_id: self.user_id.unwrap(),
            name: self.name.clone().unwrap(),
            description: self.description.clone(),
            content: self.content.clone().unwrap(),
            fingerprint: self.fingerprint.clone().unwrap(),
            created_at: chrono::Local::now().to_utc(),
            updated_at: chrono::Local::now().to_utc(),
        }
    }
}