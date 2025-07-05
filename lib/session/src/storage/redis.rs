use crate::redis;
use crate::redis::aio::ConnectionLike;
use crate::redis::{Cmd, FromRedisValue};
use crate::storage::SessionStorage;
use async_trait::async_trait;
use dashmap::DashMap;
use std::collections::HashMap;
use time::Duration;

#[derive(Clone)]
pub enum RedisStorage {
    Signal(deadpool_redis::Pool),
    Cluster(deadpool_redis::cluster::Pool),
    Sentinel(deadpool_redis::sentinel::Pool),
}

impl RedisStorage {
    pub fn new_signal(pool: deadpool_redis::Pool) -> Self {
        RedisStorage::Signal(pool)
    }
    pub fn new_cluster(pool: deadpool_redis::cluster::Pool) -> Self {
        RedisStorage::Cluster(pool)
    }
    pub fn new_sentinel(pool: deadpool_redis::sentinel::Pool) -> Self {
        RedisStorage::Sentinel(pool)
    }
    pub async fn conn(&self) -> anyhow::Result<Box<dyn ConnectionLike>> {
        match self {
            RedisStorage::Signal(pool) => Ok(Box::new(pool.get().await?)),
            RedisStorage::Cluster(pool) => Ok(Box::new(pool.get().await?)),
            RedisStorage::Sentinel(pool) => Ok(Box::new(pool.get().await?)),
        }
    }
    pub async fn execute_command<T: FromRedisValue>(&self, cmd: Cmd) -> anyhow::Result<T> {
        let mut can_retry = true;
        match self {
            RedisStorage::Signal(pool) => {
                let mut conn = pool.get().await?;
                loop {
                    match cmd.query_async(&mut conn).await {
                        Ok(value) => return Ok(value),
                        Err(err) => {
                            if can_retry && err.is_connection_dropped() {
                                can_retry = false;
                                continue;
                            } else {
                                return Err(err.into());
                            }
                        }
                    }
                }
            }
            RedisStorage::Cluster(pool) => {
                let mut conn = pool.get().await?;
                loop {
                    match cmd.query_async(&mut conn).await {
                        Ok(value) => return Ok(value),
                        Err(err) => {
                            if can_retry && err.is_connection_dropped() {
                                can_retry = false;
                                continue;
                            } else {
                                return Err(err.into());
                            }
                        }
                    }
                }
            }
            RedisStorage::Sentinel(pool) => {
                let mut conn = pool.get().await?;
                loop {
                    match cmd.query_async(&mut conn).await {
                        Ok(value) => return Ok(value),
                        Err(err) => {
                            if can_retry && err.is_connection_dropped() {
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
    }
}
#[async_trait]
impl SessionStorage for RedisStorage {
    async fn load(&self, id: &str) -> anyhow::Result<DashMap<String, String>> {
        let cmd = redis::cmd("GET").arg(id).clone();
        let value = self.execute_command::<String>(cmd).await?;
        let value = serde_json::from_str::<HashMap<String, String>>(&value)
            .map_err(|err| anyhow::anyhow!(err))?;
        let map = value
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<DashMap<String, String>>();
        Ok(map)
    }

    async fn save(
        &self,
        session: &DashMap<String, String>,
        ttl: &Duration,
    ) -> anyhow::Result<String> {
        let map = session
            .iter()
            .map(|x| (x.key().clone(), x.value().clone()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<String, String>>();
        let map = serde_json::to_string(&map)?;
        let key = format!(
            "session:{}+{}",
            uuid::Uuid::new_v4(),
            uuid::Uuid::now_v7().to_string()
        );
        self.execute_command::<()>(
            redis::cmd("SET")
                .arg(&[&key, &map, "NX", "EX"])
                .arg(ttl.whole_seconds())
                .clone(),
        )
        .await?;
        Ok(key)
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.execute_command::<()>(redis::cmd("DEL").arg(&[&id]).clone())
            .await?;
        Ok(())
    }

    async fn update(
        &self,
        key: &str,
        session: &DashMap<String, String>,
        ttl: &Duration,
    ) -> anyhow::Result<String> {
        self.delete(key).await?;
        self.save(session, ttl).await
    }

    async fn update_ttl(&self, id: &str, ttl: Duration) -> anyhow::Result<()> {
        let sec = (ttl.as_seconds_f32() as i64).to_string();
        self.execute_command::<()>(redis::cmd("EXPIRE").arg(&[&id.to_string(), &sec]).clone())
            .await?;
        Ok(())
    }
}
