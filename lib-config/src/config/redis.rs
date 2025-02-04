use crate::config::AppConfig;
use deadpool_redis::cluster::{ClusterClient, ClusterConnection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use actix_session::storage::{LoadError, SaveError, SessionKey, UpdateError};
use deadpool_redis::redis::AsyncCommands;
use log::info;

#[derive(Deserialize,Serialize,Debug)]
pub struct RedisConfig {
    pub host: String,
    pub port: i32,
    pub password: String,
    pub max_connection: i32,
    pub min_connection: i32,
    pub db: i32,
    pub kind: RedisConfigKind,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
#[derive(PartialEq)]
pub enum RedisConfigKind {
    Session,
    Cache,
}



impl RedisConfig {
    pub fn from_json(json: &str) -> std::io::Result<RedisConfig> {
        let config: RedisConfig = serde_json::from_str(json)?;
        Ok(config)
    }
    pub fn to_json(&self) -> std::io::Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
    pub fn to_string(&self) -> String {
        if self.password.is_empty() {
            format!("redis://{}:{}", self.host, self.port)
        } else {
            format!("redis://:{}@{}:{}", self.password, self.host, self.port)
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            password: "".to_string(),
            max_connection: 10,
            min_connection: 1,
            db: 0,
            kind: RedisConfigKind::Session,
        }
    }
}

impl Clone for RedisConfig {
    fn clone(&self) -> Self {
        RedisConfig {
            host: self.host.clone(),
            port: self.port,
            password: self.password.clone(),
            max_connection: self.max_connection,
            min_connection: self.min_connection,
            db: self.db,
            kind: self.kind.clone(),
        }
    }
}

impl AppConfig {
    pub async fn redis_config(&self, kind: RedisConfigKind) -> std::io::Result<Vec<RedisConfig>> {
        let mut result = Vec::new();
        let mut idx = 0;
        loop {
            let data_id = format!("redis.{}", idx);
            idx += 1;
            if let Ok(data) = self.client.get_config(data_id, "redis".to_string()).await {
                if let Ok(data) = serde_json::from_str::<RedisConfig>(data.content()) {
                    if data.kind != kind {
                        continue;
                    } else {
                        result.push(data);
                    }
                }
            } else {
                break;
            }
        }
        Ok(result)
    }
    pub async fn redis_cluster(&self, kind: RedisConfigKind) -> std::io::Result<ClusterMaster> {
        let configs:Vec<RedisConfig> = self.redis_config(kind).await?;
        let mut result = Vec::new();
        for idx in configs {
            result.push(idx.to_string());
            info!("redis connect:{}:{}", idx.host,idx.port);
        }
        let client_builder = ClusterClient::builder(result);
        let client = client_builder
            .retries(3)
            .build()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(ClusterMaster {
            client: Arc::new(Mutex::new(client.get_async_connection().await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?))
        })
    }
}
#[derive(Clone)]
pub struct ClusterMaster {
    pub client: Arc<Mutex<ClusterConnection>>,
}

impl actix_session::storage::SessionStore for ClusterMaster {
    fn load(&self, session_key: &SessionKey) -> impl Future<Output = Result<Option<actix_session::storage::interface::SessionState>, LoadError>> {
        let mut client = self.client.lock().unwrap();
        let key = session_key.as_ref().to_string();
        async move {
            let data = client.get::<String,String>(key)
                .await;
            match data {
                Ok(bytes) => {
                    let state = serde_json::from_slice((&bytes).as_ref())
                        .map_err(|e| LoadError::Other(e.into()))?;
                    Ok(Some(state))
                }
                Err(_) => Ok(None),
            }
        }
    }

    fn save(&self, session_state: actix_session::storage::interface::SessionState, ttl: &time::Duration) -> impl Future<Output = Result<SessionKey, SaveError>> {
        let mut client = self.client.lock().unwrap();
        let ttl = *ttl;
        async move {
            let session_key = SessionKey(uuid::Uuid::new_v4().to_string());
            let key = session_key.as_ref().to_string();
            let bytes = serde_json::to_string(&session_state)
                .map_err(|e| SaveError::Serialization(e.into()))?;

            client.set::<String,String,String>(key.clone(), bytes)
                .await
                .map_err(|e| SaveError::Other(e.into()))?;
            client.expire::<String,bool>(key, ttl.whole_seconds())
                .await
                .map_err(|e| SaveError::Other(e.into()))?;
            Ok(session_key)
        }
    }

    fn update(&self, session_key: SessionKey, session_state: actix_session::storage::interface::SessionState, ttl: &time::Duration) -> impl Future<Output = Result<SessionKey, UpdateError>> {
        let mut client = self.client.lock().unwrap();

        let key = session_key.as_ref().to_string();
        let ttl = *ttl;

        async move {
            let exists = client.exists::<String,bool>(key.clone())
                .await
                .map_err(|e| UpdateError::Other(e.into()))?;

            if !exists {
                return Err(UpdateError::Serialization(anyhow::anyhow!("Session not found")));
            }

            let bytes = serde_json::to_string(&session_state)
                .map_err(|e| UpdateError::Serialization(e.into()))?;

            client.set::<String,String,String>(key.clone(), bytes)
                .await
                .map_err(|e| UpdateError::Other(e.into()))?;
            client.expire::<String,bool>(key, ttl.whole_seconds())
                .await
                .map_err(|e| UpdateError::Other(e.into()))?;
            Ok(session_key)
        }
    }

    fn update_ttl(&self, session_key: &SessionKey, ttl: &time::Duration) -> impl Future<Output = Result<(), anyhow::Error>> {
        let mut client = self.client.lock().unwrap();

        let key = session_key.as_ref().to_string();
        let ttl = *ttl;

        async move {
            client.expire::<String,bool>(key, ttl.whole_seconds())
                .await?;
            Ok(())
        }
    }

    fn delete(&self, session_key: &SessionKey) -> impl Future<Output = Result<(), anyhow::Error>> {
        let mut client = self.client.lock().unwrap();
        let key = session_key.as_ref().to_string();

        async move {
            client.del::<String,String>(key)
                .await?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deadpool_redis::redis::aio::ConnectionLike;

    #[test]
    fn test_redis_config() {
        let config = RedisConfig::default();
        println!("{:?}", config);
        println!("{}", config.to_string());
        println!("{}", config.to_json().unwrap())
    }
    #[test]
    fn test_redis_config_from_json() {
        let json = r#"{"host":"127.0.0.1","port":6379,"password":"","max_connection":10,"min_connection":1,"db":0,"kind":"Session"}"#;
        let config = RedisConfig::from_json(json).unwrap();
        println!("{:?}", config);
        println!("{}", config.to_string());
        println!("{}", config.to_json().unwrap())
    }
    #[tokio::test]
    async fn test_redis_get() {
        use crate::AppNacos;
        let nacos = AppNacos::from_env().unwrap();
        let config = nacos.config;
        let result = config.redis_config(RedisConfigKind::Session).await;
        if let Ok(result) = result {
            for item in result {
                println!("{:?}", item);
                println!("{}", item.to_string());
                println!("{}", item.to_json().unwrap())
            }
        } else {
            println!("{:?}", result);
        }
    }
    #[tokio::test]
    async fn test_redis_cluster() {
        use crate::AppNacos;
        let nacos = AppNacos::from_env().unwrap();
        let config = nacos.config;
        let result = config.redis_cluster(RedisConfigKind::Session).await;
        if let Ok(result) = result {
            println!("{:?}", result.client.lock().unwrap().get_db());
        }
    }
}

