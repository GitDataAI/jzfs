use async_trait::async_trait;

#[cfg(feature = "sql")]
pub mod seaorm;
#[cfg(feature = "sql")]
pub use seaorm::*;

#[async_trait]
pub trait Queue {
    async fn push<T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync>(
        &self,
        queue: &str,
        data: T,
    ) -> anyhow::Result<String>;
    async fn pull<T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync>(
        &self,
        queue: &str,
    ) -> anyhow::Result<Option<(String, T)>>;
    async fn delete(&self, queue: &str, id: String) -> anyhow::Result<()>;
    async fn ok(&self, queue: &str, id: String) -> anyhow::Result<()>;
    async fn fail(&self, queue: &str, id: String) -> anyhow::Result<()>;
    async fn count(&self, queue: &str) -> anyhow::Result<usize>;
    async fn clear(&self, queue: &str) -> anyhow::Result<()>;
    async fn clear_all(&self) -> anyhow::Result<()>;
    async fn init(&self) -> anyhow::Result<()>;
}
