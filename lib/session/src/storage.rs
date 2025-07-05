use async_trait::async_trait;
use dashmap::DashMap;
use time::Duration;

pub mod redis;
pub use redis::*;

#[async_trait]
pub trait SessionStorage {
    async fn load(&self, id: &str) -> anyhow::Result<DashMap<String, String>>;
    async fn save(
        &self,
        session: &DashMap<String, String>,
        ttl: &Duration,
    ) -> anyhow::Result<String>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
    async fn update(
        &self,
        key: &str,
        session: &DashMap<String, String>,
        ttl: &Duration,
    ) -> anyhow::Result<String>;
    async fn update_ttl(&self, id: &str, ttl: Duration) -> anyhow::Result<()>;
}
