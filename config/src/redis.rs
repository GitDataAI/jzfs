use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct AppRedisConfig {
    #[serde(rename = "url")]
    pub urls: String,
    #[serde(rename = "max_conn")]
    pub max_conn: u32,
    #[serde(rename = "min_conn")]
    pub min_conn: Option<u32>,
    #[serde(rename = "idle_timeout")]
    pub idle_timeout: u64,
    #[serde(rename = "retry")]
    pub retry: bool,
}

impl AppRedisConfig {
    pub async fn conn(&self) -> Pool<RedisConnectionManager> {
        let manager = RedisConnectionManager::new(self.urls.clone())
            .expect("Failed to create redis connection manager");
        let pool = Pool::builder()
            .max_size(self.max_conn)
            .min_idle(self.min_conn)
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .retry_connection(self.retry)
            .build(manager)
            .await
            .expect("Failed to create redis pool");
        pool
    }
}

impl Default for AppRedisConfig {
    fn default() -> Self {
        AppRedisConfig {
            urls: "redis://127.0.0.1:6379".to_string(),
            max_conn: 10,
            min_conn: Some(2),
            idle_timeout: 60,
            retry: true,
        }
    }
}
