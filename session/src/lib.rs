pub mod config;
mod middleware;
mod session;
mod session_ext;
pub mod storage;

pub use self::{
    middleware::SessionMiddleware,
    session::{Session, SessionGetError, SessionInsertError, SessionStatus},
    session_ext::SessionExt,
};
use crate::storage::interface::SessionState;
use crate::storage::{
    LoadError, SaveError, SessionKey, SessionStore, UpdateError, generate_session_key,
};
use anyhow::Error;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use database::entity::users::Model;
use redis::{Cmd, FromRedisValue, Value};
use sea_orm::prelude::Uuid;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use time::Duration;

pub const USER_KET: &str = "user_session";
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserSession {
    pub user_uid: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl From<Model> for UserSession {
    fn from(value: Model) -> Self {
        Self {
            user_uid: value.uid,
            username: value.username,
            email: value.email,
            display_name: value.display_name,
            avatar_url: value.avatar_url,
        }
    }
}

#[derive(Clone)]
struct CacheConfiguration {
    cache_keygen: Arc<dyn Fn(&str) -> String + Send + Sync>,
}

impl Debug for CacheConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheConfiguration")
            .field("cache_keygen", &"<function>")
            .finish()
    }
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        Self {
            cache_keygen: Arc::new(str::to_owned),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SessionStorage {
    db: DatabaseConnection,
    redis: Pool<RedisConnectionManager>,
    configuration: CacheConfiguration,
}

unsafe impl Send for SessionStorage {}
unsafe impl Sync for SessionStorage {}

impl SessionStorage {
    pub fn new(db: DatabaseConnection, redis: Pool<RedisConnectionManager>) -> Self {
        Self {
            db,
            redis,
            configuration: CacheConfiguration::default(),
        }
    }
}

impl SessionStorage {
    async fn execute_command<T: FromRedisValue>(&self, cmd: &mut Cmd) -> anyhow::Result<T> {
        let mut can_retry = true;
        let mut conn = self.redis.get().await?;
        loop {
            match conn.send_packed_command(cmd).await {
                Ok(value) => {
                    return Ok(T::from_redis_value(&value).map_err(|err| anyhow::anyhow!(err))?);
                }
                Err(err) => {
                    if can_retry && err.is_connection_dropped() {
                        tracing::debug!(
                            "Connection dropped while trying to talk to Redis. Retrying."
                        );
                        can_retry = false;
                        continue;
                    } else {
                        return Err(err.into());
                    }
                }
            }
        }
    }
}

impl SessionStore for SessionStorage {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        let mut value: Option<String> = self
            .execute_command(redis::cmd("GET").arg(&[&cache_key]))
            .await
            .map_err(LoadError::Other)?;
        if let None = value {
            if let Ok(Some(db_session)) = database::entity::user_session::Entity::find()
                .filter(
                    database::entity::user_session::Column::SessionToken.eq(session_key.as_ref()),
                )
                .one(&self.db)
                .await
            {
                value = Some(db_session.value.to_string());
                let ttl = db_session
                    .expires_at
                    .signed_duration_since(chrono::Utc::now().naive_utc());
                if ttl.num_seconds() > 0 {
                    self.execute_command::<()>(
                        redis::cmd("SET")
                            .arg(&[&cache_key, &db_session.value, "EX"])
                            .arg(ttl.num_seconds()),
                    )
                    .await
                    .ok();
                }
            }
        }

        match value {
            None => Ok(None),
            Some(value) => Ok(serde_json::from_str(&value)
                .map_err(Into::into)
                .map_err(LoadError::Deserialization)?),
        }
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, SaveError> {
        let body = serde_json::to_string(&session_state)
            .map_err(Into::into)
            .map_err(SaveError::Serialization)?;
        let session_key = generate_session_key();
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        self.execute_command::<()>(redis::cmd("SET").arg(&[&cache_key, &body, "EX"]).arg(
            ttl.whole_seconds(), // EXpiry in seconds
        ))
        .await
        .map_err(SaveError::Other)?;

        if let Ok(Some(user_session)) = database::entity::user_session::Entity::find()
            .filter(database::entity::user_session::Column::SessionToken.eq(session_key.as_ref()))
            .one(&self.db)
            .await
            .map_err(|e| SaveError::Other(e.into()))
        {
            user_session
                .into_active_model()
                .delete(&self.db)
                .await
                .map_err(|e| SaveError::Other(e.into()))?;
        };

        let mut user_session = database::entity::user_session::ActiveModel::new();
        user_session.value = Set(body);
        user_session.expires_at =
            Set(chrono::Utc::now().naive_utc() + chrono::Duration::seconds(ttl.whole_seconds()));
        user_session.session_token = Set(session_key.as_ref().to_string());
        user_session
            .insert(&self.db)
            .await
            .map_err(|e| SaveError::Other(e.into()))?;

        Ok(session_key)
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        let body = serde_json::to_string(&session_state)
            .map_err(Into::into)
            .map_err(UpdateError::Serialization)?;

        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        let v: Value = self
            .execute_command(redis::cmd("SET").arg(&[
                &cache_key,
                &body,
                "XX", // XX: Only set the key if it already exist.
                "EX", // EX: set expiry
                &format!("{}", ttl.whole_seconds()),
            ]))
            .await
            .map_err(UpdateError::Other)?;

        // 更新数据库中的会话数据
        match database::entity::user_session::Entity::find()
            .filter(database::entity::user_session::Column::SessionToken.eq(session_key.as_ref()))
            .one(&self.db)
            .await
            .map_err(|e| UpdateError::Other(e.into()))?
        {
            Some(existing_session) => {
                let mut user_session = existing_session.into_active_model();
                user_session.value = Set(body.clone());
                user_session.expires_at =
                    Set(chrono::Utc::now().naive_utc()
                        + chrono::Duration::seconds(ttl.whole_seconds()));
                user_session
                    .update(&self.db)
                    .await
                    .map_err(|e| UpdateError::Other(e.into()))?;
            }
            None => {
                let mut user_session = database::entity::user_session::ActiveModel::new();
                user_session.value = Set(body.clone());
                user_session.expires_at =
                    Set(chrono::Utc::now().naive_utc()
                        + chrono::Duration::seconds(ttl.whole_seconds()));
                user_session.session_token = Set(session_key.as_ref().to_string());
                user_session
                    .insert(&self.db)
                    .await
                    .map_err(|e| UpdateError::Other(e.into()))?;
            }
        }

        match v {
            Value::Nil => Err(UpdateError::Other(anyhow::anyhow!(
                "Session not found in Redis"
            ))),
            Value::Int(_) | Value::Okay | Value::SimpleString(_) => Ok(session_key),
            val => Err(UpdateError::Other(anyhow::anyhow!(
                "Failed to update session state. {:?}",
                val
            ))),
        }
    }

    async fn update_ttl(&self, session_key: &SessionKey, ttl: &Duration) -> anyhow::Result<()> {
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());
        self.execute_command::<()>(
            redis::cmd("EXPIRE").arg(&[&cache_key, &format!("{}", ttl.whole_seconds())]),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update Redis TTL: {:?}", e))?;

        if let Ok(Some(user_session)) = database::entity::user_session::Entity::find()
            .filter(database::entity::user_session::Column::SessionToken.eq(session_key.as_ref()))
            .one(&self.db)
            .await
        {
            let mut user_session = user_session.into_active_model();
            user_session.expires_at = Set(
                chrono::Utc::now().naive_utc() + chrono::Duration::seconds(ttl.whole_seconds())
            );
            user_session.update(&self.db).await?;
        }

        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), Error> {
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        self.execute_command::<()>(redis::cmd("DEL").arg(&[&cache_key]))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to delete from Redis: {:?}", e))?;

        if let Ok(Some(user_session)) = database::entity::user_session::Entity::find()
            .filter(database::entity::user_session::Column::SessionToken.eq(session_key.as_ref()))
            .one(&self.db)
            .await
        {
            let user_session = user_session.into_active_model();
            user_session.delete(&self.db).await?;
        }
        Ok(())
    }
}
