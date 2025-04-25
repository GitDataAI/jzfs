use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::users::UsersModel;
use crate::uuid_v7;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct SecretsModel {
    #[sqlx(primary_key)]
    pub uid: Uuid,

    pub title: String,
    pub description: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub device: Option<String>,
    pub location: Option<String>,

    pub action: String,
    pub actor: String,
    pub actor_uid: Uuid,

    pub user: String,
    pub user_uid: Uuid,
    pub timestamp: chrono::NaiveDateTime,
}

pub struct SecretsMapper {
    pub db: sqlx::PgPool,
}

impl SecretsMapper {
    pub async fn insert(&self, secrets: SecretsModel) -> Result<SecretsModel, sqlx::Error> {
        sqlx::query_as::<_, SecretsModel>(
            "INSERT INTO secrets (uid, title, description, ip, user_agent, device, location, action, actor, actor_uid, user, user_uid, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) RETURNING *",
        )
        .bind(secrets.uid)
        .bind(secrets.title)
        .bind(secrets.description)
        .bind(secrets.ip)
        .bind(secrets.user_agent)
        .bind(secrets.device)
        .bind(secrets.location)
        .bind(secrets.action)
        .bind(secrets.actor)
        .bind(secrets.actor_uid)
        .bind(secrets.user)
        .bind(secrets.user_uid)
        .bind(secrets.timestamp)
        .fetch_one(&self.db)
        .await
    }
    pub fn query(&self) -> SecretsQuery {
        SecretsQuery {
            db: self.db.clone(),
        }
    }
    pub fn relation(&self, secrets: SecretsModel) -> SecretsRelation {
        SecretsRelation {
            db: self.db.clone(),
            secrets,
        }
    }
    pub fn action(&self) -> SecretsAction {
        SecretsAction {
            db: self.db.clone(),
        }
    }
}



pub struct SecretsQuery {
    pub db: sqlx::PgPool,
}

impl SecretsQuery {
    pub async fn query_by_uid(&self, uid: Uuid) -> Result<SecretsModel, sqlx::Error> {
        sqlx::query_as::<_, SecretsModel>(
            "SELECT * FROM secrets WHERE uid = $1",
        )
        .bind(uid)
        .fetch_one(&self.db)
        .await
    }
    pub async fn query_by_user_uid(&self, user_uid: Uuid) -> Result<Vec<SecretsModel>, sqlx::Error> {
        sqlx::query_as::<_, SecretsModel>(
            "SELECT * FROM secrets WHERE user_uid = $1",
        )
        .bind(user_uid)
        .fetch_all(&self.db)
        .await
    }
    pub async fn query_by_actor_uid(&self, actor_uid: Uuid) -> Result<Vec<SecretsModel>, sqlx::Error> {
        sqlx::query_as::<_, SecretsModel>(
            "SELECT * FROM secrets WHERE actor_uid = $1",
        )
        .bind(actor_uid)
        .fetch_all(&self.db)
        .await
    }
    pub async fn query_by_action(&self, action: String) -> Result<Vec<SecretsModel>, sqlx::Error> {
        sqlx::query_as::<_, SecretsModel>(
            "SELECT * FROM secrets WHERE action = $1",
        )
        .bind(action)
        .fetch_all(&self.db)
        .await
    }
}

pub struct SecretsRelation {
    pub db: sqlx::PgPool,
    pub secrets: SecretsModel,
}

impl SecretsRelation {
    pub async fn user(&self) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>(
            "SELECT * FROM users WHERE uid = $1",
        )
        .bind(self.secrets.user_uid)
        .fetch_one(&self.db)
        .await
    }
    pub async fn actor(&self) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>(
            "SELECT * FROM users WHERE uid = $1",
        )
        .bind(self.secrets.actor_uid)
        .fetch_one(&self.db)
        .await
    }
    pub async fn delete(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM secrets WHERE uid = $1",
        )
        .bind(self.secrets.uid)
        .execute(&self.db)
        .await?;
        Ok(())
    }
}


pub struct SecretsAction {
    pub db: sqlx::PgPool,
}



impl SecretsAction {
    fn mapper(&self) -> SecretsMapper {
        SecretsMapper {
            db: self.db.clone(),
        }
    }
    pub async fn login(
        &self,
        user: UsersModel, 
        ip: Option<String>, 
        user_agent: Option<String>,
        device: Option<String>,
        location: Option<String>,
        actor: UsersModel,
    ) -> Result<SecretsModel, sqlx::Error> {
        let secrets = SecretsModel {
            uid: uuid_v7(),
            title: "login".to_string(),
            description: "login".to_string(),
            ip,
            user_agent,
            device,
            location,
            action: "Login".to_string(),
            actor: actor.username,
            actor_uid: actor.uid,
            user: user.username,
            user_uid: user.uid,
            timestamp: chrono::Utc::now().naive_local(),
        };
        let secrets = self.mapper().insert(secrets).await?;
        Ok(secrets)
    }
}